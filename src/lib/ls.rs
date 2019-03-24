use std::fs;
use std::io;
use std::io::Write;
use std::fs::DirEntry;
use std::os::unix::fs::MetadataExt;
//use std::os::unix::fs::DirEntryExt;
use std::os::unix::fs::PermissionsExt;

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
    LongList,
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
    AppendSlash,
}

#[derive(Debug)]
enum SortOption {
    Alphabetical,
    Size,
    Time,
    Order,
    Extension,
}

#[derive(Debug)]
struct Instruction {
    valid: bool,
    inode: bool,
    reverse_sort: bool,
    format: FormatOption,
    content: ContentOption,
    name: NameOption,
    sort: SortOption,
    list_content: ListContent,
}

pub fn ls(lexemes: Vec<Lexeme>) {
    println!("\nExecuting from module 'ls'. . .");

    let mut options: HashSet<String> = HashSet::new();
    let mut files: Vec<&str> = Vec::new();
    
    let mut instruction = Instruction {
        valid: true,
        inode: false,
        reverse_sort: false,
        format: FormatOption::NoList,
        content: ContentOption::None,
        name: NameOption::Normal,
        sort: SortOption::Alphabetical,
        list_content: 
            ListContent {
                author: false,
                owner: true,
                group: true,
                human_readable: false,
                size: true,
            },
    };

    if lexemes.len() == 0 {
        files.push(".");
        let output = process_instruction(&mut instruction, files);
        display(output.unwrap());
        return;
    }

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

    if files.is_empty() {
        files.push(".");
    }

    // process options 
    process_options(&mut instruction, options);

    // process instruction
    process_instruction(&mut instruction, files);

    // display based on options
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
        option_set.insert("-".to_owned() + &op.to_string());
    }

    return option_set;
}


/// Create the package instructions (format struct) on how to display.
/// Check and verify that the options are valid.
/// 
/// 
/// 
fn process_options(instruction: &mut Instruction, options: HashSet<String>) {
    println!("\nPrinting from function process_options");


    for option in &options {
        match option.as_ref() {
            "-a" | "--all" => {
                println!("{} confirmed", option);
                instruction.content = ContentOption::All;
            },

            "-A" | "--almost-all" => {
                println!("{} confirmed", option);
                instruction.content = ContentOption::AlmostAll;
            },

            "--author" => {
                println!("{} confirmed", option);
                instruction.list_content.author = true;
            },
            
            "-g" => {
                println!("{} confirmed", option);
                instruction.list_content.owner = false;
                instruction.format = FormatOption::LongList;
            },

            "-G" | "--no-group" => {
                println!("{} confirmed", option);
                instruction.list_content.group = false;
            },

            "-h" | "--human-readable" => {
                println!("{} confirmed", option);
                instruction.list_content.human_readable = true;
            },

            "-i" | "--inode" => {
                println!("{} confirmed", option);
                instruction.inode = true;
            }

            "-l" => {
                println!("{} confirmed", option);
                instruction.format = FormatOption::LongList;
            },

            "-o" => {
                println!("{} confirmed", option);
                instruction.list_content.group = false;
            },
            
            "-p" | "--indicator-style=slash" => {
                println!("{} confirmed", option);
                instruction.name = NameOption::AppendSlash;
            },
            
            "-Q" | "--quote-name" => {
                println!("{} confirmed", option);
                instruction.name = NameOption::DoubleQuotes;
            },
            
            "-r" | "--reverse" => {
                println!("{} confirmed", option);
                instruction.reverse_sort = true;
            },
            
            "-s" | "--size" => {
                println!("{} confirmed", option);
                instruction.list_content.size = true; 
            },
            
            "-S" => {
                println!("{} confirmed", option);
                instruction.sort = SortOption::Size;
            },
            
            "-t" => {
                println!("{} confirmed", option);
                instruction.sort = SortOption::Time;
            },
            
            "-U" => {
                println!("{} confirmed", option);
                instruction.sort = SortOption::Order;
            },
            
            "-X" => {
                println!("{} confirmed", option);
                instruction.sort = SortOption::Extension;
            },
            
            "-1" => {
                println!("{} confirmed", option);
                instruction.format = FormatOption::List;
            },

            _ => {
                instruction.valid = false; 
                println!("Invalid option passed: {}", option); 
                break; 
            }
        }
    }
}

fn process_instruction(instruction: &mut Instruction, files: Vec<&str>) -> io::Result<Vec<String>>{
    println!("Printing from process_instruction: ");
    let mut output: Vec<String> = Vec::new();

    if !instruction.valid {
        output.push("Invalid input".to_string());
        return Ok(output)
    }
    
    println!("{:?}", instruction);
    for file in files {
        for entry in fs::read_dir(file)? {
            if let Ok(entry) = entry {
                match instruction.format {

                    FormatOption::NoList => {
                        match_content(instruction, &mut output, &entry);
                    },

                    FormatOption::List => {
                        println!("List reached");
                    },

                    FormatOption::LongList => {
                        println!("LongList");

                    },
                }
            }
        }
    }
    
    println!("Contents of output: ");
    for line in &output {
        println!("{}", line);
    }

    Ok(output)
}


fn match_content(instruction: &Instruction, output: &mut Vec<String>, entry: &DirEntry)  {

    let file_name = entry.file_name();
    match instruction.content {
        ContentOption::None => {
            if !file_name.to_str().map(|s| s.starts_with(".")).unwrap() {
                output.push(format!("{}", file_name.to_str().unwrap()));
            }
        },
    
        ContentOption::All => {
            output.push(format!("{}", file_name.to_str().unwrap()));
        },
    
        ContentOption::AlmostAll => {
            if file_name.to_str().unwrap() != ".." || file_name.to_str().unwrap() != "." {
                output.push(format!("{}", file_name.to_str().unwrap()));
            }
        },
    }
}









fn display(output: Vec<String>) {
    println!("Printing from display: ");
    for line in output {
        print!("{}", line);
        io::stdout().flush()
            .expect("Failed to flush stdout buffer");
    }
}
