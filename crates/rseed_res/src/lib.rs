use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum RessourceError {
    Directory(std::io::Error),
    PathDontExist,
    File(std::io::Error),
}

pub type Result<T> = core::result::Result<T, RessourceError>;

pub struct RessourceLoader {
    res_path: PathBuf,
}

impl RessourceLoader {
    pub fn init(path: PathBuf) -> Result<Self> {
        let exe_dir = std::env::current_dir().map_err(|e| RessourceError::Directory(e))?;
        let res_path = exe_dir.join(path);
        if !res_path.exists() {
            Err(RessourceError::PathDontExist)
        } else {
            Ok(Self { res_path })
        }
    }

    pub fn load_file(&self, file_path: &Path) -> Result<std::fs::File> {
        std::fs::File::open(self.res_path.join(file_path)).map_err(|e| RessourceError::File(e))
    }

    //pub fn load_shaders(&self, )
}
