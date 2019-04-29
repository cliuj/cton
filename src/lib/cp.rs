use std::fs;

use crate::lib::token::Token;

pub fn cp(tokens: Vec<Token>) {

    if tokens.len() > 1 {
        let data = fs::read_to_string(tokens[0].unwrap()).expect("Unable to read file");
        println!("{}", data);

        fs::write(tokens[1].unwrap(), data).expect("Unable to write file!");

    } else {
        println!("Invalid number of arguments");
    }
}
