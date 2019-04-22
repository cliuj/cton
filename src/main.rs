#[macro_use]
extern crate quick_error;

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

        // parse cmd and decide if certain string is either option or dir/file

        //let lexemes = lib::lexer::lexer(&cmd[1..]);

        //for lex in lexemes {
        //    println!("Input: {}, Kind: {}", lex.input, lex.kind);
        //}

        let mut tokens = lib::tokenizer::tokenize(input.trim());

        for token in &tokens {
            println!("{:?}", token);
        }

        

        match cmd[0].as_ref() {
            "ls" => {
                tokens.remove(0);
                lib::ls::ls(tokens);
            },

            "cat" => {
                //lib::cat::cat(lexemes);
            }

            "clear" => {
                print!("\x1b[2J\x1b[1;1H");
            },

            "exit" => {
                println!("Exiting shell!");
                break;
            }
            _ => {}
        };
    };
}
