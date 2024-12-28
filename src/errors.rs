use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to write configuration file.")]
    WriteFailed,

    #[error("Failed to format configuration to TOML.")]
    FormatFailed,

    #[error("Cannot load the configuration file because it does not exist on the file system.")]
    FileNotFound,

    #[error("Configuration file has a bad structure and cannot be deserialized.")]
    BadStructure,
}

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("A directory with projects does not exist on the file system.")]
    DirectoryNotFound,

    #[error("Failed to read the contents of the directory.")]
    ReadFailed,

    #[error("Project with the same name already exists.")]
    AlreadyExists,

    #[error("File system error occured.")]
    FileSystemError,

    #[error("Failed to clone repository.")]
    CloneFailed(ProgramError),
}

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("Failed to launch program because '{0}' was not found.")]
    ExecutableNotFound(String),

    #[error("Program was interrupted.")]
    Interrupted,

    #[error("Program failed to launch or failed: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Template with the same name already exists.")]
    AlreadyExists,

    #[error("Template not found.")]
    TemplateNotFound,

    #[error("File system error occured while working with the file.")]
    FileSystemError,
}
