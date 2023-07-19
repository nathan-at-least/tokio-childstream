use crate::ApplicationPaths;

#[test]
fn directory_creation_is_idempotent() {
    let paths = ApplicationPaths::from("application-paths-test");
    assert!(paths.config_dir().unwrap().is_dir());
    assert!(paths.config_dir().unwrap().is_dir());
}
