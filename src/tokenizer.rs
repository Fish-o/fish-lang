use std::ops::Add;

use crate::number::Number;

#[derive(Debug)]
pub enum TokenizerError {
  UnknownOperator(String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
  let mut tokens = Vec::new();
  let mut chars = input.chars().peekable();
  while let Some(c) = chars.next() {
    match c {
      ' ' | '\t' | '\r' | '\n' => continue,
      ';' => {
        tokens.push(Token::EndStatement);
      }
      '0'..='9' => {
        let mut number = String::new();
        number.push(c);
        while let Some(&('0'..='9') | &'.' | &',' | &'_') = chars.peek() {
          number.push(chars.next().unwrap());
        }
        let number = number.replace("_", "");
        let number = number.replace(",", "");

        // TODO: Improved number parsing. You know this had to be done.
        if number.contains('.') {
          tokens.push(Token::Number(Number::Float(number.parse().unwrap())));
        } else {
          tokens.push(Token::Number(Number::Integer(number.parse().unwrap())));
        }
      }
      'a'..='z' | 'A'..='Z' | '_' => {
        let mut identifier = String::new();
        identifier.push(c);
        while let Some(&('a'..='z' | 'A'..='Z' | '0'..='9' | '_')) = chars.peek() {
          identifier.push(chars.next().unwrap());
        }
        match identifier.as_str() {
          "if" => tokens.push(Token::Keyword(Keyword::If)),
          "else" => tokens.push(Token::Keyword(Keyword::Else)),
          "while" => tokens.push(Token::Keyword(Keyword::While)),
          "print" => tokens.push(Token::Keyword(Keyword::Print)),
          "input" => tokens.push(Token::Keyword(Keyword::Input)),
          "break" => tokens.push(Token::Keyword(Keyword::Break)),

          "true" => tokens.push(Token::Boolean(true)),
          "false" => tokens.push(Token::Boolean(false)),
          _ => tokens.push(Token::Identifier(identifier)),
        }
      }
      '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '^' => {
        let mut operator = String::new();
        operator.push(c);
        while let Some(&('=' | '=')) = chars.peek() {
          operator.push(chars.next().unwrap());
        }
        let operator = match operator.as_str() {
          "+" => Ok(Operator::Add),
          "-" => Ok(Operator::Subtract),
          "*" => Ok(Operator::Multiply),
          "/" => Ok(Operator::Divide),
          "%" => Ok(Operator::Modulo),
          "^" => Ok(Operator::Exponent),

          "==" => Ok(Operator::Equal),
          "!=" => Ok(Operator::NotEqual),
          "<" => Ok(Operator::LessThan),
          ">" => Ok(Operator::GreaterThan),
          "<=" => Ok(Operator::LessThanOrEqual),
          ">=" => Ok(Operator::GreaterThanOrEqual),

          "&&" => Ok(Operator::And),
          "||" => Ok(Operator::Or),
          "!" => Ok(Operator::Not),

          "=" => Ok(Operator::Assign),
          "+=" => Ok(Operator::AddAssign),
          "-=" => Ok(Operator::SubtractAssign),
          "*=" => Ok(Operator::MultiplyAssign),
          "/=" => Ok(Operator::DivideAssign),
          "%=" => Ok(Operator::ModuloAssign),

          _ => Err(TokenizerError::UnknownOperator(operator)),
        }?;
        tokens.push(Token::Operator(operator));
      }
      '&' | '|' => {
        let mut operator = String::new();
        operator.push(c);
        if let Some(&c2) = chars.peek() {
          if c2 == c {
            operator.push(chars.next().unwrap());
          }
        }
        // TODO: Throw error if operator is not valid
      }
      '"' => {
        let mut string = String::new();
        while let Some(&c) = chars.peek() {
          if c == '"' {
            chars.next();
            break;
          }
          string.push(chars.next().unwrap());
        }
        tokens.push(Token::String(string));
      }
      '#' => {
        let mut comment = String::new();
        while let Some(&c) = chars.peek() {
          if c == '#' {
            chars.next();
            break;
          }
          comment.push(chars.next().unwrap());
        }
        tokens.push(Token::Comment(comment));
      }
      '{' => tokens.push(Token::ScopeOpen),
      '}' => tokens.push(Token::ScopeClose),
      '(' => tokens.push(Token::BracketOpen),
      ')' => tokens.push(Token::BracketClose),
      _ => panic!("Unexpected character: {}", c),
    }
  }
  Ok(tokens)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  EndStatement,       // ;
  Identifier(String), // [a-zA-Z_][a-zA-Z0-9_]*
  Number(Number),     // [0-9]+
  String(String),     // ".*"
  Operator(Operator), // + - * / % = == != < > <= >=
  Keyword(Keyword),   // if else while
  Comment(String),    // #/.*#
  ScopeOpen,          // {
  ScopeClose,         // }
  BracketOpen,        // (
  BracketClose,       // )
  Boolean(bool),      // true false
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
  If,
  Else,
  While,
  Print,
  Input,
  Break,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
  Add,
  Subtract,

  Multiply,
  Divide,
  Modulo,
  Exponent,

  Equal,
  NotEqual,
  LessThan,
  GreaterThan,
  LessThanOrEqual,
  GreaterThanOrEqual,

  And,
  Or,
  Not,
  // TODO: Fix this jank, there are bracket tokens and a bracket operator
  Brackets,

  Assign,
  AddAssign,
  SubtractAssign,
  MultiplyAssign,
  DivideAssign,
  ModuloAssign,
}
