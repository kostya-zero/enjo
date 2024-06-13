use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

pub enum ContainerError {
    DirectoryNotFound,
    ReadFailed,
}

impl Project {
    pub fn new(new_name: &str, new_path: PathBuf) -> Self {
        Self {
            name: String::from(new_name),
            path: new_path,
        }
    }

    pub fn get_path_str(&self) -> String {
        let path_str = self.path.to_str().unwrap();
        String::from(path_str)
    }
}

#[derive(Debug, Clone)]
pub struct Container(Vec<Project>);
impl Container {
    pub fn new(path: &str, display_hidden: bool) -> Result<Self, ContainerError> {
        if !Path::new(path).exists() {
            return Err(ContainerError::DirectoryNotFound);
        }

        let projects = Self::collect_projects(path, display_hidden)?;
        Ok(Self(projects))
    }

    fn collect_projects(path: &str, display_hidden: bool) -> Result<Vec<Project>, ContainerError> {
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
            return Err(ContainerError::ReadFailed);
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

    pub fn contains(&self, name: &str) -> bool {
        self.0.iter().any(|x| x.name == *name)
    }

    pub fn get_vec(&self) -> Vec<Project> {
        self.0.clone()
    }

    pub fn get(&self, name: &str) -> Option<Project> {
        self.0.clone().into_iter().find(|x| x.name == *name)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
