mod test_autocomplete;
mod test_library;
mod test_project;

use std::path::Path;

use super::*;
use tempfile::TempDir;

struct TestContext {
    temp_dir: TempDir,
}

impl TestContext {
    fn setup() -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        Self { temp_dir }
    }

    fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    fn path_str(&self) -> &str {
        self.temp_dir.path().to_str().unwrap()
    }
}
