use crate::lexer::Token;
use crate::error::Error;
use std::{iter::Peekable, slice};

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxNode {
    Scope(Vec<SyntaxNode>),

    FuncHeader(Vec<Token> /* options */                          ),
    FuncName  (Token                                             ),
    FuncParams(Vec<SyntaxNode> /* parameters */                  ),
    FuncDef   (Vec<SyntaxNode> /* func header fellowed by body */),
    FuncArgs  (Vec<SyntaxNode>                                   ),
    FuncCall  (Vec<SyntaxNode> /* args */                        ),
    
    ExprLiteral(Token          ),
    ExprCast   (Vec<SyntaxNode> /* type, expr */),
    Expr       (Token /* operator */, Vec<SyntaxNode>),
    
    Var (Token /* id */, Box<SyntaxNode> /* type */          ),
    Type(Token                                               ),

    ConstDecl(Vec<SyntaxNode> /* var and expr */),
    VarDecl (Vec<SyntaxNode> /* var and expr */ ),
    VarModif(Token, Token, Box<SyntaxNode> /* name, op, new value */ ),

    If  (Vec<SyntaxNode> /* expr, body, [else] */),
    Else(Box<SyntaxNode> /* body */              ),

    While          (Vec<SyntaxNode> /* condition, scope */),
    ForTraditionnal(Vec<SyntaxNode> /* initialization, condtion, iteration */), // C for loop
    ForModern      (Vec<SyntaxNode> /* vars, iterators */),

    ForVars(Vec<SyntaxNode>),
    ForIterators(Vec<Token> /* names */)
}

pub fn parse(tokens: &Vec<Token>) -> Result<SyntaxNode, Error> {
    let mut tokens_iter = tokens.iter().peekable();
    let mut code: Vec<SyntaxNode> = vec![];

    loop {
        match tokens_iter.peek().unwrap_or(&&Token::EOF) {
            Token::EOF => break,
            _ => (),
        }

        let instruction = parse_instruction(&mut tokens_iter)?;
        code.push(instruction);

        match tokens_iter.peek().unwrap_or(&&Token::EOF) {
            Token::EOF => break,
            _ => (),
        }
    }

    let last_token = tokens_iter.peek();
    if last_token.is_some() {
        expects_token(Token::EOF, last_token.unwrap())?;
    }

    return Ok(SyntaxNode::Scope(code))
}

fn try_parse(mut tokens: &mut Peekable<slice::Iter<'_, Token>>, 
            parse_func: &dyn Fn(&mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error>) -> (Result<SyntaxNode, Error>, usize)
{
    let mut try_tokens = tokens.clone();
    let parse_try = parse_func(&mut try_tokens);
    let comsumed_tokens = tokens.len() - try_tokens.len();

    if parse_try.is_err() {
        return (parse_try, comsumed_tokens);
    }

    return (parse_func(&mut tokens), usize::MAX);
}

fn parse_multiple_options(tokens: &mut Peekable<slice::Iter<'_, Token>>, 
    functions: Vec< fn(&mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error>>) -> Result<SyntaxNode, Error> 
{
    let mut instruction: Result<SyntaxNode, Error> = Err(Error::UnknownInstruction);
    let mut biggest_dst: usize = usize::MIN;

    for func in functions {
        let (parse_result, dst) = try_parse(tokens, &func);
        check_instruction_result(parse_result, &mut instruction, &dst, &mut biggest_dst);
    }

    return instruction;
}

fn expects_token(expected: Token, found: &Token) -> Result<(), Error> {
    
    if *found != expected {
        return Err(Error::Expected( vec![expected], found.clone() ));
    }
    return Ok(());
}

fn expects_tokens(expecteds: Vec<Token>, found: &Token) -> Result<(), Error> {
    if !expecteds.contains(found) {
        return Err(Error::Expected(expecteds, found.clone()));
    }
    Ok(())
}

fn expects_an_identfier(found: &Token) -> Result<(), Error> {
    match found {
        Token::Identifier(..) => Ok(()),
        _                     => Err(Error::ExpectedIdentifier(found.clone())),
    }
}

fn check_instruction_result(instruction_result: Result<SyntaxNode, Error>, instruction: &mut Result<SyntaxNode, Error>, dst: &usize, biggest_dst: &mut usize) {
    if instruction_result.is_ok() || dst > biggest_dst || *biggest_dst == 0 {
        *instruction = instruction_result;
        *biggest_dst = *dst;
    }
}

fn parse_instruction(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let possible_instructions = vec![

        parse_const_decl,
        parse_var_decl,
        parse_var_modif,
        parse_var_inc_dec,

        parse_if_else,
        parse_while_loop,
        parse_for_traditionnal,
        parse_for_modern,
        
        parse_func_call,
        parse_func_def,
    ];

    let instruction: SyntaxNode = parse_multiple_options(tokens, possible_instructions)?;

    match instruction {
        SyntaxNode::FuncDef(..) | SyntaxNode::If(..) | SyntaxNode::While(..) | SyntaxNode::ForTraditionnal(..) | SyntaxNode::ForModern(..)
            => ( /* no semi-colon after function definition or if stmt neither for a while loop */ ),
        _ => expects_token(Token::SemiColon, tokens.next().unwrap_or(&Token::EOF))?,
    };

    return Ok(instruction);
}

fn parse_func_header(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
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

fn parse_func_params(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
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

fn parse_func_def(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let mut func_body = vec![];

    let (header, _) = try_parse(tokens, &parse_func_header);
    if header.is_ok() {
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

    let body = parse_scope(tokens)?;
    func_body.push(body);

    return Ok(SyntaxNode::FuncDef(func_body));
}

fn parse_scope(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    expects_token(Token::OpenCurlyBrackets, tokens.next().unwrap_or(&Token::EOF))?;

    let mut code: Vec<SyntaxNode> = vec![];

    loop {
        match tokens.peek().unwrap_or(&&Token::EOF) {
            Token::EOF | Token::CloseCurlyBrackets => break,
            _ => (),
        }

        let instruction = parse_instruction(tokens)?;
        code.push(instruction);

        match tokens.peek().unwrap_or(&&Token::EOF) {
            Token::EOF | Token::CloseCurlyBrackets => break,
            _ => (),
        }
    }

    expects_token(Token::CloseCurlyBrackets, tokens.next().unwrap_or(&Token::EOF))?;

    return Ok(SyntaxNode::Scope(code))
}

fn parse_func_call(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let mut call: Vec<SyntaxNode> = vec![];

    let name = tokens.next().unwrap_or(&Token::EOF);
    expects_an_identfier(name)?;
    call.push(SyntaxNode::FuncName(name.clone()));

    let args = parse_func_args(tokens)?;
    call.push(args);

    return Ok(SyntaxNode::FuncCall(call));
}

fn parse_func_args(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    expects_token(Token::OpenParentheses, tokens.next().unwrap_or(&Token::EOF))?;

    // all the arguments to this call
    let mut func_args: Vec<SyntaxNode> = vec![];

    loop {
        match tokens.peek().unwrap_or(&&Token::EOF) {
            Token::CloseParentheses | Token::EOF => break,
            _ => (),
        }

        let arg = parse_expression(tokens, 0.0)?;
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

fn get_binding_power(token: &Token) -> Result<(f64, f64), Error> {
    match token {
        // comparaison
        Token::EqualTo         => Ok((0.0, 0.1)),
        Token::BiggerThan      => Ok((0.0, 0.1)),
        Token::LessThan        => Ok((0.0, 0.1)),
        Token::BiggerOrEqualTo => Ok((0.0, 0.1)),
        Token::LessOrEqualTo   => Ok((0.0, 0.1)),

        // arithmetic
        Token::Plus  => Ok((1.0, 1.1)),
        Token::Minus => Ok((1.0, 1.1)),
        Token::Times => Ok((2.0, 2.1)),
        Token::Div   => Ok((2.0, 2.1)),

        // bitwise arithmetic
        Token::BitwiseAnd => Ok((5.0, 5.1)),
        Token::BitwiseOr  => Ok((3.0, 3.1)),
        Token::BitwiseXor => Ok((4.0, 4.1)),
        _ => Err(Error::NotAnOperator(token.clone())),
    }
}

fn parse_expression(tokens: &mut Peekable<slice::Iter<'_, Token>>, min_binding_power: f64) -> Result<SyntaxNode, Error> {
    // This algorithm is based on pratt parsing
    let mut lhs = match tokens.peek() {
        Some(Token::OpenParentheses) => {
            tokens.next();
            let in_parentheses = parse_expression(tokens, 0.0)?;
            expects_token(Token::CloseParentheses, tokens.next().unwrap_or(&Token::EOF))?;

            in_parentheses
        },
        _ => parse_value(tokens)?,
    };

    loop {
        let op = *tokens.peek().unwrap_or(&&Token::EOF);

        match op {
            // end of expression
            Token::Comma | Token::SemiColon | Token::EOF | Token::CloseParentheses | Token::OpenCurlyBrackets => break,

            // expression cast
            Token::As => {
                tokens.next();
                let cast_type = parse_type(tokens)?;

                lhs = SyntaxNode::ExprCast(vec![cast_type, lhs]);
                continue;
            }
            _ => (/* we want to throw an error if op is not an operator */)
        }
        
        // verify binding power
        let (left_power, right_power) = get_binding_power(op)?;
        if left_power < min_binding_power {
            break;
        }
        
        // consume the operator 
        let operator_token = tokens.next().unwrap();
        
        // parse right hand side
        let rhs = parse_expression(tokens, right_power)?;
        lhs = SyntaxNode::Expr(operator_token.clone(), vec![lhs, rhs]);
    }

    return Ok(lhs);
}

fn parse_value(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let (func_call, _) = try_parse(tokens, &parse_func_call);
    if func_call.is_ok() {
        return Ok(func_call.unwrap());
    }

    let expr = tokens.next().unwrap_or(&Token::EOF);
    match expr {
        Token::StringLiteral(..) | Token::CharLiteral(..) | Token::Int(..) | Token::Uint(..) | Token::Float(..) | Token::Identifier(..)
        => Ok(SyntaxNode::ExprLiteral(expr.clone())),
        found => Err(Error::ExpectedExpr(found.clone())),
    }
}

fn parse_type(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let var_type = tokens.next().unwrap_or(&Token::EOF);
    expects_an_identfier(var_type)?;

    return Ok(SyntaxNode::Type(var_type.clone()));
}

fn parse_var(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let var_type = parse_type(tokens)?;
    
    let name = tokens.next().unwrap_or(&Token::EOF);
    expects_an_identfier(name)?;

    return Ok(SyntaxNode::Var(name.clone(), Box::new(var_type)));
}

fn parse_const_decl(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    expects_token(Token::Const, tokens.next().unwrap_or(&Token::EOF))?;

    let var = parse_var(tokens)?;
    expects_token(Token::Equal, tokens.next().unwrap_or(&Token::EOF))?;

    let value = parse_expression(tokens, 0.0)?;

    return Ok(SyntaxNode::ConstDecl(vec![var, value]));
}

fn parse_var_decl(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    expects_token(Token::Let, tokens.next().unwrap_or(&Token::EOF))?;

    let var = parse_var(tokens)?;
    let maybe_semi_colon = tokens.peek().unwrap_or(&&Token::EOF);

    if **maybe_semi_colon == Token::SemiColon {
        // no problemo
        return Ok(SyntaxNode::VarDecl(vec![var]));
    }

    expects_token(Token::Equal, tokens.next().unwrap_or(&Token::EOF))?;
    let value = parse_expression(tokens, 0.0)?;

    return Ok(SyntaxNode::VarDecl(vec![var, value]));
}

fn parse_var_modif(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let name = tokens.next().unwrap_or(&Token::EOF);
    expects_an_identfier(name)?;

    let operator: &Token = *tokens.peek().unwrap_or(&&Token::EOF);
    let op_power = get_binding_power(operator);
    if *operator != Token::Equal {
        if op_power.is_err()  {
            return Err(op_power.unwrap_err());
        }
        tokens.next();
    }

    expects_token(Token::Equal, tokens.next().unwrap_or(&Token::EOF))?;
    let value = parse_expression(tokens, 0.0)?;

    return Ok(SyntaxNode::VarModif(name.clone(), operator.clone(), Box::new(value)));
}

fn parse_var_inc_dec(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let name = tokens.next().unwrap_or(&Token::EOF);
    expects_an_identfier(name)?;

    let op = tokens.next().unwrap_or(&Token::EOF);
    expects_tokens(vec![Token::Inc, Token::Dec], op)?;

    let amount = match op {
        Token::Inc => SyntaxNode::ExprLiteral(Token::Int(1)),
        Token::Dec => SyntaxNode::ExprLiteral(Token::Int(-1)),

        // impossible nonetheless I need to make the compiler happy...
        _ => SyntaxNode::ExprLiteral(Token::Int(0)),
    };

    // make it add 1 or -1
    return Ok(SyntaxNode::VarModif(name.clone(), Token::Plus, Box::new(amount)));
}

fn parse_if_else(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    expects_token(Token::If, tokens.next().unwrap_or(&Token::EOF))?;
    let condition = parse_expression(tokens, 0.0)?;
    let scope = parse_scope(tokens)?;

    let (else_stmt, _) = try_parse(tokens, &parse_else);
    if else_stmt.is_ok() {
        return Ok(SyntaxNode::If(vec![condition, scope, else_stmt.unwrap()]));
    }

    return Ok(SyntaxNode::If(vec![condition, scope]));
}

fn parse_else(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    expects_token(Token::Else, tokens.next().unwrap_or(&Token::EOF))?;

    let (elif_check, _) = try_parse(tokens, &parse_if_else);
    if elif_check.is_ok() {
        // make the interpreter believe that if is in the scope of the else
        // if ... { ... } else if ... { ... } => if ... { ... } else { if ... { ... } }
        let elif_scope = SyntaxNode::Scope(vec![elif_check.unwrap()]);
        return Ok(SyntaxNode::Else(Box::new(elif_scope)));
    }

    let scope = parse_scope(tokens)?;

    return Ok(SyntaxNode::Else(Box::new(scope)));
}

fn parse_while_loop(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    expects_token(Token::While, tokens.next().unwrap_or(&Token::EOF))?;
    let condition = parse_expression(tokens, 0.0)?;
    let scope = parse_scope(tokens)?;

    return Ok(SyntaxNode::While(vec![condition, scope]));
}

fn parse_for_traditionnal(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    expects_token(Token::For, tokens.next().unwrap_or(&Token::EOF))?;

    let initialization_funcs = vec![parse_var_decl, parse_var_modif];
    let initialization = parse_multiple_options(tokens, initialization_funcs)?;

    expects_token(Token::Comma, tokens.next().unwrap_or(&Token::EOF))?;
    let condition = parse_expression(tokens, 0.0)?;
    expects_token(Token::Comma, tokens.next().unwrap_or(&Token::EOF))?;

    let iteration_funcs = vec![parse_var_modif, parse_var_inc_dec];
    let iteration = parse_multiple_options(tokens, iteration_funcs)?;

    let scope = parse_scope(tokens)?;

    return Ok(SyntaxNode::ForTraditionnal(vec![initialization, condition, iteration, scope]));
}

fn parse_for_modern(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    expects_token(Token::For, tokens.next().unwrap_or(&Token::EOF))?;

    let vars = parse_for_vars(tokens)?;
    expects_token(Token::In, tokens.next().unwrap_or(&Token::EOF))?;

    let iterators = parse_for_iterators(tokens)?;
    let scope = parse_scope(tokens)?;

    return Ok(SyntaxNode::ForModern(vec![vars, iterators, scope]));
}

fn parse_for_vars(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let mut vars: Vec<SyntaxNode> = vec![];
    
    loop {
        let var = parse_var(tokens)?;
        vars.push(var);
        
        match **tokens.peek().unwrap_or(&&Token::EOF) {
            Token::EOF | Token::In => break,
            _ => ()
        }
        
        expects_token(Token::Comma, tokens.next().unwrap_or(&Token::EOF))?;
    }

    return Ok(SyntaxNode::ForVars(vars));
}

fn parse_for_iterators(tokens: &mut Peekable<slice::Iter<'_, Token>>) -> Result<SyntaxNode, Error> {
    let mut iterators: Vec<Token> = vec![];

    loop {
        let iterator_name = tokens.next().unwrap_or(&Token::EOF);
        expects_an_identfier(iterator_name)?;

        iterators.push(iterator_name.clone());

        match **tokens.peek().unwrap_or(&&Token::EOF) {
            Token::EOF | Token::OpenCurlyBrackets => break,
            _ => ()
        }
    }

    return Ok(SyntaxNode::ForIterators(iterators));
}