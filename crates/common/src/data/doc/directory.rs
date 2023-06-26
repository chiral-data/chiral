//! Directory based Document
//!     a directory with files included is treated as a documens.
//!     for applicaitons like gromacs simulation
//! 

struct DocDirectory {
    name: String
}

impl DocDirectory {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn create(&self, home_dir: &str) -> std::io::Result<()> {
        let dir_path = std::path::PathBuf::from(home_dir).join(&self.name);
        std::fs::create_dir(dir_path)?;
        Ok(())
    }
}

struct DataStore {
    home_dir: String
}