use std::fs::create_dir_all;
use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::path::Path;

pub struct Generator;

impl Generator {
    pub fn create_bin_project(path: &Path) -> Result<()> {
        Self::create_src_dir(path)?;
        Self::create_main_file(path)?;
        Self::create_yaml_file(path)?;
        Ok(())
    }

    fn create_src_dir(path: &Path) -> Result<()> {
        create_dir_all(path.join("src"))?;
        Ok(())
    }

    fn create_main_file(path: &Path) -> Result<()> {
        File::create(path.join("src").join("main.ne"))?.write_all(b"")?;
        Ok(())
    }

    fn create_yaml_file(path: &Path) -> Result<()> {
        File::create(path.join("Neon.yaml"))?.write_all(b"")?;
        Ok(())
    }
}
