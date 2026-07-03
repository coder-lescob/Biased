use std::collections::HashMap;

use crate::error::Error;
use crate::parser::SyntaxNode;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    String(String),
    Char(char),
}

impl Value {
    fn add(self, rhs: Self) -> Result<Value, Error> {

        match (self, rhs) {
            (Self::Int(val_a)  , Self::Int(val_b)  ) => Ok(Self::Int(val_a + val_b)),
            (Self::Int(val_a)  , Self::Uint(val_b))  => Ok(Self::Int(val_a + val_b as i64)),
            (Self::Uint(val_a)  , Self::Int(val_b))  => Ok(Self::Int(val_a as i64 + val_b)),
            (Self::Uint(val_a) , Self::Uint(val_b) ) => Ok(Self::Uint (val_a + val_b)),
            (Self::Float(val_a), Self::Float(val_b)) => Ok(Self::Float(val_a + val_b)),

            (Self::String(str_a), Self::String(str_b)) => Ok(Self::String(format!("{str_a}{str_b}"))),
            (lhs, rhs) => Err(Error::CannotPerform(Token::Plus, lhs, rhs)),
        }
    }

    fn sub(self, rhs: Self) -> Result<Value, Error> {

        match (self, rhs) {
            (Self::Int(val_a)  , Self::Int(val_b)  ) => Ok(Self::Int  (val_a - val_b)),
            (Self::Int(val_a)  , Self::Uint(val_b) ) => Ok(Self::Int  (val_a - val_b as i64)),
            (Self::Uint(val_a) , Self::Int(val_b)  ) => Ok(Self::Int  (val_a as i64 - val_b)),
            (Self::Uint(val_a) , Self::Uint(val_b) ) => Ok(Self::Uint (val_a - val_b)),
            (Self::Float(val_a), Self::Float(val_b)) => Ok(Self::Float(val_a - val_b)),

            (lhs, rhs) => Err(Error::CannotPerform(Token::Minus, lhs, rhs)),
        }
    }

    fn mul(self, rhs: Self) -> Result<Value, Error> {

        match (self, rhs) {
            (Self::Int(val_a)  , Self::Int(val_b)  ) => Ok(Self::Int  (val_a * val_b)),
            (Self::Int(val_a)  , Self::Uint(val_b) ) => Ok(Self::Int  (val_a * val_b as i64)),
            (Self::Uint(val_a) , Self::Int(val_b)  ) => Ok(Self::Int  (val_a as i64 * val_b)),
            (Self::Uint(val_a) , Self::Uint(val_b) ) => Ok(Self::Uint (val_a * val_b)),
            (Self::Float(val_a), Self::Float(val_b)) => Ok(Self::Float(val_a * val_b)),

            (lhs, rhs) => Err(Error::CannotPerform(Token::Times, lhs, rhs)),
        }
    }

    fn div(self, rhs: Self) -> Result<Value, Error> {

        match (self, rhs) {
            (Self::Int(val_a)  , Self::Int(val_b)  ) => Ok(Self::Int  (val_a / val_b)),
            (Self::Int(val_a)  , Self::Uint(val_b) ) => Ok(Self::Int  (val_a / val_b as i64)),
            (Self::Uint(val_a) , Self::Int(val_b)  ) => Ok(Self::Int  (val_a as i64 / val_b)),
            (Self::Uint(val_a) , Self::Uint(val_b) ) => Ok(Self::Uint (val_a / val_b)),
            (Self::Float(val_a), Self::Float(val_b)) => Ok(Self::Float(val_a / val_b)),

            (lhs, rhs) => Err(Error::CannotPerform(Token::Div, lhs, rhs)),
        }
    }

    fn eq(self, rhs: Self) -> Result<Value, Error> {

        match (self, rhs) {
            (Self::Int(val_a)  , Self::Int(val_b)  ) => Ok(Self::Bool(val_a == val_b)),
            (Self::Int(val_a)  , Self::Uint(val_b) ) => Ok(Self::Bool(val_a == val_b as i64)),
            (Self::Uint(val_a) , Self::Int(val_b)  ) => Ok(Self::Bool(val_a as i64 == val_b)),
            (Self::Uint(val_a) , Self::Uint(val_b) ) => Ok(Self::Bool(val_a == val_b)),
            (Self::Float(val_a), Self::Float(val_b)) => Ok(Self::Bool(val_a == val_b)),

            (Self::String(str_a), Self::String(str_b)) => Ok(Self::Bool(str_a == str_b)),
            (lhs, rhs) => Err(Error::CannotPerform(Token::EqualTo, lhs, rhs)),
        }
    }

    fn bigger(self, rhs: Self) -> Result<Value, Error> {

        match (self, rhs) {
            (Self::Int(val_a)  , Self::Int(val_b)  ) => Ok(Self::Bool(val_a > val_b)),
            (Self::Int(val_a)  , Self::Uint(val_b) ) => Ok(Self::Bool(val_a > val_b as i64)),
            (Self::Uint(val_a) , Self::Int(val_b)  ) => Ok(Self::Bool(val_a as i64 > val_b)),
            (Self::Uint(val_a) , Self::Uint(val_b) ) => Ok(Self::Bool(val_a > val_b)),
            (Self::Float(val_a), Self::Float(val_b)) => Ok(Self::Bool(val_a > val_b)),

            (lhs, rhs) => Err(Error::CannotPerform(Token::BiggerThan, lhs, rhs)),
        }
    }

    fn less(self, rhs: Self) -> Result<Value, Error> {

        match (self, rhs) {
            (Self::Int(val_a)  , Self::Int(val_b)  ) => Ok(Self::Bool(val_a < val_b)),
            (Self::Int(val_a)  , Self::Uint(val_b) ) => Ok(Self::Bool(val_a < val_b as i64)),
            (Self::Uint(val_a) , Self::Int(val_b)  ) => Ok(Self::Bool((val_a as i64) < val_b)),
            (Self::Uint(val_a) , Self::Uint(val_b) ) => Ok(Self::Bool(val_a < val_b)),
            (Self::Float(val_a), Self::Float(val_b)) => Ok(Self::Bool(val_a < val_b)),

            (lhs, rhs) => Err(Error::CannotPerform(Token::LessThan, lhs, rhs)),
        }
    }

    fn be(self, rhs: Self) -> Result<Value, Error> {

        match (self, rhs) {
            (Self::Int(val_a)  , Self::Int(val_b)  ) => Ok(Self::Bool(val_a >= val_b)),
            (Self::Int(val_a)  , Self::Uint(val_b) ) => Ok(Self::Bool(val_a >= val_b as i64)),
            (Self::Uint(val_a) , Self::Int(val_b)  ) => Ok(Self::Bool((val_a as i64) >= val_b)),
            (Self::Uint(val_a) , Self::Uint(val_b) ) => Ok(Self::Bool(val_a >= val_b)),
            (Self::Float(val_a), Self::Float(val_b)) => Ok(Self::Bool(val_a >= val_b)),

            (lhs, rhs) => Err(Error::CannotPerform(Token::LessThan, lhs, rhs)),
        }
    }

    fn le(self, rhs: Self) -> Result<Value, Error> {

        match (self, rhs) {
            (Self::Int(val_a)  , Self::Int(val_b)  ) => Ok(Self::Bool(val_a <= val_b)),
            (Self::Int(val_a)  , Self::Uint(val_b) ) => Ok(Self::Bool(val_a <= val_b as i64)),
            (Self::Uint(val_a) , Self::Int(val_b)  ) => Ok(Self::Bool((val_a as i64) <= val_b)),
            (Self::Uint(val_a) , Self::Uint(val_b) ) => Ok(Self::Bool(val_a <= val_b)),
            (Self::Float(val_a), Self::Float(val_b)) => Ok(Self::Bool(val_a <= val_b)),

            (lhs, rhs) => Err(Error::CannotPerform(Token::LessThan, lhs, rhs)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    return_type: SyntaxNode,
    parameters: Vec<SyntaxNode>,
    scope: Option<SyntaxNode>,
    callback: Option<fn(HashMap<String, Value>) -> Option<Value>>,
    
}

pub struct Interpreter {
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, Function>,
}

impl Interpreter {
    fn println_callback(vars: HashMap<String, Value>) -> Option<Value> {
        match vars["str"].clone() {
            Value::String(str_val) => println!("{}", str_val),
            _bad_value => (),
        }
        
        None
    }

    pub fn new() -> Self {
        let builtin_functions: HashMap<String, Function> = HashMap::from([
            ("println".to_string(), Function {name: "println".to_string(), return_type: SyntaxNode::Type(Token::Identifier("void".to_string())), parameters: vec![SyntaxNode::Var(Token::Identifier("str".to_string()), Box::new(SyntaxNode::Type(Token::Identifier("string".to_string()))))], scope: None, callback: Some(Self::println_callback)})
        ]);

        Self {
            variables: HashMap::new(),
            functions: builtin_functions,
        }
    }

    pub fn interprete(&mut self, syntax: SyntaxNode) -> Result<Option<Option<Value>>, Error> {
        match syntax.clone() {
            SyntaxNode::Scope(scope) => {
                let outside_vars = self.variables.clone();
                let outside_func = self.functions.clone();
                
                for instruction in scope {
                    let return_value = self.interprete(instruction)?;
                    if return_value.is_some() {
                        // must have been a return stmt
                        // for now debugging
                        //println!("{:#?}", self.variables);
                        //println!("{:#?}", self.functions);

                        self.variables.retain(|x, _| outside_vars.contains_key(x));
                        self.functions.retain(|x, _| outside_func.contains_key(x));
                        return Ok(return_value);
                    }
                }

                // for now debugging
                //println!("{:#?}", self.variables);
                //println!("{:#?}", self.functions);

                self.variables.retain(|x, _| outside_vars.contains_key(x));
                self.functions.retain(|x, _| outside_func.contains_key(x));
            }
            SyntaxNode::VarDecl(decl) => {
                let var = decl[0].clone();
                let (name, _type) = match var {
                    SyntaxNode::Var(Token::Identifier(name), _type) => Ok((name, _type)),
                    var => return Err(Error::NotAVar(var)),
                }?;

                if self.variables.contains_key(&name) {
                    return Err(Error::VariableAlreadyExists(name));
                }

                let value = self.cast_expression(*_type, decl[1].clone())?;

                self.variables.insert(name, value);
            }
            SyntaxNode::VarModif(Token::Identifier(name), op, expr) => {
                // if the variable does not exist error!
                if !self.variables.contains_key(&name) {
                    return Err(Error::UnknownVariable(name))
                }

                // cannot fail due to the check above
                let value = &self.variables[&name];

                // update the value
                let new_value = self.eval_partial_operation(&op, value.clone(), &expr)?;
                self.variables.insert(name, new_value);
            }
            SyntaxNode::FuncDef(def) => {
                // register the function
                if def.len() != 4 && def.len() != 5 {
                    println!("{:#?}", def);
                    return Err(Error::FuncRegisterFailed);
                }

                let name_node = def[0].clone();
                let name = match name_node.clone() {
                    SyntaxNode::FuncName(Token::Identifier(name)) => Ok(name),
                    _ => Err(Error::FuncRegisterFailed),
                }?;

                let parameters = match def[1].clone() {
                    SyntaxNode::FuncParams(vars) => Ok(vars),
                    _ => Err(Error::FuncRegisterFailed),
                }?;

                let scope = def[2].clone();
                let return_type = def[3].clone();

                let function = Function {
                    name: name,
                    return_type: return_type,
                    parameters: parameters,
                    scope: Some(scope),
                    callback: None
                };

                self.functions.insert(function.name.clone(), function);

                if def.len() == 5 {
                    let header = def[4].clone();

                    match header.clone() {
                        SyntaxNode::FuncHeader(options) => {
                            for option in options {
                                match option {
                                    Token::Identifier(option) => {
                                        match option.as_str() {
                                            "entry_point" => {
                                                // then run the function immedialy
                                                return Ok(Some(self.call_function(SyntaxNode::FuncCall(vec![name_node, SyntaxNode::FuncArgs(vec![])]))?));
                                            }
                                            _ => return Err(Error::NotAnFuncOption(Token::Identifier(option)))
                                        }
                                    }
                                    option => return Err(Error::NotAnFuncOption(option))
                                }
                            }
                        }
                        _ => return Err(Error::NotAFuncHeader(header))
                    }
                }
            }
            SyntaxNode::FuncCall(_) => {
                // forward the function call
                self.call_function(syntax)?;
            }
            SyntaxNode::If(if_stmt) => {
                let condition_expr = if_stmt[0].clone();
                let condition      = self.eval_expression(&condition_expr)?;

                match condition {
                    Value::Bool(true) => {
                        // execute the if branch
                        let if_scope = if_stmt[1].clone();
                        return Ok(self.interprete(if_scope)?);
                    }
                    Value::Bool(false) => {
                        // try to execute the else branch
                        if if_stmt.len() <= 2 {
                            return Ok(None);
                        }

                        // get the else branch
                        let else_branch = if_stmt[2].clone();
                        match else_branch {
                            SyntaxNode::Else(body) => {
                                // run the scope
                                return Ok(self.interprete(*body)?);
                            }
                            not_an_else_branch => return Err(Error::ExpectedElseBranch(not_an_else_branch)),
                        }
                    }
                    got => return Err(Error::ExpectedBool(got)),
                }
            }
            SyntaxNode::Return(ret_expr) => {
                if ret_expr.is_some() {
                    let return_value = self.eval_expression(&ret_expr.unwrap())?;
                    return Ok(Some(Some(return_value)));
                }
                return Ok(Some(None));
            }
            _ => todo!("implement {:#?}", syntax),
        }

        Ok(None)
    }

    fn call_function(&mut self, function: SyntaxNode) -> Result<Option<Value>, Error> {
        match function {
            SyntaxNode::FuncCall(call_args) => {
                let func_name = match call_args[0].clone() {
                    SyntaxNode::FuncName(Token::Identifier(name)) => Ok(name),
                    got => Err(Error::ExpectedFuncName(got)),
                }?;

                if !self.functions.contains_key(&func_name) {
                    return Err(Error::UnknownFunction(func_name));
                }

                let function = self.functions[&func_name].clone();
                let mut arguments: HashMap<String, Value> = HashMap::new();

                let args = match call_args[1].clone() {
                    SyntaxNode::FuncArgs(args) => Ok(args),
                    args  => Err(Error::ExpectedFuncArgs(args)),
                }?;

                for (arg, param) in args.iter().zip(function.parameters.iter()) {
                    let (param_name, param_type) = match param {
                        SyntaxNode::Var(Token::Identifier(param_name), param_type) => Ok((param_name.clone(), *param_type.clone())),
                        otherwise => Err(Error::ExpectedFuncParam(otherwise.clone())),
                    }?;
                    arguments.insert(param_name, self.cast_expression(param_type, arg.clone())?);
                }

                let outside_vars = self.variables.clone();
                self.variables.clear();

                // push arguments to the vars
                for (name, value) in arguments.clone() {
                    if self.variables.contains_key(&name) {
                        return Err(Error::VariableAlreadyExists(name));
                    }
                    self.variables.insert(name, value);
                }

                // a scope returns a value only on a return stmt
                let return_value = if function.scope.is_none() {
                    // must be an outside function so let's call it
                    if function.callback.is_none() {
                        return Err(Error::MissingFunction(function));
                    }
                    Some(function.callback.unwrap()(self.variables.clone()))
                }
                else {
                    self.interprete(function.scope.unwrap())?
                };
                
                // restore variables like at the start
                self.variables.extend(outside_vars);

                if return_value.is_none() {
                    if function.return_type != SyntaxNode::Type(Token::Identifier(String::from("void"))) {
                        return Err(Error::MissingReturn(func_name));
                    }
                    return Ok(None);
                }

                if !Self::has_type(function.return_type, return_value.clone().unwrap()) {
                    return Err(Error::ReturnIncompatibleType(return_value.clone().unwrap().unwrap()));
                }

                // return the input value
                Ok(return_value.unwrap())
            }
            // should never be called
            err => Err(Error::NotAFunction(err)),
        }
    }

    fn eval_expression(&mut self, expression: &SyntaxNode) -> Result<Value, Error> {
        match expression.clone() {
            SyntaxNode::ExprLiteral(token) => Ok(self.eval_expression_literal(token)?),
            SyntaxNode::Expr(op, operands) => Ok(self.eval_operation(op, &operands)?),
            SyntaxNode::ExprCast(cast) => Ok(self.cast_expression(cast[0].clone(), cast[1].clone())?),
            SyntaxNode::FuncCall(_) => {
                let return_value = self.call_function(expression.clone())?;
                if return_value.is_some() {
                    Ok(return_value.unwrap())
                }
                else {
                    Err(Error::VoidInExpression(expression.clone()))
                }
            }
            expr => Err(Error::NotAnExpression(expr)),
        }
    }

    fn eval_operation(&mut self, op: Token, operands: &Vec<SyntaxNode>) -> Result<Value, Error> {
        if operands.len() > 2 {
            return Err(Error::TooMuchOperands(operands.clone()));
        }
        if operands.len() < 2 {
            return Err(Error::NotEnoughOperand(operands.clone()));
        }

        let lhs = self.eval_expression(&operands[0].clone())?;
        let rhs = self.eval_expression(&operands[1].clone())?;

        match op {
            Token::Plus => Ok(Value::add(lhs, rhs)?),
            Token::Minus => Ok(Value::sub(lhs, rhs)?),
            Token::Times => Ok(Value::mul(lhs, rhs)?),
            Token::Div => Ok(Value::div(lhs, rhs)?),
            Token::EqualTo => Ok(Value::eq(lhs, rhs)?),
            Token::BiggerThan => Ok(Value::bigger(lhs, rhs)?),
            Token::LessThan   => Ok(Value::less(lhs, rhs)?),
            Token::BiggerOrEqualTo => Ok(Value::be(lhs, rhs)?),
            Token::LessOrEqualTo => Ok(Value::le(lhs, rhs)?),
            _ => Err(Error::NotAnOperator(op)),
        }
    }

    fn eval_partial_operation(&mut self, op: &Token, lhs: Value, rhs_expr: &SyntaxNode) -> Result<Value, Error> {
        let rhs = self.eval_expression(rhs_expr)?;

        match op {
            Token::Plus => Ok(Value::add(lhs, rhs)?),
            Token::Minus => Ok(Value::sub(lhs, rhs)?),
            Token::Times => Ok(Value::mul(lhs, rhs)?),
            Token::Div => Ok(Value::div(lhs, rhs)?),
            Token::EqualTo => Ok(Value::eq(lhs, rhs)?),
            Token::BiggerThan => Ok(Value::bigger(lhs, rhs)?),
            Token::LessThan   => Ok(Value::less(lhs, rhs)?),
            Token::BiggerOrEqualTo => Ok(Value::be(lhs, rhs)?),
            Token::LessOrEqualTo => Ok(Value::le(lhs, rhs)?),
            op => Err(Error::NotAnOperator(op.clone())),
        }
    }

    fn eval_expression_literal(&mut self, token: Token) -> Result<Value, Error> {
        match token {
            Token::Identifier(var_name) => {
                if !self.variables.contains_key(&var_name) {
                    Err(Error::UnknownVariable(var_name))
                }
                else {
                    Ok(self.variables[&var_name].clone())
                }
            }
            Token::StringLiteral(s) => Ok(Value::String(s)),
            Token::CharLiteral(ch)  => Ok(Value::Char(ch)),
            Token::Int(val) => Ok(Value::Int(val)),
            Token::Uint(val) => Ok(Value::Uint(val)),
            Token::Float(val) => Ok(Value::Float(val)),
            _ => Err(Error::NotExpressionLiteral(token)),
        }
    }

    fn cast_expression(&mut self, _type: SyntaxNode, expression: SyntaxNode) -> Result<Value, Error> {
        let expression_ret = self.eval_expression(&expression)?;

        match (expression_ret.clone(), _type.clone()) {
            (Value::Int(a), SyntaxNode::Type(Token::Identifier(t))) => match t.as_str() {
                "string" => Ok(Value::String(a.to_string())),
                "int"   => Ok(expression_ret),
                "uint"  => Ok(Value::Uint(a as u64)),
                "float" => Ok(Value::Float(a as f64)),
                _ => Err(Error::CannotCast(expression, _type)),
            },
            (Value::Uint(a), SyntaxNode::Type(Token::Identifier(t))) => match t.as_str() {
                "int"   => Ok(Value::Int(a as i64)),
                "uint"  => Ok(expression_ret),
                "float" => Ok(Value::Float(a as f64)),
                _ => Err(Error::CannotCast(expression, _type)),
            },
            (Value::Float(a), SyntaxNode::Type(Token::Identifier(t))) => match t.as_str() {
                "int"   => Ok(Value::Int(a as i64)),
                "uint"  => Ok(Value::Uint(a as u64)),
                "float" => Ok(expression_ret),
                _ => Err(Error::CannotCast(expression, _type)),
            },
            (Value::String(str_val), SyntaxNode::Type(Token::Identifier(t))) => match t.as_str() {
                "string" => Ok(Value::String(str_val)),
                _ => Err(Error::CannotCast(expression, _type)),
            }
            (_, expr_type) => Err(Error::CannotCast(expression, expr_type))
        }
    }

    fn has_type(_type: SyntaxNode, value: Option<Value>) -> bool {
        if value.is_none() {
            return _type == SyntaxNode::Type(Token::Identifier(String::from("void")));
        }

        match (_type, value.unwrap()) {
            (SyntaxNode::Type(Token::Identifier(_type)), Value::Bool(_)) => _type == "bool",
            (SyntaxNode::Type(Token::Identifier(_type)), Value::Uint(_)) => _type == "uint",
            (SyntaxNode::Type(Token::Identifier(_type)), Value::Int (_)) => _type == "int",
            (SyntaxNode::Type(Token::Identifier(_type)), Value::Float(_)) => _type == "float",
            (SyntaxNode::Type(Token::Identifier(_type)), Value::Char(_)) => _type == "char",
            (SyntaxNode::Type(Token::Identifier(_type)), Value::String(_)) => _type == "string",
            _ => false,
        }
    }
            
}