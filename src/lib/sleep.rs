use std::{thread, time};
use std::time::Duration;

use crate::lib::token::Token;

pub fn sleep(tokens: Vec<Token>) {
    if !&tokens.is_empty() {
        
        let s_duration: u64 = match tokens[0].unwrap().parse::<u64>() {
            Ok(val) => val, 
            //Err(val) => panic!("test"),
            Err(_) => 0,
        };

        if s_duration != 0 {
            let duration = Duration::from_secs(s_duration);
            thread::sleep(duration);
        } else {
            println!("Invalid duration entered");
        }


    } else {
        println!("sleep: missing operand");
    }
}
