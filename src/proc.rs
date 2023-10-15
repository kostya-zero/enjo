use std::process::{Command, Stdio};

#[derive(Default)]
pub struct Proc {
    prog: String,
    args: Vec<String>,
    cwd: String,
}

impl Proc {
    pub fn new(program: &str) -> Self {
        let mut new_proc = Proc::default();
        new_proc.set_prog(program);
        new_proc
    }

    pub fn set_args(&mut self, new_args: Vec<&str>) {
        let converted = new_args.into_iter().map(|i| i.to_string()).collect();
        self.args = converted;
    }

    pub fn set_prog(&mut self, new_prog: &str) {
        self.prog.clear();
        self.prog.push_str(new_prog);
    }

    pub fn set_cwd(&mut self, new_cwd: &str) {
        self.cwd = new_cwd.to_string();
    }

    pub fn run(&self) {
        let mut cmd = Command::new(self.prog.clone());
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let converted_args: Vec<&str> = self.args.iter().map(|i| i.as_str()).collect();
        cmd.args(converted_args);
        if !self.cwd.is_empty() {
            cmd.current_dir(self.cwd.as_str());
        }
        cmd.output().expect("Failed to launch program.");
    }
}
