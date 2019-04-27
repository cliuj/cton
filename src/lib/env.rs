use std::env;

use crate::lib::token::Token;

pub fn env() {
    for (n, v) in env::vars() {
        println!("{}={}", n, v);
    }
}
