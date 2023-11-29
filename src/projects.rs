use std::{fs, path::Path};

pub struct ProjectsContainer {
    pub root: String,
    pub projects: Vec<String>,
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

        let mut new_vec: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(root_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if entry.metadata().map(|m| m.is_dir()).unwrap_or(false)
                        && !name.starts_with('.')
                    {
                        new_vec.push(name.to_owned());
                    }
                }
            }
        } else {
            return Err(ProjectsError::DirReadFailed);
        }

        Ok(ProjectsContainer {
            root: root_path.to_string(),
            projects: new_vec,
        })
    }

    pub fn contains(&self, name: &str) -> bool {
        self.projects.contains(&name.to_string())
    }
}
