use crate::lexer::Token;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
    UnableToOpenFile(String),

    LexicalError(String),

    Expected          (Vec<Token>, Token),
    ExpectedIdentifier(Token),
    ExpectedExpr      (Token),
    NotAnOperator     (Token),

    UnknownInstruction,
}