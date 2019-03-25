use std::fs::{metadata, File};
use std::io::{self, stderr, stdout, Read, Write};
use std::collections::HashSet;

use quick_error::ResultExt;

use crate::lib::lexeme::Lexeme;

quick_error! {
    #[derive(Debug)]
    enum CatError {
        /// Invalid path
        Input(err: io::Error, path: String) {
            display("cat: {0}: {1}", path, err)
            context(path: &'a str, err: io::Error) -> (err, path.to_owned())
            cause(err)
        }

        /// Path error with no context
        Output(err: io::Error) {
            display("cat: {0}", err) from()
            cause(err)
        }

        /// Unknown filetype
        UnknownFileType(path: String) {
            display("cat: {0}: unknown filetype", path)
        }

        /// `cat` operation on a directory
        IsDirectory(path: String) {
            display("cat: {0}: Is a directory", path)
        }

        /// Other encountered errors
        EncounteredErrors(count: usize) {
            display("cat: encountered {0} errors", count)
        }
    }
}

enum InputType {
    Directory,
    File
}

struct InputHandle {
    reader: Box<Read>,
}

type CatResult<T> = Result<T, CatError>;

pub fn cat(lexemes: Vec<Lexeme>) {

   println!("\n Executing from module 'cat'....");

   let mut _options: HashSet<String> = HashSet::new();
   let mut files: Vec<String> = Vec::new();

   for lexeme in &lexemes {
       match &lexeme {
           Lexeme::OPTION(_option) => {

           },

           Lexeme::FILE(_file) => {
            files.push(lexeme.unwrap().to_string());
           }
       }
   }

   print(files).is_ok();

}

fn validate_input_type(path: &str) -> CatResult<InputType> {
    match metadata(path).context(path)?.file_type() {
        ft if ft.is_dir() => Ok(InputType::Directory),
        ft if ft.is_file() => Ok(InputType::File),
        _ => Err(CatError::UnknownFileType(path.to_owned()))
    }
}

fn open(path: &str) -> CatResult<InputHandle> {
    match validate_input_type(path)? {
        InputType::Directory => Err(CatError::IsDirectory(path.to_owned())),
        _ => {
            let file = File::open(path).context(path)?;
            Ok(InputHandle {
                reader: Box::new(file) as Box<Read>
            })
        }
    }
}

fn print(files: Vec<String>) -> CatResult<()> {
    let mut writer = stdout();
    let mut input_buffer = [0; 1024 * 64];
    let mut error_count = 0;

    for file in files {
        match open(&file[..]) {
            Ok(mut handle) => while let Ok(n) = handle.reader.read(&mut input_buffer) {
                if n == 0 {
                    break;
                }
                writer.write_all(&input_buffer[..n]).context(&file[..])?;
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
