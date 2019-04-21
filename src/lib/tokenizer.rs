use crate::lib::token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
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
                            b'<' => { 
                                iter += 1; 
                                lexeme.push(bytes[iter] as char); 
                            },
                            b'>' => { 
                                iter += 1; 
                                lexeme.push(bytes[iter] as char); 
                                break; 
                            },
                            _ => { break; }
                        }
                    }
                    tokens.push(Token::InputOperator(lexeme.clone()));
                    lexeme.clear();
                },

                b'>' => {
                    lexeme.push(bytes[iter] as char);
                    match bytes[iter + 1] {
                        b'>' => { 
                            iter += 1; 
                            lexeme.push(bytes[iter] as char); 
                        },
                        b'|' => { 
                            iter += 1; 
                            lexeme.push(bytes[iter] as char); 
                        },
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
    return tokens;
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
