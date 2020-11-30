use std::ffi::OsStr;
use std::path::Path;

pub fn get_extension_from_filename(path: &Path) -> Option<&str> {
	path.extension().and_then(OsStr::to_str)
}
