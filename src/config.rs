use crate::error::AppError;
use std::path::{Path, PathBuf};

pub fn resolve_data_dir(
    override_dir: Option<&Path>,
    xdg: Option<&Path>,
    home: Option<&Path>,
) -> Result<PathBuf, AppError> {
    if let Some(path) = override_dir {
        return if path.is_absolute() {
            Ok(path.to_owned())
        } else {
            Err(AppError::InvalidDataDir)
        };
    }
    if let Some(path) = xdg.filter(|path| path.is_absolute()) {
        return Ok(path.join("rust-gpui"));
    }
    home.filter(|path| path.is_absolute())
        .map(|path| path.join(".local/share/rust-gpui"))
        .ok_or(AppError::InvalidDataDir)
}
