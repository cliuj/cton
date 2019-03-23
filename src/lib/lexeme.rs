//pub const OPTION: i8 = 1;
//pub const FILE: i8 = 2;

pub enum Lexeme {
    OPTION(String),
    FILE(String),
    None,
}

impl Lexeme {
    pub fn unwrap(self) -> String {
        use crate::lib::lexeme::Lexeme::OPTION;
        use crate::lib::lexeme::Lexeme::FILE;
        use crate::lib::lexeme::Lexeme::None;
        match self {
            OPTION(val) => val,
            FILE(val) => val,
            None => panic!("called `Lexeme::unwrap()` on a `None` value"),
        }
    }
}
