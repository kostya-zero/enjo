use anyhow::{Result, anyhow};
use std::{
    path::Path,
    process::{Command, Stdio},
};

pub struct Program;

impl Program {
    pub fn launch_program(
        program: &str,
        args: Vec<String>,
        cwd: &str,
        fork_mode: bool,
    ) -> Result<()> {
        let mut cmd = Command::new(program);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let converted_args: Vec<&str> = args.iter().map(|i| i.as_str()).collect();
        cmd.args(converted_args);
        if !cwd.is_empty() {
            cmd.current_dir(cwd);
        }

        #[cfg(windows)]
        ctrlc::set_handler(|| {}).unwrap();

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

    pub fn execute_command(program: &str, command: &str, cwd: &Path, quiet: bool) -> Result<()> {
        let mut cmd = Command::new(program);
        cmd.args(["-c", command]).current_dir(cwd);

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
