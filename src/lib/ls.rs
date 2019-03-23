use std::fs;
use std::collections::HashSet;

use crate::lib::lexeme::Lexeme;

#[derive(Debug)]
enum ContentOption {
    All,
    AlmostAll,
    None,
}

#[derive(Debug)]
enum FormatOption {
    List,
    LongList(ListContent),
    NoList,
}

#[derive(Debug)]
struct ListContent {
    author: bool,
    owner: bool,
    group: bool,
    human_readable: bool,
    size: bool,
}

#[derive(Debug)]
enum NameOption {
    Normal,
    DoubleQuotes,
}

#[derive(Debug)]
enum SortOption {
    Alphabetical,
    Size,
    Time,
    Order,
    ReverseAlphabetical,
    ReverseSize,
    ReverseTime,
    ReverseOrder,
}

#[derive(Debug)]
struct Instruction {
    valid: bool,
    format: FormatOption,
    content: ContentOption,
    name: NameOption,
    sort: SortOption,
}

pub fn ls(lexemes: Vec<Lexeme>) {
    println!("\nExecuting from module 'ls'. . .");


    let mut options: HashSet<String> = HashSet::new();
    let mut files: Vec<&str> = Vec::new();

    if lexemes.len() > 0 {
        
        // separate option and files
        for lexeme in &lexemes {
            match &lexeme {
                Lexeme::OPTION(_option) => {
                    
                    if !lexeme.unwrap().starts_with("--") {
                        println!("OPTION: {}", &lexeme.unwrap());
                        options.extend(parse_options(&lexeme.unwrap()));
                    } else {
                        options.insert(lexeme.unwrap().to_string());
                    }
                },

                Lexeme::FILE(_file) => {
                    files.push(&lexeme.unwrap());
                    println!("FILE: {}", &lexeme.unwrap());
                },
            }
        }
        
        println!("\nOptions vec:");
        for option in &options {
            println!("{}", &option);
        }
        
        println!("\nFiles vec:");
        for file in &files {
            println!("{}", &file);
        }

        // process options 
        
        let instruction = process_options(options);

        // display based on options
        display(&instruction);


    } else {
        ls_noargs();
    }
}

fn ls_noargs() {
    println!("ls no args function called!");

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                if !entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap() {
                    print!("{}   ", entry.file_name().to_str().unwrap());
                }
            }
        }
    }
    println!();
}


/// Parse a option glob string and returns a hashset of the option chars
///
/// [Note]
/// The char(s) are in no way being checked if they are valid option(s) or not.
/// The checking procedure should be done during actual option matching.
/// 
/// [Example]
/// "-lasssssss" --> {"-l", "-a", "-s"}
///
fn parse_options(options: &str) -> HashSet<String> {
    println!("\nPrinting from function parse_options");
    
    let mut option_set: HashSet<String> = HashSet::new();

    let char_set: Vec<char> = options.chars().collect();
    
    for op in &char_set[1..] {
        //println!("{}", op);
        option_set.insert("-".to_owned() + &op.to_string());
    }

    return option_set;
}


/// Create the package instructions (format struct) on how to display.
/// Check and verify that the options are valid.
/// 
/// 
/// 
fn process_options(options: HashSet<String>) -> Instruction {
    println!("\nPrinting from function process_options");
    let mut instruction = Instruction {
        valid: true,
        format: FormatOption::NoList,
        content: ContentOption::None,
        name: NameOption::Normal,
        sort: SortOption::Alphabetical,
    };

    for option in &options {
        match option.as_ref() {
            "-a" | "--all" => {
                println!("{} confirmed", option);
            },

            "-A" | "--almost-all" => {
                println!("{} confirmed", option);
            },

            "--author" => {
                println!("{} confirmed", option);
            },
            
            "-g" => {
                println!("{} confirmed", option);
            },
            
            "--group-directories-first" => {
                println!("{} confirmed", option);
            },

            "-G" | "--no-group" => {
                println!("{} confirmed", option);
            },


            "-h" | "--human-readable" => {
                println!("{} confirmed", option);
            },

            "-l" => {
                println!("{} confirmed", option);  
            },

            "-o" => {
                println!("{} confirmed", option);
            },

            "-p" | "--indicator-style=slash" => {
                println!("{} confirmed", option);
            },
            
            "-Q" | "--quote-name" => {
                println!("{} confirmed", option);
            },
            
            "-r" | "--reverse" => {
                println!("{} confirmed", option);
            },
            
            "-s" | "--size" => {
                println!("{} confirmed", option);
            },
            
            "-S" => {
                println!("{} confirmed", option);
            },
            
            "-t" => {
                println!("{} confirmed", option);
            },
            
            "-U" => {
                println!("{} confirmed", option);
            },
            
            "-X" => {
                println!("{} confirmed", option);
            },
            
            "-1" => {
                instruction.format = FormatOption::List;
                println!("{} confirmed", option);

            },

            _ => {
                instruction.valid = false; 
                println!("Invalid option passed: {}", option); 
                break; 
            }
        }
    }

    return instruction
}

fn display(instruction: &Instruction) {
    println!("{:?}", instruction);
}

