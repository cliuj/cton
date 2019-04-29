use std::time::Duration;
use std::process;
use std::thread;

use crate::lib::token::Token;

pub fn timeout(tokens: Vec<Token>) {
    if &tokens.len() > &1 {
        let s_duration: u64 = match tokens[0].unwrap().parse::<u64>() {
            Ok(val) => val,
            Err(_) => 0,
        };


        let mut child = process::Command::new(&tokens[1].unwrap())
            .spawn()
            .expect("Couldn't run command");

        if s_duration != 0 {
            let duration = Duration::from_secs(s_duration);
            thread::sleep(duration);
        } else {
            println!("duration is 0");
        }

        child.kill().expect("Can't end child process");


    } else {
        println!("Missing valid args");
    }
}
