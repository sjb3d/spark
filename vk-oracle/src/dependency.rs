use crate::oracle::*;

#[derive(Debug, Clone)]
pub enum DependencyExpr<T> {
    Never,
    Always,
    Version(Version),
    Extension(T),
    Feature,
    And(Vec<DependencyExpr<T>>),
    Or(Vec<DependencyExpr<T>>),
}

impl<T> DependencyExpr<T>
where
    T: Clone,
{
    fn map_extensions_impl<U, F>(&self, f: &mut F) -> DependencyExpr<U>
    where
        F: FnMut(T) -> U,
    {
        match self {
            Self::Never => DependencyExpr::Never,
            Self::Always => DependencyExpr::Always,
            Self::Version(v) => DependencyExpr::Version(*v),
            Self::Extension(t) => DependencyExpr::Extension(f(t.clone())),
            Self::Feature => DependencyExpr::Feature,
            Self::And(v) => DependencyExpr::And(v.iter().map(|e| e.map_extensions_impl(f)).collect()),
            Self::Or(v) => DependencyExpr::Or(v.iter().map(|e| e.map_extensions_impl(f)).collect()),
        }
    }

    pub fn map_extensions<U, F>(&self, mut f: F) -> DependencyExpr<U>
    where
        F: FnMut(T) -> U,
    {
        self.map_extensions_impl(&mut f)
    }
}

impl<T> DependencyExpr<T> {
    fn visit_extensions_impl(&self, f: &mut impl FnMut(&T)) {
        match self {
            Self::Never | Self::Always | Self::Version(_) | Self::Feature => {}
            Self::Extension(t) => f(t),
            Self::And(v) | Self::Or(v) => {
                for e in v {
                    e.visit_extensions_impl(f);
                }
            }
        }
    }

    pub fn visit_extensions(&self, mut f: impl FnMut(&T)) {
        self.visit_extensions_impl(&mut f);
    }

    fn apply_at_leaves_impl(&mut self, f: &mut impl FnMut(&mut Self)) {
        match self {
            Self::Never | Self::Always | Self::Version(_) | Self::Extension(_) | Self::Feature => f(self),
            Self::And(v) | Self::Or(v) => {
                for dep in v.iter_mut() {
                    dep.apply_at_leaves_impl(f)
                }
            }
        }
    }

    pub fn apply_at_leaves(&mut self, mut f: impl FnMut(&mut Self)) {
        self.apply_at_leaves_impl(&mut f)
    }

    pub fn is_always(&self) -> bool {
        matches!(self, Self::Always)
    }

    pub fn max_involved_version(&self) -> Version {
        match self {
            Self::Never | Self::Always | Self::Extension(_) | Self::Feature => Version::new(0, 0),
            Self::Version(v) => *v,
            Self::And(v) | Self::Or(v) => v
                .iter()
                .map(|dep| dep.max_involved_version())
                .max()
                .unwrap_or(Version::new(0, 0)),
        }
    }
}

impl<T> DependencyExpr<T>
where
    T: Clone + PartialEq + Ord,
{
    fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Never, Self::Never) | (Self::Always, Self::Always) => true,
            (Self::Feature, Self::Feature) => false, // TODO: store string/hash to match here?
            (Self::Version(a), Self::Version(b)) => a == b,
            (Self::Extension(a), Self::Extension(b)) => a == b,
            (Self::And(ref a), Self::And(ref b)) | (Self::Or(ref a), Self::Or(ref b)) => match (a.len(), b.len()) {
                (0, 0) => true,
                (1, 1) => a[0].matches(&b[0]),
                (2, 2) => (a[0].matches(&b[0]) && a[1].matches(&b[1])) || (a[0].matches(&b[1]) && a[1].matches(&b[0])),
                _ => false,
            },
            _ => false,
        }
    }

    // only true when "other" is true
    fn is_subset_of(&self, other: &Self) -> bool {
        self.matches(other)
            || match (self, other) {
                (Self::Never, _) => true,
                (_, Self::Always) => true,
                (Self::Version(a), Self::Version(b)) => a >= b,
                (Self::And(ref a), _) => a.iter().any(|a_elem| a_elem.is_subset_of(other)),
                (Self::Or(ref a), _) => a.iter().all(|a_elem| a_elem.is_subset_of(other)),
                _ => false,
            }
    }

    fn try_merge_and(a: &Self, b: &Self) -> Option<Self> {
        if a.is_subset_of(b) {
            Some(a.clone())
        } else if b.is_subset_of(a) {
            Some(b.clone())
        } else {
            None
        }
    }

    fn try_merge_or(a: &Self, b: &Self) -> Option<Self> {
        if b.is_subset_of(a) {
            Some(a.clone())
        } else if a.is_subset_of(b) {
            Some(b.clone())
        } else {
            None
        }
    }

    pub fn simplify(&mut self) {
        match self {
            Self::Never | Self::Always => {}
            Self::Version(v) => {
                if *v == Version::new(1, 0) {
                    *self = Self::Always;
                }
            }
            Self::Feature | Self::Extension(_) => {}
            Self::And(v) => {
                let mut tmp = Vec::new();
                for mut dep in v.drain(..) {
                    dep.simplify();
                    if let Self::And(mut inner) = dep {
                        tmp.append(&mut inner);
                    } else {
                        tmp.push(dep);
                    }
                }
                let mut result = Vec::new();
                'outer: for dep in tmp.drain(..) {
                    for prev in &mut result {
                        if let Some(merged) = Self::try_merge_and(prev, &dep) {
                            *prev = merged;
                            continue 'outer;
                        }
                    }
                    result.push(dep);
                }
                match result.len() {
                    0 => unreachable!(),
                    1 => *self = result.pop().unwrap(),
                    _ => *v = result,
                }
            }
            Self::Or(v) => {
                let mut tmp = Vec::new();
                for mut dep in v.drain(..) {
                    dep.simplify();
                    if let Self::Or(mut inner) = dep {
                        tmp.append(&mut inner);
                    } else {
                        tmp.push(dep);
                    }
                }
                let mut result = Vec::new();
                'outer: for dep in tmp.drain(..) {
                    for prev in &mut result {
                        if let Some(merged) = Self::try_merge_or(prev, &dep) {
                            *prev = merged;
                            continue 'outer;
                        }
                    }
                    result.push(dep);
                }
                result.sort_by(|a, b| a.max_involved_version().cmp(&b.max_involved_version()).reverse());
                match result.len() {
                    0 => unreachable!(),
                    1 => *self = result.pop().unwrap(),
                    _ => *v = result,
                }
            }
        }
    }
}
