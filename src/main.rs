use std::io;
use std::io::Write;
use std::vec::Vec;
use std::convert::AsRef;

mod lib;

fn main() {
    loop {
        print!("~> ");
        // flush buffer or print! won't print immediately!
        io::stdout().flush()
            .expect("Failed to flush stdout buffer");

        let input = { 
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line");
            input   
        };
        
        let cmd = input.trim().split_whitespace().collect::<Vec<&str>>();

        match cmd[0].as_ref() {
            "ls" => {
                lib::ls::ls(cmd);
            },

            "clear" => {
                print!("\x1b[2J\x1b[1;1H");
            },

            "exit" => {
                print!("Exiting shell!");
                break;
            },
            _ => {}
        };
    };
}
