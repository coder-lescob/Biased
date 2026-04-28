use std::{fmt::Debug, str::FromStr};

static BLANK_SYMBOLS: &[char] = &[' ', '\n', '\t'];

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Token {
    // symbols
    OpenParentheses,
    CloseParentheses,
    OpenSqrBrackets,
    CloseSqrBrackets,
    OpenCurlyBrackets,
    CloseCurlyBrackets,
    SemiColon,
    Comma,
    Colon,
    Hash,

    // arithmetic
    Equal,
    Plus,
    Minus,
    Times,
    Div,
    Mod,
    
    // bitwise arithmetic
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,

    // boolean arithmetic
    BoolAnd,
    BoolOr,
    BoolXor,

    // keywords
    Func,
    Let,
    
    StringLiteral(String),
    InvalidStringLiteral(String),
    CharLiteral(char),
    InvalidCharLiteral(char),

    // ids
    Identifier(String),

    // numbers
    Uint(u64),
    Int(i64),
    Float(f64),

    // errors
    Invalid,

    EOF,
}

#[derive(Debug)]
pub struct TokenParseErr;

fn is_indentifier(s: &str) -> bool {
    let mut buffer = String::new();

    for ch in s.chars() {
        buffer.push(ch);

        if is_type::<i64>(buffer.as_str()) 
        || buffer.chars().any(|c| !(c.is_alphanumeric() || c == '_')) {
            return false;
        }
    }

    return true;
} 

fn is_str_literal(s: &str) -> bool {
    let quote_at_start: bool = s.chars().nth(0) == Some('"');
    let quote_at_end: bool   = s.chars().last() == Some('"') && (s.chars().nth_back(1) != Some('\\') || s.chars().nth_back(2) == Some('\\'));
    let not_the_same_quote   = s.len() >= 2;
    return quote_at_start && quote_at_end && not_the_same_quote;
}

fn is_invalid_str_literal(s: &str) -> bool {
    let quote_at_start: bool   = s.chars().nth(0) == Some('"');
    let quote_not_at_end: bool = s.chars().last() != Some('"') || s.chars().nth_back(1) == Some('\\') && s.chars().nth_back(2) != Some('\\');
    let not_the_same_quote     = s.len() >= 2;
    return quote_at_start && quote_not_at_end && not_the_same_quote;
}

fn is_char_literal(s: &str) -> bool {
    let has_correct_len = 3 <= s.len() && s.len() <= 4;
    if !has_correct_len { 
        return false; 
    }

    let is_escape_char = s.chars().nth(1).unwrap() == '\'';
    if s.len() == 4 && !is_escape_char { 
        return false; 
    }
    
    let quote_at_start = s.chars().nth(0).unwrap() == '\'';
    let quote_at_end   = s.chars().last().unwrap() == '\'';
    
    return quote_at_start && quote_at_end;
}

fn is_invalid_char_literal(s: &str) -> bool {
    let has_correct_len = 0 < s.len() && s.len() <= 4;
    if !has_correct_len {
        return false;
    }

    let quote_at_start = s.chars().nth(0).unwrap() == '\'';
    let quote_at_end   = s.chars().last().unwrap() == '\'' && s.len() >= 2;
    return quote_at_start && !quote_at_end;
}

fn get_escape(ch: char) -> char {
    match ch {
        'n' => '\n',
        'r' => '\r',
        '0' => '\0',
        _ => ch,
    }
}

fn str_literal_process(mut s: String) -> String {
    if !is_str_literal(s.as_str()) {
        return s;
    }
    
    let mut escape_idces: Vec<usize> = vec![];
    s.remove(0);
    s.remove(s.len() - 1);

    let mut i = 0;
    while i < s.len() {
        let ch = s.chars().nth(i).unwrap();

        if ch == '\\' {
            escape_idces.push(i);
            i += 1; // skip th next char
        }
        i += 1;
    }
    
    escape_idces.reverse();

    for escape_idx in escape_idces {
        let ch = s.chars().nth(escape_idx+1).expect("what the heck ?!");
        s.remove(escape_idx);
        s.remove(escape_idx);
        s.insert(escape_idx, get_escape(ch));
    }

    return s;
}

fn char_literal_process(s: String) -> char {
    if s.len() < 2 {
        return '\0';
    }
    let is_escape_char = s.len() == 4;

    let ch = s.chars().nth_back(1);

    if ch == None {
        return '\0';
    }

    if is_escape_char {
        return get_escape(ch.unwrap());
    }

    return ch.unwrap();
}

impl FromStr for Token {
    type Err = TokenParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        // string literals are able to have blank symbols in them
        // string literals and character literal
        if is_invalid_str_literal(s)    { return Ok(Token::InvalidStringLiteral(s.to_string()))                       }
        if is_str_literal(s)            { return Ok(Token::StringLiteral       (str_literal_process(s.to_string())))  }
        if is_char_literal(s)           { return Ok(Token::CharLiteral         (char_literal_process(s.to_string()))) }
        if is_invalid_char_literal(s)   { return Ok(Token::InvalidCharLiteral  (char_literal_process(s.to_string()))) }

        if s.len() == 0 || s.chars().any(|c| BLANK_SYMBOLS.contains(&c)) {
            return Err(TokenParseErr);
        }

        // number literals
        if is_type::<u64>(s) { return Ok(Token::Uint (s.parse::<u64>().unwrap())); }
        if is_type::<i64>(s) { return Ok(Token::Int  (s.parse::<i64>().unwrap())); }
        if is_type::<f64>(s) { return Ok(Token::Float(s.parse::<f64>().unwrap())); }

        match s {
            "(" => return Ok(Token::OpenParentheses),
            ")" => return Ok(Token::CloseParentheses),
            "[" => return Ok(Token::OpenSqrBrackets),
            "]" => return Ok(Token::CloseSqrBrackets),
            "{" => return Ok(Token::OpenCurlyBrackets),
            "}" => return Ok(Token::CloseCurlyBrackets),
            "#" => return Ok(Token::Hash),
            ";" => return Ok(Token::SemiColon),
            "," => return Ok(Token::Comma),
            ":" => return Ok(Token::Colon),

            // arithmetic
            "=" => return Ok(Token::Equal),
            "+" => return Ok(Token::Plus),
            "-" => return Ok(Token::Minus),
            "*" => return Ok(Token::Times),
            "/" => return Ok(Token::Div),

            // bitwise arithmetic
            "&" => return Ok(Token::BitwiseAnd),
            "|" => return Ok(Token::BitwiseOr ),
            "^" => return Ok(Token::BitwiseXor),
            "!" => return Ok(Token::BitwiseNot),

            // boolean arithmetic
            "&&" => return Ok(Token::BoolAnd),
            "||" => return Ok(Token::BoolOr ),
            "##" => return Ok(Token::BoolXor),

            "func" => return Ok(Token::Func),
            "let"  => return Ok(Token::Let),
            _ => { /* not a keyword or a known symbol */ },
        }

        if is_indentifier(s) { return Ok(Token::Identifier(s.to_string())); }

        Err(TokenParseErr)
    }
}

#[inline]
fn is_type<T>(value: &str) -> bool
where T: FromStr 
{
    return !value.parse::<T>().is_err();
}

pub struct TokenizationErr {
    token: String,
}

fn get_num_unknowns(s: &str) -> u32 {
    let mut num = 0;

    for ch in s.chars() {
        if is_type::<Token>(ch.to_string().as_str()) {
            break;
        }
        
        num += 1;
    }

    return num;
}

impl Debug for TokenizationErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_unknowns_letters = get_num_unknowns(self.token.as_str()) as usize;
        let underline = vec!["^"; num_unknowns_letters].concat();
        let bad_letters: String = self.token
            .get(0..num_unknowns_letters)
            .unwrap_or("")
            .chars()
            .flat_map(|c| ['\'', c, '\'', ','])
            .collect::<String>()
            .trim_end_matches(',')
            .to_string();

        f.write_str(format!("Unknown token kind\n{}\n\x1b[38;5;196m{}unknown symbol(s) {}\x1b[0m", self.token, underline, bad_letters).as_str())?;
        Ok(())
    }
}

pub fn tokenize(code: &str) -> Result<Vec<Token>, TokenizationErr> {

    // create the token vector
    let mut tokens: Vec<Token> = Vec::new();

    // create a buffer
    let mut buffer: String = String::new();

    for symbol in code.chars() {

        // create an hypothetic buffer containing the new character
        let mut next_buffer = buffer.clone();
        next_buffer.push(symbol);

        let is_next_buffer_invalid = is_type::<Token>(&buffer) 
                                 && !is_type::<Token>(&next_buffer);
        let is_string_literal = buffer.parse::<Token>().unwrap_or(Token::Invalid) == Token::StringLiteral(str_literal_process(buffer.clone()));
        let next_string_literal_invalid = next_buffer.parse::<Token>().unwrap_or(Token::Invalid) == Token::InvalidStringLiteral(next_buffer.clone()) &&
                                next_buffer.chars().nth_back(1) != Some('\\');

        let is_char_literal = buffer.parse::<Token>().unwrap_or(Token::Invalid) == Token::CharLiteral(char_literal_process(buffer.clone()));
        let next_chr_literal_invalid = next_buffer.parse::<Token>().unwrap_or(Token::Invalid) == Token::InvalidCharLiteral(char_literal_process(next_buffer.clone()));

        if is_next_buffer_invalid 
        || is_string_literal && next_string_literal_invalid 
        || is_char_literal && next_chr_literal_invalid 
        {
            // token found
            let token = buffer.parse::<Token>().unwrap();
            tokens.push(token);

            // reset the token buffer
            buffer.clear();
        }
        
        if BLANK_SYMBOLS.contains(&symbol) 
        && (!next_string_literal_invalid || is_string_literal) 
        && (!next_chr_literal_invalid || is_char_literal) 
        {
            continue;
        }

        buffer.push(symbol);
    }

    if is_type::<Token>(&buffer) {
        // token found
        let token = buffer.parse::<Token>().unwrap();
        tokens.push(token);

        // reset the token buffer
        buffer.clear();
    }
    else {
        return Err(TokenizationErr {token: buffer});
    }

    tokens.push(Token::EOF);

    return Ok(tokens);
}

#[allow(dead_code)]
fn print_diff(tokens: &Vec<Token>, expecteds: &Vec<Token>) {
    for token in tokens {
        if !expecteds.contains(token) {
            println!("unexpected token: {:?}", token);
        }
    }

    for token in expecteds {
        if !tokens.contains(token) {
            println!("token: {:?} not found", token);
        }
    }
}

#[test]
pub fn hellow_test() {
    let hellow_program = std::fs::read_to_string("test/hellow.bias").expect("there must be code");
    let tokens = tokenize(hellow_program.as_str()).expect("Lexical error");
    let expecteds: Vec<Token> = vec!
    [
        Token::OpenSqrBrackets,
        Token::Identifier(String::from("entry_point")),
        Token::CloseSqrBrackets,
        Token::Func,
        Token::Identifier(String::from("main")),
        Token::OpenParentheses,
        Token::CloseParentheses,
        Token::OpenCurlyBrackets,
        Token::Identifier(String::from("println")),
        Token::OpenParentheses,
        Token::StringLiteral(String::from("hello, world !")),
        Token::CloseParentheses,
        Token::SemiColon,
        Token::CloseCurlyBrackets,
        Token::EOF,
    ];

    print_diff(&tokens, &expecteds);

    assert!(tokens == expecteds);
}