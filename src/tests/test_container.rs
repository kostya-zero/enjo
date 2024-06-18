use crate::container::{Container, ContainerError, Project};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_project_new() {
    let project = Project::new("test_project", PathBuf::from("/test/path"));
    assert_eq!(project.name, "test_project");
    assert_eq!(project.path, PathBuf::from("/test/path"));
}

#[test]
fn test_project_get_path_str() {
    let project = Project::new("test_project", PathBuf::from("/test/path"));
    assert_eq!(project.get_path_str(), "/test/path");
}

#[test]
fn test_container_new_directory_not_found() {
    let result = Container::new("/nonexistent/path", false);
    assert!(matches!(result, Err(ContainerError::DirectoryNotFound)));
}

#[test]
fn test_container_new_read_failed() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("not_a_directory");
    fs::File::create(&file_path).unwrap();

    let result = Container::new(file_path.to_str().unwrap(), false);
    temp_dir.close().unwrap();
    assert!(matches!(result, Err(ContainerError::ReadFailed)));
}

#[test]
fn test_container_new() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project1");
    fs::create_dir(project_dir).unwrap();

    let result = Container::new(temp_dir.path().to_str().unwrap(), false);
    assert!(result.is_ok());
    temp_dir.close().unwrap();

    let container = result.unwrap();
    assert!(!container.is_empty());
    assert!(container.contains("project1"));
}

#[test]
fn test_container_new_display_hidden() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project1");
    let hidden_dir = temp_dir.path().join(".hidden_project");
    fs::create_dir(project_dir).unwrap();
    fs::create_dir(hidden_dir).unwrap();

    let result = Container::new(temp_dir.path().to_str().unwrap(), false);
    assert!(result.is_ok());
    temp_dir.close().unwrap();

    let container = result.unwrap();
    assert!(!container.is_empty());
    assert!(!container.contains(".hidden_project"));
}

#[test]
fn test_container_contains() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project1");
    fs::create_dir(project_dir).unwrap();

    let container = Container::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    temp_dir.close().unwrap();
    assert!(container.contains("project1"));
    assert!(!container.contains("project2"));
}

#[test]
fn test_container_get_vec() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project1");
    fs::create_dir(project_dir).unwrap();

    let container = Container::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    let projects = container.get_vec();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "project1");
}

#[test]
fn test_container_get() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project1");
    fs::create_dir(project_dir).unwrap();

    let container = Container::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    temp_dir.close().unwrap();
    let project = container.get("project1");
    assert!(project.is_some());
    assert_eq!(project.unwrap().name, "project1");

    let project = container.get("project2");
    assert!(project.is_none());
}

#[test]
fn test_container_is_empty() {
    let temp_dir = tempdir().unwrap();

    let container = Container::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    assert!(container.is_empty());

    let project_dir = temp_dir.path().join("project1");
    fs::create_dir(project_dir).unwrap();

    let container = Container::new(temp_dir.path().to_str().unwrap(), false).unwrap();
    temp_dir.close().unwrap();
    assert!(!container.is_empty());
}
