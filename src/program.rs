use crate::errors::ProgramError;
use std::process::{Command, Stdio};

#[derive(Default)]
pub struct Program {
    prog: String,
    args: Vec<String>,
    cwd: String,
    fork_mode: bool,
}

impl Program {
    pub fn new(program: &str) -> Self {
        Self {
            prog: program.to_string(),
            ..Default::default()
        }
    }

    pub fn set_args(&mut self, new_args: Vec<String>) {
        self.args = new_args;
    }

    pub fn set_fork_mode(&mut self, fork_mode: bool) {
        self.fork_mode = fork_mode;
    }

    pub fn set_cwd(&mut self, new_cwd: &str) {
        self.cwd = new_cwd.to_string();
    }

    pub fn run(&self) -> Result<(), ProgramError> {
        let mut cmd = Command::new(self.prog.clone());
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let converted_args: Vec<&str> = self.args.iter().map(|i| i.as_str()).collect();
        cmd.args(converted_args);
        if !self.cwd.is_empty() {
            cmd.current_dir(self.cwd.as_str());
        }

        #[cfg(windows)]
        ctrlc::set_handler(|| {}).unwrap();

        if self.fork_mode {
            match cmd.spawn() {
                Ok(_) => Ok(()),
                Err(e) => Err(ProgramError::Other(e.to_string())),
            }
        } else {
            match cmd.output() {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        Err(ProgramError::ExecutableNotFound(self.prog.clone()))
                    }
                    std::io::ErrorKind::Interrupted => Err(ProgramError::Interrupted),
                    _ => Err(ProgramError::Other(e.kind().to_string())),
                },
            }
        }
    }
}
