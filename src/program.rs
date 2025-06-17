use anyhow::Result;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("Failed to launch program: {0}")]
    ProgramNotFound(String),

    #[error("Process was interrupted")]
    ProcessInterrupted,

    #[error("No permission to execute the program")]
    NoPermission,

    #[error("An unexpected error occurred: {0}")]
    UnexpectedError(String),
}

pub fn launch_program(
    program: &str,
    args: &Vec<String>,
    cwd: Option<&str>,
    fork_mode: bool,
    quiet: bool,
) -> Result<(), ProgramError> {
    let mut cmd = Command::new(program);

    if quiet {
        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
    } else {
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
    }

    cmd.args(args);
    if let Some(cwd_path) = cwd {
        cmd.current_dir(cwd_path);
    }

    // Required for Windows because if user runs program inside a shell and presses Ctrl+C,
    // user will lose control over the shell. I don't know why it happens, but it does.
    // On Linux and macOS Ctrl+C works as expected.
    #[cfg(windows)]
    let _ = ctrlc::set_handler(|| {});

    let result = if fork_mode {
        cmd.spawn().err()
    } else {
        cmd.status().err()
    };

    if let Some(e) = result {
        return match e.kind() {
            std::io::ErrorKind::NotFound => Err(ProgramError::ProgramNotFound(program.to_string())),
            std::io::ErrorKind::PermissionDenied => Err(ProgramError::NoPermission),
            std::io::ErrorKind::Interrupted => Err(ProgramError::ProcessInterrupted),
            _ => Err(ProgramError::UnexpectedError(e.to_string())),
        };
    }

    Ok(())
}
