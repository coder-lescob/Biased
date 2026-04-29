use crate::lexer::Token;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
    LexicalError(String),

    Expected          (Token, Token),
    ExpectedIdentifier(Token),
    ExpectedExpr      (Token),
    NotAnOperator     (Token),

    UnknownInstruction,
}