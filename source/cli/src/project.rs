use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const CONFIG_NAME: &str = "OPC UA Data Client Module_Config.xml";

pub(crate) fn find_opc_config(
    project_dir: &Path,
) -> io::Result<PathBuf> {
    let mut directories = vec![project_dir.to_path_buf()];
    let mut found = None;

    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(&directory)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                directories.push(entry.path());
            } else if file_type.is_file()
                && entry.file_name() == CONFIG_NAME
            {
                if found.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        format!(
                            "в проекте несколько файлов {CONFIG_NAME}"
                        ),
                    ));
                }

                found = Some(entry.path());
            }
        }
    }

    found.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("в проекте не найден {CONFIG_NAME}"),
        )
    })
}