use std::process::{Command, Stdio};

#[derive(Default)]
pub struct Proc {
    prog: String,
    args: Vec<String>,
    cwd: String,
}

pub enum ProcError {
    ExecutableNotFound,
    Interrupted,
    Other(String),
}

impl Proc {
    pub fn new(program: &str) -> Self {
        Self {
            prog: program.to_string(),
            ..Default::default()
        }
    }

    pub fn set_args(&mut self, new_args: Vec<&str>) {
        let converted = new_args.into_iter().map(|i| i.to_string()).collect();
        self.args = converted;
    }

    pub fn set_cwd(&mut self, new_cwd: &str) {
        self.cwd = new_cwd.to_string();
    }

    pub fn run(&self) -> Result<(), ProcError> {
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

        match cmd.output() {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(ProcError::ExecutableNotFound),
                std::io::ErrorKind::Interrupted => Err(ProcError::Interrupted),
                _ => Err(ProcError::Other(e.kind().to_string())),
            },
        }
    }
}
