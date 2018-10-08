

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_instance_version() {
        let entry = Entry::new().unwrap();
        let v = unsafe { entry.enumerate_instance_version() };
        assert!(v.is_ok());
    }
}
