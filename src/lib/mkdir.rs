use std::io;
use std::fs::{self, create_dir};

use crate::lib::token::Token;




pub fn mkdir(tokens: Vec<Token>) {

    for token in &tokens {
        fs::create_dir(token.unwrap());
    }
}
