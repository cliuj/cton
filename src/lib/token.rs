#[derive(Debug)]
pub enum Token {
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
    pub fn unwrap(&self) -> &str {
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
