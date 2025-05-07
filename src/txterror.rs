use std::{fs::File, io::Error, path::PathBuf, process};

pub type EmptyTxtResult = Result<(), TxtError>;
pub type TxtResult<T> = Result<T, TxtError>;

pub enum TxtError {
    // ect
    ArgNotFound(usize),
    FailedReadingCWD(Error),

    // not found
    FolderNotFound(PathBuf),
    FileNotFound(PathBuf),

    // reading
    FailedReadingFolder(Error, PathBuf),
    FailedReadingFile(Error, PathBuf),

    // creating
    FailedCreatingFolder(Error, PathBuf),
    FailedCreatingFile(Error, PathBuf),

    // deleting
    FailedDeletingFolder(Error, PathBuf),
    FailedDeletingFile(Error, PathBuf),

    // writing
    FailedWritingFile(Error, File),
    FailedWritingFileFromPath(Error, PathBuf),

    // setting
    FailedGettingFileName(PathBuf),
}

impl TxtError {
    pub fn fire(&self) -> ! {
        match self {
            // ect
            Self::ArgNotFound(arg_index) => {
                eprintln!("the argument at {arg_index} is not found.");
            }
            Self::FailedReadingCWD(err) => {
                eprintln!("reading the current working directory is failed: {err}");
            }

            // not found
            Self::FolderNotFound(folder_path) => {
                eprintln!("the folder at {} is not found.", folder_path.display());
            }
            Self::FileNotFound(file_path) => {
                eprintln!("the file at {} is not found.", file_path.display());
            }

            // reading
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

            // creating
            Self::FailedCreatingFolder(err, folder_path) => {
                eprintln!(
                    "the folder at {} is not created: {err}",
                    folder_path.display()
                );
            }
            Self::FailedCreatingFile(err, file_path) => {
                eprintln!("the file at {} is not created: {err}", file_path.display());
            }

            // deleting
            Self::FailedDeletingFolder(err, folder_path) => {
                eprintln!(
                    "the folder at {} is not deleted: {err}",
                    folder_path.display()
                );
            }
            Self::FailedDeletingFile(err, file_path) => {
                eprintln!("the file at {} is not deleted: {err}", file_path.display());
            }

            // writing
            Self::FailedWritingFile(err, file) => {
                eprintln!("writing to the file({file:#?}) is failed: {err}");
            }
            Self::FailedWritingFileFromPath(err, file_path) => {
                eprintln!(
                    "writing to the file at {} is failed: {err}",
                    file_path.display()
                );
            }

            // setting
            Self::FailedGettingFileName(file_path) => {
                eprintln!("getting the file at {} is failed", file_path.display());
            }
        };

        process::exit(1);
    }
}
