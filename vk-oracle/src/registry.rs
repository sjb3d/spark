use vk_parse as vk;

pub trait CollectOne {
    type Item;
    fn collect_one(self) -> Option<Self::Item>;
}

impl<I: Iterator> CollectOne for I {
    type Item = I::Item;
    fn collect_one(mut self) -> Option<Self::Item> {
        self.next().filter(|_| self.next().is_none())
    }
}

pub trait IsVulkanApi {
    fn is_vulkan_api(&self) -> bool;
}

impl IsVulkanApi for &str {
    fn is_vulkan_api(&self) -> bool {
        self.split(',').any(|a| a == "vulkan")
    }
}

impl IsVulkanApi for vk::Extension {
    fn is_vulkan_api(&self) -> bool {
        self.supported.as_deref().map(|s| s.is_vulkan_api()).unwrap_or(false)
    }
}

impl IsVulkanApi for vk::ExtensionChild {
    fn is_vulkan_api(&self) -> bool {
        match self {
            vk::ExtensionChild::Require { api, .. } => api,
            vk::ExtensionChild::Remove { api, .. } => api,
            vk::ExtensionChild::Deprecate { api, .. } => api,
            _ => unimplemented!(),
        }
        .as_deref()
        .map(|s| s.is_vulkan_api())
        .unwrap_or(true)
    }
}

impl IsVulkanApi for vk::Feature {
    fn is_vulkan_api(&self) -> bool {
        self.api.as_str().is_vulkan_api()
    }
}

impl IsVulkanApi for vk::CommandDefinition {
    fn is_vulkan_api(&self) -> bool {
        self.api.as_deref().map(|s| s.is_vulkan_api()).unwrap_or(true)
    }
}

impl IsVulkanApi for vk::Type {
    fn is_vulkan_api(&self) -> bool {
        self.api.as_deref().map(|s| s.is_vulkan_api()).unwrap_or(true)
    }
}

impl IsVulkanApi for vk::Enum {
    fn is_vulkan_api(&self) -> bool {
        self.api.as_deref().map(|s| s.is_vulkan_api()).unwrap_or(true)
    }
}

impl IsVulkanApi for vk::TypeMemberDefinition {
    fn is_vulkan_api(&self) -> bool {
        self.api.as_deref().map(|s| s.is_vulkan_api()).unwrap_or(true)
    }
}

impl IsVulkanApi for vk::CommandParam {
    fn is_vulkan_api(&self) -> bool {
        self.api.as_deref().map(|s| s.is_vulkan_api()).unwrap_or(true)
    }
}

pub trait IsBlacklisted {
    fn is_blacklisted(&self) -> bool;
}

impl IsBlacklisted for vk::Extension {
    fn is_blacklisted(&self) -> bool {
        matches!(self.author.as_deref(), Some("GGP") | Some("QNX"))
            || self.name.contains("KHR_video")
            || self.depends.as_deref().map_or(false, |s| s.contains("KHR_video"))
            || self.name.contains("EXT_video")
            || self.supported.as_deref() == Some("disabled")
    }
}

pub trait HasSpecName {
    fn spec_name(&self) -> &str;
}

impl HasSpecName for vk::Tag {
    fn spec_name(&self) -> &str {
        self.name.as_str()
    }
}

impl HasSpecName for vk::Type {
    fn spec_name(&self) -> &str {
        self.name.as_deref().unwrap_or_else(|| match &self.spec {
            vk::TypeSpec::Funcpointer(code) => &code.proto.name.as_str(),
            vk::TypeSpec::Code(code) => code
                .markup
                .iter()
                .filter_map(|markup| match markup {
                    vk::TypeCodeMarkup::Name(name) => Some(name.as_str()),
                    _ => None,
                })
                .collect_one()
                .expect("failed to find name in code markup"),
            _ => unimplemented!(),
        })
    }
}

impl HasSpecName for vk::Extension {
    fn spec_name(&self) -> &str {
        self.name.as_str()
    }
}

impl HasSpecName for vk::Command {
    fn spec_name(&self) -> &str {
        match self {
            vk::Command::Definition(cmd_def) => cmd_def.proto.name.as_str(),
            vk::Command::Alias { name, .. } => name.as_str(),
            _ => unimplemented!(),
        }
    }
}

impl HasSpecName for vk::Enum {
    fn spec_name(&self) -> &str {
        self.name.as_str()
    }
}

pub trait AliasOf<'a> {
    fn alias_of(&self) -> Option<&'a str>;
}

impl<'a> AliasOf<'a> for &'a vk::Command {
    fn alias_of(&self) -> Option<&'a str> {
        match self {
            vk::Command::Alias { alias, .. } => Some(alias.as_str()),
            _ => None,
        }
    }
}

impl<'a> AliasOf<'a> for &'a vk::Enum {
    fn alias_of(&self) -> Option<&'a str> {
        match &self.spec {
            vk::EnumSpec::Alias { alias, .. } => Some(alias.as_str()),
            _ => None,
        }
    }
}

pub trait RegistryIter {
    fn iter_tags(&self) -> impl Iterator<Item = &vk::Tag>;
    fn iter_types(&self) -> impl Iterator<Item = &vk::Type>;
    fn iter_enums(&self) -> impl Iterator<Item = &vk::Enums>;
    fn iter_extensions(&self) -> impl Iterator<Item = &vk::Extension>;
    fn iter_features(&self) -> impl Iterator<Item = &vk::Feature>;
    fn iter_commands(&self) -> impl Iterator<Item = &vk::Command>;
}

impl RegistryIter for vk::Registry {
    fn iter_tags(&self) -> impl Iterator<Item = &vk::Tag> {
        self.0
            .iter()
            .filter_map(|child| match child {
                vk::RegistryChild::Tags(tags) => Some(tags),
                _ => None,
            })
            .flat_map(|tags| tags.children.iter())
    }

    fn iter_types(&self) -> impl Iterator<Item = &vk::Type> {
        self.0
            .iter()
            .filter_map(|child| match child {
                vk::RegistryChild::Types(types) => Some(types),
                _ => None,
            })
            .flat_map(|types| types.children.iter())
            .filter_map(|child| match child {
                vk::TypesChild::Type(ty) => Some(ty),
                _ => None,
            })
            .filter(|ty| ty.is_vulkan_api())
    }

    fn iter_enums(&self) -> impl Iterator<Item = &vk::Enums> {
        self.0.iter().filter_map(|child| match child {
            vk::RegistryChild::Enums(enums) => Some(enums),
            _ => None,
        })
    }

    fn iter_extensions(&self) -> impl Iterator<Item = &vk::Extension> {
        self.0
            .iter()
            .filter_map(|child| match child {
                vk::RegistryChild::Extensions(extensions) => Some(extensions),
                _ => None,
            })
            .flat_map(|extensions| extensions.children.iter())
            .filter(|ext| ext.is_vulkan_api() && !ext.is_blacklisted())
    }

    fn iter_features(&self) -> impl Iterator<Item = &vk::Feature> {
        self.0
            .iter()
            .filter_map(|child| match child {
                vk::RegistryChild::Feature(feature) => Some(feature),
                _ => None,
            })
            .filter(|feature| feature.is_vulkan_api())
    }

    fn iter_commands(&self) -> impl Iterator<Item = &vk::Command> {
        self.0
            .iter()
            .filter_map(|child| match child {
                vk::RegistryChild::Commands(commands) => Some(commands),
                _ => None,
            })
            .flat_map(|commands| commands.children.iter())
            .filter(|cmd| match cmd {
                vk::Command::Definition(cmd_def) => cmd_def.is_vulkan_api(),
                _ => true,
            })
    }
}
