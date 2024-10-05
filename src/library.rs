use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{errors::LibraryError, program::Program};

#[derive(Debug, Clone, Default)]
pub struct CloneOptions {
    pub remote: String,
    pub branch: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Project {
    name: String,
    path: PathBuf,
}

impl Project {
    pub fn new(new_name: &str, new_path: PathBuf) -> Self {
        Self {
            name: String::from(new_name),
            path: new_path,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_path_str(&self) -> String {
        let path_str = self.path.to_str().unwrap();
        String::from(path_str)
    }

    pub fn is_empty(&self) -> bool {
        let mut entries = fs::read_dir(self.path.clone()).unwrap();
        entries.next().is_none()
    }
}

#[derive(Debug, Clone)]
pub struct Library {
    projects: Vec<Project>,
    path: PathBuf,
}
impl Library {
    pub fn new(path: &str, display_hidden: bool) -> Result<Self, LibraryError> {
        if !Path::new(path).exists() {
            return Err(LibraryError::DirectoryNotFound);
        }

        let projects = Self::collect_projects(path, display_hidden)?;
        Ok(Self {
            projects,
            path: PathBuf::from(path),
        })
    }

    fn collect_projects(path: &str, display_hidden: bool) -> Result<Vec<Project>, LibraryError> {
        let mut projects: Vec<Project> = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    let path = entry.path();
                    if Self::is_valid_project(&entry, name, display_hidden) {
                        let project = Project::new(name, path);
                        projects.push(project);
                    }
                }
            }
        } else {
            return Err(LibraryError::ReadFailed);
        }

        Ok(projects)
    }

    fn is_valid_project(entry: &fs::DirEntry, name: &str, display_hidden: bool) -> bool {
        let system_dirs = [
            ".",
            "..",
            "$RECYCLE.BIN",
            "System Volume Information",
            "msdownld.tmp",
            ".Trash-1000",
        ];
        let is_dir = entry.metadata().map(|m| m.is_dir()).unwrap_or(false);
        let is_hidden = name.starts_with('.');
        let is_system_dir = system_dirs.contains(&name);

        is_dir && (!is_hidden || display_hidden) && !is_system_dir
    }

    pub fn clone(&self, options: CloneOptions) -> Result<(), LibraryError> {
        let mut program = Program::new("git");
        let mut args: Vec<String> = vec!["clone".to_string(), options.remote.clone()];

        if let Some(name) = options.name.clone() {
            args.push(name);
        }

        if let Some(branch) = options.branch.clone() {
            args.push("-b".to_string());
            args.push(branch);
        }

        program.set_args(args);
        program.set_cwd(self.path.to_str().unwrap());
        match program.run() {
            Ok(_) => Ok(()),
            Err(e) => Err(LibraryError::CloneFailed(e)),
        }
    }

    pub fn create(&self, name: &str) -> Result<(), LibraryError> {
        if self.path.join(name).exists() {
            return Err(LibraryError::AlreadyExists);
        }

        match fs::create_dir(self.path.join(name)) {
            Ok(_) => Ok(()),
            Err(_) => Err(LibraryError::FileSystemError),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.projects.iter().any(|x| x.name == *name)
    }

    pub fn get_vec(&self) -> Vec<Project> {
        self.projects.clone()
    }

    pub fn get(&self, name: &str) -> Option<Project> {
        self.projects.clone().into_iter().find(|x| x.name == *name)
    }

    pub fn is_empty(&self) -> bool {
        self.projects.is_empty()
    }
}
