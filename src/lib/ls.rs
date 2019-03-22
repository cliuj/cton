use std::fs;
use std::io;
use std::io::Write;

pub fn ls(cmd: Vec<&str>) {
    println!("Executing from module 'ls'. . .");

    println!("cmd line: {:?}", cmd);

    //if cmd.copy().len() {
    match cmd.len() {
        // run ls with no args 
        1 => ls_noargs(),
        _ => println!("cmd length is greater than 1!")
    };
    //}
    // ls_opts

    // ls_all
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
