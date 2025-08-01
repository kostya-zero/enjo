use anyhow::Result;
use std::{
    io::ErrorKind,
    process::{Command, Stdio},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("Failed to launch program: {0}")]
    ProgramNotFound(String),

    #[error("Process was interrupted")]
    ProcessInterrupted,

    #[error("No permission to execute the program")]
    NoPermission,

    #[error("Program exited with non-zero status: {0}")]
    NonZeroExitCode(i32),

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

    if fork_mode {
        // In fork mode, we just spawn and don't wait for completion
        let result = cmd.spawn();
        if let Err(e) = result {
            return match e.kind() {
                ErrorKind::NotFound => Err(ProgramError::ProgramNotFound(program.to_string())),
                ErrorKind::PermissionDenied => Err(ProgramError::NoPermission),
                ErrorKind::Interrupted => Err(ProgramError::ProcessInterrupted),
                _ => Err(ProgramError::UnexpectedError(e.to_string())),
            };
        }
    } else {
        // In blocking mode, we wait for completion and check exit status
        let result = cmd.status();
        match result {
            Ok(status) => {
                if !status.success() {
                    // Handle non-zero exit codes
                    if let Some(code) = status.code() {
                        return Err(ProgramError::NonZeroExitCode(code));
                    } else {
                        // Process was terminated by signal (Unix only)
                        return Err(ProgramError::ProcessInterrupted);
                    }
                }
            }
            Err(e) => {
                return match e.kind() {
                    ErrorKind::NotFound => Err(ProgramError::ProgramNotFound(program.to_string())),
                    ErrorKind::PermissionDenied => Err(ProgramError::NoPermission),
                    ErrorKind::Interrupted => Err(ProgramError::ProcessInterrupted),
                    _ => Err(ProgramError::UnexpectedError(e.to_string())),
                };
            }
        }
    }

    Ok(())
}
