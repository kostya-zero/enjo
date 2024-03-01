use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
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
    pub fn new(path: &str) -> Self {
        let mut projects: Vec<Project> = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) && (name != "." || name != "..") {
                        let project: Project = Project::new(name, Path::new(path).join(name));
                        projects.push(project);
                    }
                }
            }
        }
        Self(projects)
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
