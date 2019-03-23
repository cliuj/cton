use std::fs;

use crate::lib::lexeme::Lexeme;

pub fn ls(lexemes: Vec<Lexeme>) {
    println!("Executing from module 'ls'. . .");
    
    //for lexeme in lexemes {
    //    println!("{}", lexeme.unwrap());
    //}





}

fn ls_noargs() {
    println!("ls no args function called!");

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                if !entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap() {
                    print!("{}   ", entry.file_name().to_str().unwrap());
                }
            }
        }
    }
    println!();
}

