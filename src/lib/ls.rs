extern crate libc;
use libc::{ 
    S_IRUSR, S_IWUSR, S_IXUSR,
    S_IRGRP, S_IWGRP, S_IXGRP,
    S_IROTH, S_IWOTH, S_IXOTH,
};


use std::io;
use std::fs;
use std::io::Write;
use std::fs::{ DirEntry, FileType };
use std::os::unix::fs::{ FileTypeExt, MetadataExt, PermissionsExt };

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
    blksize: bool,
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

struct Output {
    inode: String,
    block_size: String,
    file_type: String,
    permissions: String,
    dir_links: String,
    owner: String,
    group: String,
    bytes: String,
    last_mod_date: String,
    file_name: String,
    list: String,
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
                blksize: false,
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
    let output = process_instruction(&mut instruction, files);

    // display based on options
    display(output.unwrap());
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
                instruction.list_content.blksize = true; 
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

fn process_instruction(instruction: &mut Instruction, files: Vec<&str>) -> io::Result<Vec<Output>>{
    println!("Printing from process_instruction: ");
    let mut entries: Vec<Output> = Vec::new();

    //if !instruction.valid {
    //    entries.push("invalid input");
    //    return Ok(entries)
    //}

    //println!("{:?}", instruction);
    for file in files {
        for entry in fs::read_dir(file)? {
            if let Ok(entry) = entry {
                
                let filemd = entry.metadata().unwrap();

                // every entry will have this
                let mut output = Output {
                    inode: "".to_string(),
                    block_size: "".to_string(),
                    file_type: "".to_string(),
                    permissions: "".to_string(),
                    dir_links: "".to_string(),
                    owner: "".to_string(),
                    group: "".to_string(),
                    bytes: "".to_string(),
                    last_mod_date: "".to_string(),
                    file_name: "".to_string(),
                    list: "".to_string(),
                };

                let file_name = entry.file_name().to_str().unwrap().to_string();
                
                match instruction.content {
                    ContentOption::None => {
                        if file_name.chars().next().unwrap() != '.' {
                            output.file_name = file_name;
                        } else {
                            continue;
                        }
                    },
                
                    ContentOption::All => {
                        output.file_name = file_name;
                    },
                
                    ContentOption::AlmostAll => {
                        output.file_name = file_name;
                    },
                }
                
                match instruction.format {

                    FormatOption::NoList => {
                        output.list = "   ".to_string();
                    },

                    FormatOption::List => {
                        output.list = "\n".to_string();
                    },

                    FormatOption::LongList => {
                        output.list = "\n".to_string();
                        output.permissions = format!("{} ", format_permissions(filemd.mode()));
                        output.file_type = get_filetype(&filemd.file_type());
                        let mut size = filemd.size().to_string();
                        let mut block_size = filemd.blksize().to_string();


                        if instruction.list_content.author {
                            
                        }

                        if instruction.list_content.owner {
                            output.owner = format!("{} ", filemd.uid().to_string());
                        }

                        if instruction.list_content.group {
                            output.group = format!("{} ", filemd.gid().to_string());

                        }

                        if instruction.list_content.human_readable {
                            //output.bytes = format!("{}K", output.bytes / 1000.0);
                            if filemd.size() > 1024 {
                                size = format!("{}K ", (filemd.size() / 1024).to_string());
                            }

                            if filemd.blksize() > 1024 {
                                block_size = format!("{}K ", (filemd.blksize() / 1024).to_string());
                            }
                        }

                        if instruction.list_content.blksize {
                            
                            output.block_size = block_size;
                        }
                        
                        //println!
                        let s = get_last_mod_date(convert_sec_to_date(filemd.mtime()));
                        output.last_mod_date = format!("{} ", s);
                        
                        output.dir_links = format!("{} ", filemd.nlink().to_string());
                        output.bytes = format!("{} ", size.to_string());
                    },
                }


                match instruction.name {

                    NameOption::Normal => {},

                    NameOption::DoubleQuotes => {
                        output.file_name = format!("\"{}\"", output.file_name);
                    },

                    NameOption::AppendSlash => {
                        if filemd.is_dir() {
                            output.file_name = format!("{}/", output.file_name);
                        }
                    },

                }

                match instruction.sort {
                    SortOption::Alphabetical => {

                    }

                    SortOption::Size => {
                    
                    },

                    SortOption::Time => {
                    
                    },

                    SortOption::Order => {
                    
                    },

                    SortOption::Extension => {
                    
                    },
                }

                if instruction.inode {
                    output.inode = format!("{} ", filemd.ino().to_string());
                }
                
                //println!("{:?}", meta.unwrap());


                entries.push(output);
            }
        }
    }
    Ok(entries)
}

fn get_filetype(file_type: &FileType) -> String {

    let mut ft = String::with_capacity(1);

    if file_type.is_dir() {
        ft.push('d');
    
    } else if file_type.is_symlink() {
        ft.push('l');
    
    } else if file_type.is_block_device() {
        ft.push('b');
    
    } else if file_type.is_char_device() {
        ft.push('c');
    
    } else if file_type.is_fifo() {
        // called "named pipe" hence the 'p'
        ft.push('p');
    
    } else if file_type.is_socket() {
        ft.push('s');

    } else {
        ft.push('-');
    }

    ft
}

fn format_permissions(mode: u32) -> String {
    
    let user = get_permissions(mode, S_IRUSR, S_IWUSR, S_IXUSR);
    let group = get_permissions(mode, S_IRGRP, S_IWGRP, S_IXGRP);
    let other = get_permissions(mode, S_IROTH, S_IWOTH, S_IXOTH);
    [user, group, other].join("")
}

fn get_permissions(mode: u32, read: u32, write: u32, execute: u32) -> String {
    match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, 0, _) => "r-x",
        (_, _, 0) => "rw-",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx",
    }.to_string()
}

struct Date {
    year: i64,
    month: String,
    day: i64,
    hour: i64,
    min: i64,
}

fn convert_sec_to_date(mtime: i64) -> Date {
    let year = mtime / 31556926;
    
    let mtime = mtime % 31556926;

    let month = mtime / 2629743;

    let mtime = mtime % 2629743;

    let day = mtime / 86400;

    let mtime = mtime % 86400;

    let hour = mtime / 3600;

    let mtime = mtime % 3600;

    let mins = mtime / 60;
    
    Date {
        year: year,
        month: get_month(month),
        day: day,
        hour: hour,
        min: mins,
    }
}

fn get_last_mod_date(date: Date) -> String {
    format!("{month} {day} {hour}:{min}", month=date.month, day=date.day.to_string(), hour=date.hour.to_string(), min=date.min.to_string())
}

fn get_month(month: i64) -> String {
    let mn = month % 12;
    match mn {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }.to_string()
}

//(<inode> <block_size> <file_type> <permissions> <dir_links> <owner> <group> <bytes> <data_of_last_modification> <file_name>

fn display(output: Vec<Output>) {
    println!("Printing from display: \n");
    for line in output {
        print!("{inode}{block_size}{file_type}{permissions}{dir_links}{owner}{group}{bytes}{last_mod_date}{file_name}{list}"
               ,inode=line.inode
               ,block_size=line.block_size
               ,file_type=line.file_type
               ,permissions=line.permissions
               ,dir_links=line.dir_links
               ,owner=line.owner
               ,group=line.group
               ,bytes=line.bytes
               ,last_mod_date=line.last_mod_date
               ,file_name=line.file_name
               ,list=line.list
               );
        io::stdout().flush()
            .expect("Failed to flush stdout buffer");
    }
    println!();
}
