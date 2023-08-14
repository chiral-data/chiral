//! Environmental Variables
//! 

pub enum Variable {
    ChiralDataDir,
    GromacsWorkDir
}

impl Variable {
    pub fn as_str(&self) -> &str {
        match self {
            Variable::ChiralDataDir => "CHIRAL_DATA_DIR",
            Variable::GromacsWorkDir => "GROMACS_WORK_DIR",
        }
    }

    pub fn get(&self) -> String {
        match std::env::var_os(self.as_str()) {
            Some(s) => {
                s.to_str().unwrap().to_owned()
            }
            None => panic!("Environment variable {} not set", self.as_str())
        }
    }

    pub fn set(&self, v: &str) {
        std::env::set_var(self.as_str(), v);
    }
}