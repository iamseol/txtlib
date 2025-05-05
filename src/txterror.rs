use std::{io::Error, path::PathBuf, process};

pub type EmptyTxtResult = Result<(), TxtError>;
pub type TxtResult<T> = Result<T, TxtError>;

pub enum TxtError {
    ArgNotFound(usize),
    FailedReadingCWD(Error),

    FolderNotFound(PathBuf),
    FileNotFound(PathBuf),

    FailedCreatingFolder(Error, PathBuf),
    FailedCreatingFile(Error, PathBuf),

    FailedDeletingFolder(Error, PathBuf),
    FailedDeletingFile(Error, PathBuf),

    FailedReadingFolder(Error, PathBuf),
    FailedReadingFile(Error, PathBuf),
    FailedWritingFile(Error, PathBuf),
}

impl TxtError {
    pub fn fire(&self) -> ! {
        match self {
            Self::ArgNotFound(arg_index) => {
                eprintln!("the argument at {arg_index} is not found.");
            }
            Self::FailedReadingCWD(err) => {
                eprintln!("reading the current working directory is failed: {err}");
            }
            Self::FolderNotFound(folder_path) => {
                eprintln!("the folder at {} is not found.", folder_path.display());
            }
            Self::FileNotFound(file_path) => {
                eprintln!("the file at {} is not found.", file_path.display());
            }
            Self::FailedCreatingFolder(err, folder_path) => {
                eprintln!(
                    "the folder at {} is not created: {err}",
                    folder_path.display()
                );
            }
            Self::FailedCreatingFile(err, file_path) => {
                eprintln!("the file at {} is not created: {err}", file_path.display());
            }
            Self::FailedDeletingFolder(err, folder_path) => {
                eprintln!(
                    "the folder at {} is not deleted: {err}",
                    folder_path.display()
                );
            }
            Self::FailedDeletingFile(err, file_path) => {
                eprintln!("the file at {} is not deleted: {err}", file_path.display());
            }
            Self::FailedReadingFolder(err, folder_path) => {
                eprintln!(
                    "reading the folder at {} is failed: {err}",
                    folder_path.display()
                );
            }
            Self::FailedReadingFile(err, file_path) => {
                eprintln!(
                    "reading the file at {} is failed: {err}",
                    file_path.display()
                );
            }
            Self::FailedWritingFile(err, file_path) => {
                eprintln!(
                    "writing to the file at {} is failed: {err}",
                    file_path.display()
                );
            }
        };

        process::exit(1);
    }
}
