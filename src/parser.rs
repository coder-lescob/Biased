use crate::lexer::Token;
use std::{iter::Peekable, slice};

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum SyntaxNode {
    Nothing,
    Code      ( Vec<SyntaxNode>                                     ),
    FuncHeader( Vec<Token> /* options */                            ),
    FuncName  ( Token                                               ),
    Var       ( Token /* id */, Box<SyntaxNode> /* type */          ),
    Type      ( Token                                               ),
    FuncParams( Vec<SyntaxNode> /* parameters */                    ),
    FuncDef   ( Vec<SyntaxNode> /* func header fellowed by body */  ),
    Frame     ( Vec<SyntaxNode>                                     ),
    FuncArgs  ( Vec<SyntaxNode>                                     ),
    FuncCall  ( Vec<SyntaxNode> /* args */                          ),
    Expr      ( Token                                               ),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SyntaxError {
    Expected( Vec<Token>, Token ),
    ExpectedIdentifier( Token ),
    ExpectedExpr      ( Token ),
}

pub fn parse(tokens: Vec<Token>) -> Result<SyntaxNode, SyntaxError> {
    let mut tokens_iter = tokens.iter().peekable();

    let mut code: Vec<SyntaxNode> = vec![];
    let mut parse_done: bool = false;

    while !parse_done {
        let mut any_succeeded = false;

        let instruction = parse_instruction(&mut tokens_iter)?;
        if instruction != SyntaxNode::Nothing {
            code.push(instruction);
            any_succeeded = true;
        }

        if !any_succeeded {
            parse_done = true;
        }
    }

    let last_token = tokens_iter.peek();
    if last_token.is_some() {
        expects_token(Token::EOF, last_token.unwrap())?;
    }

    return Ok(SyntaxNode::Code(code))
}

fn try_parse(mut tokens: &mut Peekable<slice::Iter<'_, Token>>, 
            parse_func: &dyn Fn(&mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError>) -> Option<SyntaxNode>
{
    let mut try_tokens = tokens.clone();
    let success = parse_func(&mut try_tokens).is_ok();

    if !success {
        return None;
    }

    // must be Ok so we can unwrap without panic!
    return Some(parse_func(&mut tokens).unwrap());
}

fn expects_token(expected: Token, found: &Token) -> Result<(), SyntaxError> {
    if *found != expected {
        return Err(SyntaxError::Expected( vec![expected], found.clone() ));
    }
    return Ok(());
}

fn expects_an_identfier(found: &Token) -> Result<(), SyntaxError> {
    match found {
        Token::Identifier(..) => Ok(()),
        _                     => Err(SyntaxError::ExpectedIdentifier(found.clone())),
    }
}

fn parse_func_header(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    let open_sqr_brackets = tokens.next().unwrap_or(&Token::EOF);
    expects_token(Token::OpenSqrBrackets, open_sqr_brackets)?;

    let mut func_options: Vec<Token> = vec![];

    loop {
        let identfier = tokens.next().unwrap();
        expects_an_identfier(identfier)?;
        func_options.push(identfier.clone());

        match tokens.peek().unwrap_or(&&Token::EOF) {
            Token::CloseSqrBrackets | Token::EOF => break,
            _ => ()
        }
        
        expects_token(Token::Comma, tokens.next().unwrap_or(&Token::EOF))?;
    }

    let close_sqr_brackets = tokens.next().unwrap_or(&Token::EOF);
    expects_token(Token::CloseSqrBrackets, close_sqr_brackets)?;

    return Ok(SyntaxNode::FuncHeader(func_options));
}

fn parse_func_params(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    let open_parentheses = tokens.next().unwrap_or(&Token::EOF);
    expects_token(Token::OpenParentheses, open_parentheses)?;

    let mut func_params: Vec<SyntaxNode> = vec![];

    loop {
        match tokens.peek().unwrap_or(&&Token::EOF) {
            Token::CloseParentheses | Token::EOF => break,
            _ => (),
        }

        let var = parse_var(tokens)?;
        func_params.push(var);

        match tokens.peek().unwrap_or(&&Token::EOF) {
            Token::CloseParentheses | Token::EOF => break,
            _ => (),
        }
        
        expects_token(Token::Comma, tokens.next().unwrap_or(&Token::EOF))?;
    }

    let close_parentheses = tokens.next().unwrap_or(&Token::EOF);
    expects_token(Token::CloseParentheses, close_parentheses)?;

    return Ok(SyntaxNode::FuncParams(func_params));
}

fn parse_type(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    let var_type = tokens.next().unwrap_or(&Token::EOF);
    expects_an_identfier(var_type)?;

    return Ok(SyntaxNode::Type(var_type.clone()));
}

fn parse_var(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    let identfier = tokens.next().unwrap_or(&Token::EOF);
    expects_an_identfier(identfier)?;
    expects_token(Token::Colon, tokens.next().unwrap_or(&Token::EOF))?;

    let var_type = parse_type(tokens)?;

    return Ok(SyntaxNode::Var(identfier.clone(), Box::new(var_type)));
}

fn parse_func_def(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    let mut func_body = vec![];

    let header = try_parse(tokens, &parse_func_header);
    if header.is_some() {
        // it is garentee to unwrap successfully
        func_body.push(header.unwrap());
    }

    let func_keyword = tokens.next().unwrap_or(&Token::EOF);
    expects_token(Token::Func, func_keyword)?;

    // function name
    let func_name = tokens.next().unwrap_or(&Token::EOF);
    expects_an_identfier(func_name)?;
    func_body.push(SyntaxNode::FuncName(func_name.clone()));

    let parameters = parse_func_params(tokens)?;
    func_body.push(parameters);

    let body = parse_frame_body(tokens)?;
    func_body.push(body);

    return Ok(SyntaxNode::FuncDef(func_body));
}

fn parse_frame_body(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    expects_token(Token::OpenCurlyBrackets, tokens.next().unwrap_or(&Token::EOF))?;

    let mut body: Vec<SyntaxNode> = vec![];
    let mut parsing_done = false;

    while !parsing_done {
        let mut any_succeeded = false;

        let instruction = parse_instruction(tokens)?;
        if instruction != SyntaxNode::Nothing {
            body.push(instruction);
            any_succeeded = true;
        }

        if !any_succeeded {
            parsing_done = true;
        }
    }

    expects_token(Token::CloseCurlyBrackets, tokens.next().unwrap_or(&Token::EOF))?;

    return Ok(SyntaxNode::Frame(body))
}

fn parse_instruction(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    let mut instruction: Option<SyntaxNode> = None;

    let func_def = try_parse(tokens, &parse_func_def);
    if func_def.is_some() {
        instruction = func_def.clone();
    }

    let func_call = try_parse(tokens, &parse_func_call);
    if func_call.is_some() {
        instruction = func_call.clone();
        expects_token(Token::SemiColon, tokens.next().unwrap_or(&Token::EOF))?;
    }

    if instruction.is_none() {
        return Ok(SyntaxNode::Nothing);
    }

    return Ok(instruction.unwrap());
}

fn parse_func_call(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    let mut call: Vec<SyntaxNode> = vec![];

    let name = tokens.next().unwrap_or(&Token::EOF);
    expects_an_identfier(name)?;
    call.push(SyntaxNode::FuncName(name.clone()));

    let args = parse_func_args(tokens)?;
    call.push(args);

    return Ok(SyntaxNode::FuncCall(call));
}

fn parse_func_args(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    expects_token(Token::OpenParentheses, tokens.next().unwrap_or(&Token::EOF))?;

    // all the arguments to this call
    let mut func_args: Vec<SyntaxNode> = vec![];

    loop {
        match tokens.peek().unwrap_or(&&Token::EOF) {
            Token::CloseParentheses | Token::EOF => break,
            _ => (),
        }

        let arg = parse_expression(tokens)?;
        func_args.push(arg);

        match tokens.peek().unwrap_or(&&Token::EOF) {
            Token::CloseParentheses | Token::EOF => break,
            _ => (),
        }
        
        expects_token(Token::Comma, tokens.next().unwrap_or(&Token::EOF))?;
    }

    expects_token(Token::CloseParentheses, tokens.next().unwrap_or(&Token::EOF))?;

    return Ok(SyntaxNode::FuncArgs(func_args));
}

fn parse_expression(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    // for now an expression can be either a string literal, a char literal, an int or uint literal, a float literal or an identfier
    return parse_value(tokens);
}

fn parse_value(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, SyntaxError> {
    let func_call = try_parse(tokens, &parse_func_call);
    if func_call.is_some() {
        return Ok(func_call.unwrap());
    }

    let expr = tokens.next().unwrap_or(&Token::EOF);
    match expr {
        Token::StringLiteral(..) | Token::CharLiteral(..) | Token::Int(..) | Token::Uint(..) | Token::Float(..) | Token::Identifier(..)
        => Ok(SyntaxNode::Expr(expr.clone())),
        found => Err(SyntaxError::ExpectedExpr(found.clone())),
    }
}