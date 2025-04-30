use std::fs;

use library::Project;

use super::*;

#[test]
fn test_project_new() {
    let context = TestContext::setup();
    let path = context.path().to_path_buf();
    let project = Project::new("test_project", path.clone());

    assert_eq!(project.get_name(), "test_project");
    assert_eq!(project.get_path_str(), path.to_str().unwrap());
}

#[test]
fn test_project_get_name() {
    let context = TestContext::setup();
    let project = Project::new("test_project", context.path().to_path_buf());
    assert_eq!(project.get_name(), "test_project");
}

#[test]
fn test_project_get_path_str() {
    let context = TestContext::setup();
    let project = Project::new("test_project", context.path().to_path_buf());
    assert_eq!(project.get_path_str(), context.path_str());
}

#[test]
fn test_project_is_empty() {
    let context = TestContext::setup();
    let project = Project::new("test_project", context.path().to_path_buf());
    assert!(project.is_empty().unwrap());

    fs::write(context.path().join("test.txt"), "test").unwrap();
    assert!(!project.is_empty().unwrap());

    assert!(context.path().exists());
}
