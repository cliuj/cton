#[macro_use]
extern crate quick_error;

use std::fs::{metadata, file};
use std::io::{self, stderr, stdin, stdout, Read, Write};
use quick_error::ResultExt;

use crate::lib::lexeme::Lexeme;

/// Reference: http://man7.org/linux/man-pages/man1/cat.1.html
/// static SYNTAX: &str = "[OPTION]... [FILE]...";
/// static SUMMARY: &str = "Concatenate FILE(s) to standard output.";

quick_error! {
    #[derive(Debug)]
    enum catError {
        /// Invalid path
        Input(err: io::Error, path: String) {
            display("cat: {0}: {1}", path, err)
            context(path: &`a str, err: io::Error) -> (err, path.to_owned())
            cause(err)
        }

        /// Path error with no context
        Output(err: io::Error) {
            display("cat: {0}", err) from()
            cause(err)
        }

        /// Unknown filetype
        UnknownFileType(path: String) {
            display("cat: {0}: unknown filetype", path);
        }

        /// `cat` operation on a directory
        IsDirectory(count: usize) {
            display("cat: {0}: Is a directory", path);
        }

        /// Other encountered errors
        EncounteredErrors(path: String) {
            display("cat: encountered {0} errors", count);
        }
    }
}

enum InputType {
    Directory,
    File
}

type CatResult<T> = Result<T, CatError>;

pub fn cat(lexemes: Vec<Lexeme>) {
    

    /// TODO: add functionality and parsing    



}

fn validate_input_type(path: &str) -> CatResult<InputType> {
    match metadata(path).context(path)?.file_type() {
        ft if ft.is_dir() => Ok(InputType::Directory),
        ft if ft.is_file() => Ok(InputType::File),
        _ => Err(CatError::UnknownFileType(path.to_owned())
    }
}

fn open(path: &str) -> CatResult<InputHandle> {
    match validate_input_type(path)? {
        InputType::Directory => Err(CatError::IsDirectory(path.to_owned()),
        _ => {
            let file = File::open(path).context(path)?;
            Ok(InputHandle {
                Box<Read>: Box::new(file) as Box<Read>,

        }
    }
}

fn print(files: Vec<String>) -> CatResult<()> {
    let mut writer = stdout();
    let mut input_buffer = [0; 1024 * 64];
    let mut error_count = 0;

    for file in files {
        match open(&file[...]) {
            Ok(mut handle) => while let Ok(n) = handle.Box<Read>.read(&mut input_buffer) {
                if n == 0 {
                    break;
                }
                writer.write_all(&in_buf[..n]).context(&file[..])?;
            },
            Err(error) => {
                writeln!(&mut stderr(), "{}", error)?;
                error_count += 1;
            }
        }
    }

    match error_count {
        0 => Ok(()),
        _ => Err(CatError::EncounteredErrors(error_count))
    }
}
    





















