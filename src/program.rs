use anyhow::{Result, anyhow};
use std::{
    path::Path,
    process::{Command, Stdio},
};

pub struct Program;

impl Program {
    pub fn launch_program(
        program: &str,
        args: &Vec<String>,
        cwd: Option<&str>,
        fork_mode: bool,
    ) -> Result<()> {
        let mut cmd = Command::new(program);
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        cmd.args(args);
        if let Some(cwd_path) = cwd {
            cmd.current_dir(cwd_path);
        }

        #[cfg(windows)]
        ctrlc::set_handler(|| {})?;

        if fork_mode {
            match cmd.spawn() {
                Ok(_) => Ok(()),
                Err(e) => Err(anyhow!(e.to_string())),
            }
        } else {
            match cmd.output() {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        Err(anyhow!("Executable not found: {}", program))
                    }
                    std::io::ErrorKind::Interrupted => Err(anyhow!("Process interrupted")),
                    _ => Err(anyhow!(e.kind().to_string())),
                },
            }
        }
    }

    pub fn execute_command(program: &str, args: &Vec<String>, cwd: &Path, quiet: bool) -> Result<()> {
        let mut cmd = Command::new(program);
        cmd.args(args).current_dir(cwd);

        if !quiet {
            cmd.stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        match cmd.output() {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}
