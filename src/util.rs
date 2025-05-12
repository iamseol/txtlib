use crate::txterror::{EmptyTxtResult, TxtError, TxtResult};
use std::{
    env,
    fs::{self, DirEntry, File},
    io::{Read, Write},
    path::PathBuf,
};

pub fn get_cwd() -> TxtResult<PathBuf> {
    env::current_dir().map_err(|e| TxtError::FailedReadingCWD(e))
}

pub fn get_user_arg(arg_index: usize) -> TxtResult<String> {
    env::args()
        .nth(arg_index)
        .ok_or(TxtError::ArgNotFound(arg_index))
}

pub fn create_new_folder(file_path: &PathBuf) -> EmptyTxtResult {
    fs::create_dir(file_path)
        .map_err(|e| TxtError::FailedCreatingFolder(e, file_path.to_path_buf()))
}

pub fn create_new_file(file_path: &PathBuf) -> TxtResult<File> {
    File::create_new(file_path)
        .map_err(|e| TxtError::FailedCreatingFile(e, file_path.to_path_buf()))
}

pub fn create_clean_folder(folder_path: &PathBuf) -> EmptyTxtResult {
    if folder_path.exists() && folder_path.is_dir() {
        delete_folder(folder_path)?
    }

    create_new_folder(folder_path)
}

pub fn create_clean_file(file_path: &PathBuf) -> TxtResult<File> {
    if file_path.exists() && file_path.is_file() {
        delete_file(file_path)?
    }

    create_new_file(file_path)
}

pub fn write_new_file(file_path: &PathBuf, content: &str) -> EmptyTxtResult {
    create_clean_file(file_path)?
        .write_all(content.as_bytes())
        .map_err(|e| TxtError::FailedWritingFileFromPath(e, file_path.to_path_buf()))
}

pub fn write_file(mut selected_file: File, content: &str) -> EmptyTxtResult {
    selected_file
        .write_all(content.as_bytes())
        .map_err(|e| TxtError::FailedWritingFile(e, selected_file))
}

pub fn get_file_stem(file_path: &PathBuf) -> TxtResult<&str> {
    Ok(file_path
        .file_stem()
        .ok_or(TxtError::FailedGettingFileName(file_path.to_path_buf()))?
        .to_str()
        .ok_or(TxtError::FailedGettingFileName(file_path.to_path_buf()))?)
}

pub fn get_whole_file_name(file_path: &PathBuf) -> TxtResult<&str> {
    Ok(file_path
        .file_name()
        .ok_or(TxtError::FailedGettingFileName(file_path.to_path_buf()))?
        .to_str()
        .ok_or(TxtError::FailedGettingFileName(file_path.to_path_buf()))?)
}

pub fn delete_folder(folder_path: &PathBuf) -> EmptyTxtResult {
    fs::remove_dir_all(folder_path)
        .map_err(|e| TxtError::FailedDeletingFolder(e, folder_path.to_path_buf()))
}

pub fn delete_file(file_path: &PathBuf) -> EmptyTxtResult {
    fs::remove_file(file_path).map_err(|e| TxtError::FailedDeletingFile(e, file_path.to_path_buf()))
}

pub fn get_entries(folder_path: &PathBuf) -> TxtResult<Vec<DirEntry>> {
    if !folder_path.exists() || folder_path.is_file() {
        return Err(TxtError::FolderNotFound(folder_path.to_path_buf()));
    }

    folder_path
        .read_dir()
        .map_err(|e| TxtError::FailedReadingFolder(e, folder_path.to_path_buf()))?
        .map(|current_entry| {
            current_entry.map_err(|e| TxtError::FailedReadingFolder(e, folder_path.to_path_buf()))
        })
        .collect::<Result<Vec<DirEntry>, TxtError>>()
}

pub fn read_file(buf: &mut String, file_path: &PathBuf) -> EmptyTxtResult {
    if !file_path.exists() || file_path.is_dir() {
        return Err(TxtError::FileNotFound(file_path.to_path_buf()));
    }

    File::open(file_path)
        .map_err(|e| TxtError::FailedReadingFile(e, file_path.to_path_buf()))?
        .read_to_string(buf)
        .map_err(|e| TxtError::FailedReadingFile(e, file_path.to_path_buf()))?;

    Ok(())
}

pub fn copy_all_folder(src: &PathBuf, des: &PathBuf) -> EmptyTxtResult {
    fs::create_dir_all(&des).unwrap();

    for current_entry in get_entries(src)? {
        let entry_type = current_entry.file_type().unwrap();

        if entry_type.is_dir() {
            copy_all_folder(&current_entry.path(), &des.join(current_entry.file_name()))?;
        } else {
            fs::copy(current_entry.path(), des.join(current_entry.file_name())).unwrap();
        }
    }

    Ok(())
}
