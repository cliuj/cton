use std::io::{BufRead, BufReader, Result};
use std::fs::File;

use crate::lib::token::Token;


pub fn grep(tokens: Vec<Token>) {
    println!("\nExecuting from module 'grep'. . .");
    
    let mut pathnames: Vec<&str> = Vec::new();
    let mut patterns: Vec<&str> = Vec::new();
    
    // separate files and patterns
    for token in &tokens {
        match &token {
            Token::Misc(_) => {
                pathnames.push(&token.unwrap());
                println!("FILE: {}", &token.unwrap());
            },

            Token::SingleQuotes(_) => {
                patterns.push(&token.unwrap());
                //pattern = token.unwrap();
                println!("PATTERN: {}", &token.unwrap());
            },

            Token::DoubleQuotes(_) => {
                patterns.push(&token.unwrap());
                //pattern = token.unwrap();
                println!("PATTERN: {}", &token.unwrap());
            },

            _ => (),
        }
    }

    // open files and obtain contents
    
    if !patterns.is_empty() {
        for pathname in &pathnames {
            if let Ok(file) = File::open(pathname) {
                for line in BufReader::new(file).lines() {
                    let line_content = &line.unwrap();
                    if pattern_found(patterns[0], line_content) {
                        println!("{}", line_content);

                    }
                }
            }
        }
    }
    

    // search whether line contains the pattern
}

fn pattern_found(pattern: &str, line: &str) -> bool {
    
    let line_bytes = line.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    let eol: usize = line.len();
    let last_byte = pattern.bytes().last().unwrap();

    let mut iter: usize = 0;
    let mut scanner: usize = 0;

    while iter < eol {
        if line_bytes[iter] == last_byte {
            if iter >= pattern.len() - 1 {
                let mut offset: usize = iter - (pattern.len() - 1);
                while pattern_bytes[scanner] == line_bytes[offset] {
                    if scanner == pattern.len() - 1 {
                        return true;
                    }
                    scanner += 1;
                    offset += 1;
                }
            }
        }
        iter += 1;
    }
    return false;
}
