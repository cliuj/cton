//use std::io;
//use std::io::Write;

#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    Misc(String),
    Option(String),
    InputOperator(String),
    OutputOperator(String),
    PipeOperator(String),
    LogicalOperator(String),
    Terminator(String),
    SingleQuotes(String),
    DoubleQuotes(String),
}

impl Token {
    fn unwrap(&self) -> &str {
        use Token::*;
        match self {
            Misc(data) => data,
            Option(data) => data,
            InputOperator(data) => data,
            OutputOperator(data) => data,
            PipeOperator(data) => data,
            LogicalOperator(data) => data,
            Terminator(data) => data,
            SingleQuotes(data) => data,
            DoubleQuotes(data) => data,
        }
    }
}

fn main() {
    //print!("~> ");
    //io::stdout().flush()
    //    .expect("Faoled to flush stdout buffer");

    //let input = {
    //    let mut input = String::new();
    //    io::stdin().read_line(&mut input)
    //        .expect("Failed to read line");
    //    input
    //};
    
    let input: String = String::from("   H<el|lo world<test.txt <> \"so>>mething something 123\" -l --s|ize -test < \'something2 test test test\' |&echo \"test\">>cat test>test2'foo' << | foobar; echo off ||__unknown;grep \"apple\" a a a <<   test--test-test,");
    
    //let input: String = String::from("   echo a  ae e e rr e | grep    e  < > < |");

    //let input:String = String::from("echo \"this is a test\" >> test.txt && cat test.txt | grep \"test\"");

    println!("Original String: {}\n", &input.trim());
    tokenize(&input.trim());
}

fn tokenize(input: &str) {
    let eol: usize = input.len();
    let bytes = input.as_bytes();
    
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter: usize = 0;
    let mut lexeme = String::new();

    while iter < eol {

        if is_terminator(bytes[iter]) {
            if !lexeme.is_empty() {
                tokens.push(Token::Misc(lexeme.clone()));
                lexeme.clear();
            }
        }
        
        if iter < eol - 1 {
            match bytes[iter] {
                b' ' => {
                    iter += 1;
                    continue;
                },

                b'<' => {
                    lexeme.push(bytes[iter] as char);
                    for _ in 1..2 {
                        match bytes[iter + 1] {
                            b'<' => { iter += 1; lexeme.push(bytes[iter] as char); },
                            b'>' => { iter += 1; lexeme.push(bytes[iter] as char); break; },
                            _ => { break; }
                        }
                    }
                    tokens.push(Token::InputOperator(lexeme.clone()));
                    lexeme.clear();
                },

                b'>' => {
                    lexeme.push(bytes[iter] as char);
                    match bytes[iter + 1] {
                        b'>' => { iter += 1; lexeme.push(bytes[iter] as char); },
                        b'|' => { iter += 1; lexeme.push(bytes[iter] as char); },
                        _ => ()
                    }
                    tokens.push(Token::OutputOperator(lexeme.clone()));
                    lexeme.clear();
                }

                b'"' => {
                    iter += 1;
                    while bytes[iter] != b'"'{
                        lexeme.push(bytes[iter] as char);
                        iter += 1;
                    } 
                    tokens.push(Token::DoubleQuotes(lexeme.clone()));
                    lexeme.clear();
                },

                b'\'' => {
                    iter += 1;
                    while bytes[iter] != b'\'' {
                        lexeme.push(bytes[iter] as char);
                        iter += 1;
                    }
                    tokens.push(Token::SingleQuotes(lexeme.clone()));
                    lexeme.clear();
                },

                b'|' => {
                    lexeme.push(bytes[iter] as char);
                    match bytes[iter + 1] {
                        b'|' => { 
                            iter += 1; 
                            lexeme.push(bytes[iter] as char); 
                            tokens.push(Token::LogicalOperator(lexeme.clone())); 
                        },
                        b'&' => { 
                            iter += 1; 
                            lexeme.push(bytes[iter] as char);
                            tokens.push(Token::PipeOperator(lexeme.clone()));
                        },
                        _ => tokens.push(Token::PipeOperator(lexeme.clone())),
                    }
                    lexeme.clear();
                },
                
                b'-' => {
                    if bytes[iter - 1] == b' ' {
                        while bytes[iter] != b' ' && iter < eol - 1 {
                            lexeme.push(bytes[iter] as char);
                            iter += 1;
                        }
                        tokens.push(Token::Option(lexeme.clone()));
                        lexeme.clear();
                    } else {
                        lexeme.push(bytes[iter] as char);
                    }
                },
                    
                b';' => {
                    lexeme.push(bytes[iter] as char);
                    tokens.push(Token::Terminator(lexeme.clone()));
                    lexeme.clear();
                },

                _ => {
                    lexeme.push(bytes[iter] as char);
                }
            }

        } else {
            lexeme.push(bytes[iter] as char);
            tokens.push(Token::Misc(lexeme.clone()));
        }

        iter += 1;
    }
    
    for token in tokens {
        println!("{:?}", token);
    }
}

fn is_terminator(character: u8) -> bool {
    match character {
        b' ' => true,
        b'|' => true,
        b'&' => true,
        b'>' => true,
        b'<' => true,
        b';' => true,
        _ => false,
    }
}
