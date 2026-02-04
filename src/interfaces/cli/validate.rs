use super::Cli;
use super::error::{Error, ValidationError};
use std::fs;

impl Cli {
    pub fn validate(&self) -> Result<(), Error> {
        if let Some(file) = &self.file {
            if !file.exists() {
                return Err(Error::Validation(ValidationError::FileNotFound(
                    file.to_path_buf(),
                )));
            };

            if !file.is_file() {
                return Err(Error::Validation(ValidationError::NotAFile(
                    file.to_path_buf(),
                )));
            }
        }

        if let Some(out_dir) = &self.out_dir {
            if out_dir.exists() {
                if !out_dir.is_dir() {
                    return Err(Error::Validation(ValidationError::NotADirectory(
                        out_dir.to_path_buf(),
                    )));
                }
            } else {
                if let Err(e) = fs::create_dir_all(out_dir) {
                    return Err(Error::Validation(ValidationError::DirectoryCreationFailed(
                        out_dir.to_path_buf(),
                        e,
                    )));
                }
            }
        }
        Ok(())
    }
}
