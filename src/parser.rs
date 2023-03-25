use crate::{
  number::Number,
  tokenizer::{Keyword, Operator, Token},
};
use std::{collections::HashMap, iter::Peekable, ops::Add, sync::Arc};

/*
 TokenStream:
  - Keyword(If)
  - Identifier(a)
  - Operator(==)
  - Number(1)
  - EndLine
  - ScopeOpen
    - Identifier(b)
    - Operator(=)
    - Number(2)
    - EndLine
  - ScopeClose



*/
#[derive(Debug)]
pub enum ParserError {
  ExpectedToken(Token),
  UnexpectedToken(Token),
  UnknownOperator(String),
  InvalidOperator(Operator),
  UnexpectedEnd,
}
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, ParserError> {
  let mut instructions = Vec::new();
  let mut tokens = tokens.into_iter().peekable();
  loop {
    let token = tokens.next();
    if token == None {
      break;
    }
    let token = token.unwrap();

    let instruction = match token {
      Token::Comment(_) => None,
      Token::Keyword(keyword) => match keyword {
        Keyword::If => {
          let condition_tokens = parse_brackets(&mut tokens)?;
          let condition = parse_value(condition_tokens)?;
          let scope = parse_scope(&mut tokens)?;
          Some(Instruction::If {
            condition,
            instructions: scope,
          })
        }
        Keyword::Else => {
          let scope = parse_scope(&mut tokens)?;
          Some(Instruction::Else {
            instructions: scope,
          })
        }
        Keyword::While => {
          let condition_tokens = parse_brackets(&mut tokens)?;
          let condition = parse_value(condition_tokens)?;
          let scope = parse_scope(&mut tokens)?;
          Some(Instruction::While {
            condition,
            instructions: scope,
          })
        }
        Keyword::Print => {
          let value_tokens = parse_brackets(&mut tokens)?;
          let value = parse_value(value_tokens)?;
          Some(Instruction::Print { message: value })
        }
        Keyword::Input => {
          if let Some(Token::Identifier(variable)) = tokens.next() {
            Some(Instruction::Input {
              variable: variable.to_string(),
            })
          } else {
            return Err(ParserError::ExpectedToken(Token::Identifier(
              "".to_string(),
            )));
          }
        }
        Keyword::Break => Some(Instruction::Break),
      },
      Token::EndStatement => None,
      Token::Number(_)
      | Token::String(_)
      | Token::Boolean(_)
      | Token::Identifier(_)
      | Token::Operator(_)
      | Token::BracketOpen => {
        let mut value_tokens = vec![token];
        while let Some(next_token) = tokens.peek() {
          match next_token {
            Token::EndStatement => break,
            Token::Number(_)
            | Token::String(_)
            | Token::Boolean(_)
            | Token::Identifier(_)
            | Token::Operator(_)
            | Token::BracketOpen
            | Token::BracketClose => value_tokens.push(tokens.next().unwrap()),
            _ => return Err(ParserError::UnexpectedToken(next_token.clone())),
          }
        }
        let value = parse_value(value_tokens)?;
        Some(Instruction::Value { value })
      }
      _ => return Err(ParserError::UnexpectedToken(token.clone())),
    };

    if let Some(instruction) = instruction {
      instructions.push(instruction);
    }
  }
  Ok(instructions)
}

fn parse_brackets<'a>(
  token_stream: &'a mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<Token>, ParserError> {
  if let Some(Token::BracketOpen) = token_stream.peek() {
    token_stream.next();
  } else {
    return Err(ParserError::ExpectedToken(Token::BracketOpen));
  }
  parse_already_open_brackets(token_stream)
}
fn parse_already_open_brackets<'a>(
  token_stream: &'a mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<Token>, ParserError> {
  let mut tokens = Vec::new();
  let mut bracket_count = 1;
  while let Some(token) = token_stream.next() {
    match token {
      Token::BracketOpen => bracket_count += 1,
      Token::BracketClose => bracket_count -= 1,
      _ => (),
    }
    if bracket_count == 0 {
      break;
    }
    tokens.push(token);
  }
  if bracket_count != 0 {
    return Err(ParserError::UnexpectedEnd);
  }
  Ok(tokens)
}

fn parse_scope<'a>(
  token_stream: &'a mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<Instruction>, ParserError> {
  if let Some(Token::ScopeOpen) = token_stream.peek() {
    token_stream.next();
  } else {
    return Err(ParserError::ExpectedToken(Token::ScopeOpen));
  }
  parse_already_open_scope(token_stream)
}
fn parse_already_open_scope<'a>(
  token_stream: &'a mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<Instruction>, ParserError> {
  let mut tokens = Vec::new();
  let mut bracket_count = 1;
  while let Some(token) = token_stream.next() {
    match token {
      Token::ScopeOpen => bracket_count += 1,
      Token::ScopeClose => bracket_count -= 1,
      _ => (),
    }
    if bracket_count == 0 {
      break;
    }
    tokens.push(token.clone());
  }
  if bracket_count != 0 {
    return Err(ParserError::UnexpectedEnd);
  }
  parse(tokens)
}

fn parse_value(tokens: Vec<Token>) -> Result<Value, ParserError> {
  let mut tokens = tokens.into_iter().peekable();
  let mut value: Option<Value> = None;

  loop {
    let token = tokens.next();
    if token.is_none() {
      break;
    }
    let token = token.unwrap();
    if value.is_some() {
      match token {
        Token::Operator(operator) => {
          if let Some(left_hand) = value {
            match operator {
              Operator::Not | Operator::Brackets => {
                value = Some(Value::Expression(Box::new(Expression::new_not_or_bracket(
                  operator.clone(),
                  left_hand,
                ))))
              }
              _ => {
                value = Some(Value::Expression(Box::new(Expression::new(
                  operator.clone(),
                  left_hand,
                  parse_value(tokens.by_ref().collect())?,
                ))))
              }
            }
          } else {
            return Err(ParserError::ExpectedToken(token.clone()));
          }
        }
        _ => return Err(ParserError::ExpectedToken(token.clone())),
      }
    } else {
      match token {
        Token::Identifier(identifier) => value = Some(Value::Identifier(identifier.clone())),
        Token::Number(numb) => value = Some(Value::Number(numb.clone())),
        Token::String(string) => value = Some(Value::String(string.clone())),
        Token::Boolean(boolean) => value = Some(Value::Boolean(boolean.clone())),
        // 2 + (2 + 2) + 1
        Token::BracketOpen => {
          if value.is_some() {
            return Err(ParserError::ExpectedToken(token.clone()));
          }
          let bracketed_tokens = parse_already_open_brackets(&mut tokens)?;
          value = Some(parse_value(bracketed_tokens)?);
        }
        Token::BracketClose => return Err(ParserError::UnexpectedToken(token.clone())),
        Token::EndStatement => return Err(ParserError::UnexpectedEnd),
        _ => return Err(ParserError::UnexpectedToken(token.clone())),
      }
    }
  }
  if let Some(value) = value {
    Ok(value)
  } else {
    Err(ParserError::UnexpectedEnd)
  }
}
#[derive(Debug)]
pub enum Instruction {
  If {
    condition: Condition,
    instructions: Vec<Instruction>,
  },
  Else {
    instructions: Vec<Instruction>,
  },
  While {
    condition: Condition,
    instructions: Vec<Instruction>,
  },
  Scope {
    instructions: Vec<Instruction>,
  },
  Value {
    value: Value,
  },
  Break,
  Print {
    message: Value,
  },
  Input {
    variable: String,
  },
}

// TODO: Change this so it does some fancy checks like type checking for booleans
type Condition = Value;

// pub type Scope = Vec<Instruction>;
#[derive(Debug, Clone)]
pub enum Value {
  Number(Number),
  String(String),
  Boolean(bool),
  Identifier(String),
  Expression(Box<Expression>),
}
#[derive(Debug, Clone)]
pub struct Expression {
  operator: Operator,
  left: Box<Value>,
  right: Option<Box<Value>>,
}
impl Expression {
  pub fn new(operator: Operator, left: Value, right: Value) -> Self {
    if is_operator_single(&operator) {
      panic!("Invalid operator for new, use new_not_or_bracket instead");
    }
    Self {
      operator,
      left: Box::new(left),
      right: Some(Box::new(right)),
    }
  }
  pub fn new_not_or_bracket(operator: Operator, value: Value) -> Self {
    if !is_operator_single(&operator) {
      panic!("Invalid operator for new_not_or_bracket use new instead");
    }
    Self {
      operator,
      left: Box::new(value),
      right: None,
    }
  }
  pub fn get_operator(&self) -> &Operator {
    &self.operator
  }
  pub fn get_left(&self) -> &Value {
    &self.left
  }
  pub fn get_right(&self) -> Option<&Value> {
    self.right.as_ref().map(|v| &**v)
  }
}
fn is_operator_single(operator: &Operator) -> bool {
  match operator {
    Operator::Not | Operator::Brackets => true,
    _ => false,
  }
}
