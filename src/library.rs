use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use thiserror::Error;
use crate::program::launch_program;

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("Project with the same name already exists.")]
    AlreadyExists,

    #[error("Project not found.")]
    ProjectNotFound,

    #[error("Invalid path to the projects directory.")]
    InvalidPath,

    #[error("File system error occurred.")]
    FileSystemError,

    #[error("Failed to clone repository.")]
    CloneFailed,

    #[error("Failed to rename.")]
    FailedToRename,

    #[error("A project with the same name already exists.")]
    ProjectExists,

    #[error("This name of the project is not allowed.")]
    InvalidProjectName,

    #[error("An unexpected I/O error occurred: {0}.")]
    IoError(String),
}

const SYSTEM_DIRECTORIES: [&str; 6] = [
    ".",
    "..",
    "$RECYCLE.BIN",
    "System Volume Information",
    "msdownld.tmp",
    ".Trash-1000",
];

#[derive(Debug, Clone, Default)]
pub struct CloneOptions {
    pub remote: String,
    pub branch: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct Project {
    name: Cow<'static, str>,
    path: PathBuf,
}

impl Project {
    pub fn new(new_name: &str, new_path: PathBuf) -> Self {
        Self {
            name: Cow::Owned(new_name.to_string()),
            path: new_path,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        self.path.to_str().unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        if let Ok(entries) = fs::read_dir(&self.path) {
            entries.count() == 0
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Library {
    projects: Vec<Project>,
    base_path: PathBuf,
}

impl Library {
    pub fn new(path: &str, display_hidden: bool) -> Result<Self, LibraryError> {
        let base_path = PathBuf::from(path);
        if !base_path.exists() || !base_path.is_dir() {
            return Err(LibraryError::InvalidPath);
        }
        let projects = Self::collect_projects(path, display_hidden)?;
        Ok(Self {
            projects,
            base_path,
        })
    }

    pub fn collect_projects(
        path: &str,
        display_hidden: bool,
    ) -> Result<Vec<Project>, LibraryError> {
        let dir_entries = fs::read_dir(path).map_err(|e| LibraryError::IoError(e.to_string()))?;

        // Pre-allocate with estimated capacity
        let mut projects = Vec::with_capacity(10);

        for entry in dir_entries {
            let entry = entry.map_err(|e| LibraryError::IoError(e.to_string()))?;
            let name = entry.file_name().to_string_lossy().into_owned();

            if Self::is_valid_project(&entry, &name, display_hidden) {
                projects.push(Project::new(&name, entry.path()));
            }
        }

        Ok(projects)
    }

    fn is_valid_project(entry: &fs::DirEntry, name: &str, display_hidden: bool) -> bool {
        if !display_hidden && name.starts_with('.') {
            return false;
        }

        entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
            && !SYSTEM_DIRECTORIES.contains(&name)
    }

    pub fn clone(&self, options: &CloneOptions) -> Result<(), LibraryError> {
        let mut args = vec!["clone".to_string(), options.remote.clone()];

        if let Some(name) = &options.name {
            args.push(name.to_owned());
        }

        if let Some(branch) = &options.branch {
            args.push("-b".to_string());
            args.push(branch.to_owned());
        }

        let cwd = self.base_path.to_str().unwrap();
        match launch_program("git", &args, Some(cwd), false) {
            Ok(_) => Ok(()),
            Err(_) => Err(LibraryError::CloneFailed),
        }
    }

    pub fn create(&self, name: &str) -> Result<(), LibraryError> {
        if self.base_path.join(name).exists() {
            return Err(LibraryError::AlreadyExists);
        }

        match fs::create_dir(self.base_path.join(name)) {
            Ok(_) => Ok(()),
            Err(_) => Err(LibraryError::FileSystemError),
        }
    }

    pub fn delete(&self, name: &str) -> Result<(), LibraryError> {
        match fs::remove_dir_all(self.base_path.join(name)) {
            Ok(_) => Ok(()),
            Err(_) => Err(LibraryError::FileSystemError),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.projects.iter().any(|x| x.name == *name)
    }

    pub fn get_vec(&self) -> &[Project] {
        &self.projects
    }

    pub fn get_names(&self) -> Vec<&str> {
        self.projects.iter().map(|p| p.get_name()).collect()
    }

    pub fn get(&self, name: &str) -> Result<&Project, LibraryError> {
        self.projects
            .iter()
            .find(|x| x.name == name)
            .ok_or(LibraryError::ProjectNotFound)
    }

    pub fn is_empty(&self) -> bool {
        self.projects.is_empty()
    }

    pub fn rename(&self, old_name: &str, new_name: &str) -> Result<(), LibraryError> {
        if !self.contains(old_name) {
            return Err(LibraryError::ProjectNotFound);
        }

        if self.contains(new_name) {
            return Err(LibraryError::ProjectExists);
        }

        if SYSTEM_DIRECTORIES.contains(&new_name) {
            return Err(LibraryError::InvalidProjectName);
        }

        let old_path = Path::new(&self.base_path).join(old_name);
        let new_path = Path::new(&self.base_path).join(new_name);

        fs::rename(old_path, new_path).map_err(|_| LibraryError::FailedToRename)?;

        Ok(())
    }
}
