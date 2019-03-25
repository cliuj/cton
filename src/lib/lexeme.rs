pub enum Lexeme {
    OPTION(String),
    FILE(String),
}

impl Lexeme {
    pub fn unwrap(&self) -> &str {
        use crate::lib::lexeme::Lexeme::OPTION;
        use crate::lib::lexeme::Lexeme::FILE;
        match self {
            OPTION(val) => val,
            FILE(val) => val,
        }
    }
}
