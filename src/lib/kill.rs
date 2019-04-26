extern crate libc;

use crate::lib::token::Token;

pub fn kill(tokens: Vec<Token>) {
    if !tokens.is_empty() {
        
        let pid_t: usize = tokens[0].unwrap().parse::<usize>().unwrap();

        if unsafe { libc::kill(pid_t as libc::pid_t, libc::SIGTERM) } != 0 {
            println!("Call to libc kill failed!");
            println!("pid: {}", pid_t);
        }
    } else {
        println!("No process id passed");
    }
}
