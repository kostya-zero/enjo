use std::{fs, path::Path};

pub struct ProjectsContainer {
    pub root: String,
    pub projects: Vec<Project>,
}

pub struct Project {
    name: String,
    path: String,
}

impl Project {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_path(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    pub fn set_path(&mut self, new_path: &str) {
        self.path = new_path.to_string();
    }
}

pub enum ProjectsError {
    RootNotFound,
    DirReadFailed,
}

impl ProjectsContainer {
    pub fn new(root_path: &str) -> Result<Self, ProjectsError> {
        let path = Path::new(&root_path);
        if !path.exists() {
            return Err(ProjectsError::RootNotFound);
        }

        let mut new_vec: ProjectsContainer = ProjectsContainer {
            root: root_path.to_string(),
            projects: Vec::new(),
        };

        if let Ok(entries) = fs::read_dir(root_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if entry.metadata().map(|m| m.is_dir()).unwrap_or(false)
                        && !name.starts_with('.')
                    {
                        let root = path.join(name);
                        let project_path = root.to_str().unwrap();
                        let project = Project::new(name, project_path);
                        new_vec.put(project);
                    }
                }
            }
        } else {
            return Err(ProjectsError::DirReadFailed);
        }

        Ok(new_vec)
    }

    pub fn put(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn contains(&self, name: &str) -> bool {
        self.projects.iter().any(|i| i.name == name)
    }
}
