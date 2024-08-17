use crate::library::Library;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_library_new() {
    let temp_dir = tempdir().unwrap();
    let library = Library::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    temp_dir.close().unwrap();
    assert!(library.is_empty());
}

#[test]
fn test_library_create() {
    let temp_dir = tempdir().unwrap();
    let library = Library::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    assert!(library.create("test_project").is_ok());
    temp_dir.close().unwrap();
}

#[test]
fn test_library_get() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("test_project");
    fs::create_dir(project_dir).unwrap();
    let library = Library::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    temp_dir.close().unwrap();
    let project = library.get("test_project");
    assert!(project.is_some());
    assert_eq!(project.unwrap().get_name(), "test_project");
}

#[test]
fn test_library_contains() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("test_project");
    fs::create_dir(project_dir).unwrap();
    let library = Library::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    temp_dir.close().unwrap();
    assert!(library.contains("test_project"));
}

#[test]
fn test_library_is_empty() {
    let temp_dir = tempdir().unwrap();
    let library = Library::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    assert!(library.is_empty());
    temp_dir.close().unwrap();
}