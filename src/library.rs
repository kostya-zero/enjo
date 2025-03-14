use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::{program::Program, constants::SYSTEM_DIRECTORIES, errors::LibraryError};

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

    pub fn get_path_str(&self) -> &str {
        self.path.to_str().unwrap_or_default()
    }

    pub fn is_empty(&self) -> Result<bool, LibraryError> {
        let entries = fs::read_dir(&self.path).map_err(|e| LibraryError::IoError(e.to_string()))?;
        Ok(entries.count() == 0)
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
        match Program::launch_program("git", args, cwd, false) {
            Ok(_) => Ok(()),
            Err(_) => Err(LibraryError::CloneFailed),
        }
    }

    pub fn create(&self, name: &str) -> Result<(), LibraryError> {
        if name.is_empty() {
            return Err(LibraryError::EmptyArgument);
        }

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
        if name.is_empty() {
            return Err(LibraryError::EmptyArgument);
        }
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
