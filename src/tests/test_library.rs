use crate::{
    errors::LibraryError,
    library::{CloneOptions, Library},
    tests::TestContext,
};
use std::fs;

#[test]
fn test_library_new() {
    let context = TestContext::setup();

    let library = Library::new(context.path_str(), false).unwrap();
    assert!(library.is_empty());

    assert!(context.path().exists());

    assert!(matches!(
        Library::new("/non/existent/path", false),
        Err(LibraryError::IoError(_))
    ));

    assert!(context.path().exists());
}

#[test]
fn test_library_create_project() {
    let context = TestContext::setup();
    let library = Library::new(context.path_str(), false).unwrap();

    assert!(library.create("new_project").is_ok());
    assert!(context.path().join("new_project").exists());

    assert!(matches!(
        library.create("new_project"),
        Err(LibraryError::AlreadyExists)
    ));
}

#[test]
fn test_library_contains() {
    let context = TestContext::setup();

    fs::create_dir(context.path().join("test_project")).unwrap();

    let library = Library::new(context.path_str(), false).unwrap();
    assert!(library.contains("test_project"));
    assert!(!library.contains("non_existent_project"));
}

#[test]
fn test_library_get() {
    let context = TestContext::setup();

    fs::create_dir(context.path().join("test_project")).unwrap();

    let library = Library::new(context.path_str(), false).unwrap();
    assert!(library.get("test_project").is_ok());
    assert!(library.get("non_existent_project").is_err());
}

#[test]
fn test_hidden_projects() {
    let context = TestContext::setup();

    fs::create_dir(context.path().join("visible_project")).unwrap();
    fs::create_dir(context.path().join(".hidden_project")).unwrap();

    let library = Library::new(context.path_str(), false).unwrap();
    assert!(library.contains("visible_project"));
    assert!(!library.contains(".hidden_project"));

    let library_with_hidden = Library::new(context.path_str(), true).unwrap();
    assert!(library_with_hidden.contains("visible_project"));
    assert!(library_with_hidden.contains(".hidden_project"));
}

#[test]
fn test_cleanup() {
    let temp_path;
    {
        let context = TestContext::setup();
        temp_path = context.path().to_path_buf();

        fs::write(context.path().join("test.txt"), "test").unwrap();

        assert!(temp_path.exists());
    }

    assert!(!temp_path.exists());
}

#[test]
fn test_clone_options() {
    let options = CloneOptions {
        remote: String::from("https://github.com/user/repo.git"),
        branch: Some(String::from("main")),
        name: Some(String::from("my-repo")),
    };

    assert_eq!(options.remote, "https://github.com/user/repo.git");
    assert_eq!(options.branch, Some(String::from("main")));
    assert_eq!(options.name, Some(String::from("my-repo")));
}
