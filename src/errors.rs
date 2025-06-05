use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to write configuration file.")]
    WriteFailed,

    #[error("Failed to format configuration to TOML.")]
    FormatFailed,

    #[error("Cannot find configuration file.")]
    FileNotFound,

    #[error("Configuration file has a bad structure and cannot be deserialized.")]
    BadStructure,
}

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

    #[error("Failed to launch Git.")]
    GitNotFound,

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

#[derive(Debug, Error, Deserialize)]
pub enum TemplatesError {
    #[error("Template with the same name already exists.")]
    AlreadyExists,

    #[error("Template not found.")]
    TemplateNotFound,

    #[error("File system error occurred.")]
    FileSystemError,

    #[error("Failed to serialize templates data.")]
    SerializationError,

    #[error("Failed to deserialize templates data.")]
    DeserializationError,

    #[error("Commands in the template are empty.")]
    CommandsAreEmpty,
}
