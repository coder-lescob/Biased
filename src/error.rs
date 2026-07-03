use crate::{interpreter::{Function, Value}, lexer::Token, parser::SyntaxNode};

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

    NotAnExpression(SyntaxNode),
    NotExpressionLiteral(Token),

    TooMuchOperands(Vec<SyntaxNode>),
    NotEnoughOperand(Vec<SyntaxNode>),

    CannotPerform(Token, Value, Value),
    CannotCast(SyntaxNode, SyntaxNode),
    UnknownType(String),

    NotAVar(SyntaxNode),
    FuncRegisterFailed,
    VariableAlreadyExists(String),
    UnknownVariable(String),
    UnknownFunction(String),

    ExpectedFuncName(SyntaxNode),
    ExpectedFuncParam(SyntaxNode),
    ExpectedFuncArgs(SyntaxNode),

    ExpectedBool(Value),
    ExpectedElseBranch(SyntaxNode),

    NotAFunction(SyntaxNode),
    VoidInExpression(SyntaxNode),
    MissingReturn(String),
    MissingFunction(Function),
    NotAFuncHeader(SyntaxNode),
    NotAnFuncOption(Token),

    ReturnIncompatibleType(Value),
}