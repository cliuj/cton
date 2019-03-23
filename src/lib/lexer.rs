use crate::lib::lexeme::Lexeme;

pub fn lexer(cmd: &[&str]) -> Vec<Lexeme> {

    let mut lexemes: Vec<Lexeme> = Vec::new();
    
    // check if the word starts with "--" or "-" for options
    for word in cmd {
        
        if word.starts_with("--") && word.chars().count() > 2{
            lexemes.push( Lexeme::OPTION(word.to_string()));

        } else if word.starts_with("-") && word.chars().count() > 1 && !word.starts_with("--"){

            lexemes.push( Lexeme::OPTION(word.to_string()));

        } else {

            lexemes.push( Lexeme::FILE(word.to_string()));
            println!("{} pushed to lexemes", word.to_string());
        }
    }

    return lexemes
}
