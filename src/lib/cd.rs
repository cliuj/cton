use std::env;
use std::path::Path;

use crate::lib::token::Token;

pub fn cd(tokens: Vec<Token>) {
    if !tokens.is_empty() {
        match tokens[0].unwrap() {
            "/" => {
                let root = Path::new("/");
                env::set_current_dir(&root);
            },

            ".." => {
                if tokens[0].unwrap() != "/" {
                    let current_dir = env::current_dir().unwrap();
                    let current_dir_str = current_dir.as_path().to_str().unwrap();

                    let bytes = current_dir_str.as_bytes();
                    let mut iter = current_dir_str.len() - 1;
                    while bytes[iter] != b'/' {
                        iter -= 1;
                    }

                    //println!("{}", &current_dir_str[0..iter]);
                    env::set_current_dir(&current_dir_str[0..iter]);

                }
            },
            
            // note home_dir is deprecated so it should be removed later
            "~" => {
                match env::home_dir() {
                    Some(path) => {
                        env::set_current_dir(path);
                    },
                    None => println!("No $HOME env found"),
                }
            },

            _ => {
                match &tokens[0].unwrap()[0..1] {
                    "." => {
                        let current_dir = env::current_dir().unwrap();
                        let current_dir_str = current_dir.as_path().to_str().unwrap();
                        
                        let target_dir = format!("{}{}", current_dir_str, &tokens[0].unwrap()[1..]); 

                        if Path::new(&target_dir).exists() {
                            env::set_current_dir(target_dir);
                        } else {
                            println!("Target directory not found or does not exist");
                        }
                    }, 

                    "/" => {

                        if Path::new(&tokens[0].unwrap()).exists() {
                            env::set_current_dir(&tokens[0].unwrap());
                        } else {
                            println!("Target directory not found or does not exist");
                        }
                    },

                    _ => {
                        let current_dir = env::current_dir().unwrap();
                        let current_dir_str = current_dir.as_path().to_str().unwrap();
                        
                        let target_dir = format!("{}/{}", current_dir_str, &tokens[0].unwrap());

                        if Path::new(&target_dir).exists() {
                            env::set_current_dir(target_dir);
                        } else {
                            println!("Target directory not found or does not exist");
                        }
                        
                    }
                }

            }
        }
        
    } else {
        match env::home_dir() {
            Some(path) => {
                env::set_current_dir(path);
            },
            None => println!("No $HOME env found"),
        }

    }
}
