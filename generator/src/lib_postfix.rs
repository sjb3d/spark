

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_instance_version() {
        let loader = Loader::new().unwrap();
        let v = unsafe { loader.enumerate_instance_version() };
        assert!(v.is_ok());
    }
}
