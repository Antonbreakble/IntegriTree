//! Shared fixture paths for the standalone workspace test package.

use std::path::PathBuf;

pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures").join(name)
}
