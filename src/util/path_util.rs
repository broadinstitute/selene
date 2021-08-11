use std::path::Path;
use crate::util::error::Error;

pub(crate) fn path_to_string(path: &Path) -> Result<String, Error> {
    let path_str = path.to_str().ok_or_else(||Error::from(
        format!("Path not representable as string: '{}'", path.to_string_lossy())))?;
    Ok(String::from(path_str))
}

pub(crate) fn replace_file_name(path_string: &str, file_name: &str) -> Result<String, Error> {
    let path = Path::new(path_string);
    let path_buf = path.with_file_name(file_name);
    let path_new = path_buf.as_path();
    path_to_string(path_new)
}
