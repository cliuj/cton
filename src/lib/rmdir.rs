use std::io;
use std::fs;

use crate::lib::token::Token;

pub fn rmdir(tokens: Vec<Token>) {
    for token in &tokens {
        fs::remove_dir(token.unwrap());
    }
}
