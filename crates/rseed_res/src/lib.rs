use std::{io::Read, path::{Path, PathBuf}};

#[derive(Debug)]
pub enum RessourceError {
    Directory(std::io::Error),
    PathDontExist,
    FileContainsNullByte,
    File(std::io::Error),
}

pub type Result<T> = std::result::Result<T, RessourceError>;

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
        std::fs::File::open(self.res_path.join(file_path))
            .map_err(|e| RessourceError::File(e))
    }

    pub fn load_as_cstring(&self, file_path : &Path) -> Result<std::ffi::CString> {
        let mut file = std::fs::File::open(self.res_path.join(file_path))
            .map_err(|e| RessourceError::File(e))?;

        let mut content: Vec<u8> = Vec::with_capacity(
            file.metadata()
            .map_err(|e| RessourceError::File(e))?
            .len() as usize + 1
        );
        file.read_to_end(&mut content)
            .map_err(|e| RessourceError::File(e))?;

        if content.iter().find(|i| **i == 0).is_some() {
            return Err(RessourceError::FileContainsNullByte);
        }

        Ok(unsafe { std::ffi::CString::from_vec_unchecked(content) })
    }

    //pub fn load_shaders(&self, )
}
