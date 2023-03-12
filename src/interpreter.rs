use std::{cell::RefCell, collections::HashMap};

use crate::{
  number::Number,
  parser::{Expression, Instruction, Value},
  tokenizer::Operator,
};
pub fn interpret(instructions: Vec<Instruction>) -> Result<(), InterpreterError> {
  let mut stack_frame = StackFrame::empty();
  stack_frame.execute(&instructions)
}
struct StackFrame<'a> {
  variables: HashMap<String, Data>,
  outer: Option<&'a StackFrame<'a>>,
}

#[derive(Debug, Clone)]
pub enum InterpreterError {
  VariableNotDefined(String),
  TypeMismatch(String),
}

fn execute_instructions(
  outer_stack_frame: &mut StackFrame,
  instructions: &Vec<Instruction>,
) -> Result<(), InterpreterError> {
  let mut stack_frame = StackFrame::empty();
  stack_frame.outer = Some(outer_stack_frame);
  stack_frame.execute(instructions)
}

impl StackFrame<'_> {
  pub fn empty() -> Self {
    Self {
      variables: HashMap::new(),
      outer: None,
    }
  }

  pub fn execute(&mut self, instructions: &Vec<Instruction>) -> Result<(), InterpreterError> {
    self.run(&instructions)
  }

  fn run(&mut self, instructions: &Vec<Instruction>) -> Result<(), InterpreterError> {
    for instruction in instructions {
      match instruction {
        Instruction::Break => break,
        Instruction::Value { value } => {
          let _ = self.evaluate_value(&value).unwrap();
        }
        Instruction::If {
          condition,
          instructions,
        } => {
          let condition = self.evaluate_value(&condition)?;
          if let Data::Boolean(condition) = condition {
            if condition {
              execute_instructions(self, instructions)?;
            }
          } else {
            return Err(InterpreterError::TypeMismatch(
              "Expected boolean for if condition".to_string(),
            ));
          }
        }
        Instruction::Else { instructions } => {
          execute_instructions(self, instructions)?;
        }
        Instruction::While {
          condition,
          instructions,
        } => {
          while {
            let condition = self.evaluate_value(&condition)?;
            if let Data::Boolean(condition) = condition {
              condition
            } else {
              return Err(InterpreterError::TypeMismatch(
                "Expected boolean for if condition".to_string(),
              ));
            }
          } {
            execute_instructions(self, &instructions)?;
          }
        }
        Instruction::Scope { instructions } => {
          execute_instructions(self, &instructions)?;
        }
        Instruction::Print { value } => {
          let value = self.evaluate_value(&value)?;
          let string = value.to_string();
          println!("{}", string);
        }
        Instruction::Input { variable } => todo!(),
      }
    }
    Ok(())
  }
  fn get_variable(&self, variable: &str) -> Option<&Data> {
    let value = self.variables.get(variable);
    if value.is_some() {
      return value;
    }
    if let Some(outer) = &self.outer {
      return outer.get_variable(variable);
    }
    None
  }
  fn evaluate_value(&mut self, value: &Value) -> Result<Data, InterpreterError> {
    let data = match value {
      Value::Number(number) => Data::Number(*number),
      Value::String(string) => Data::String(string.clone()),
      Value::Boolean(boolean) => Data::Boolean(*boolean),
      Value::Identifier(identifier) => {
        let variable = self.get_variable(identifier);
        if variable.is_none() {
          return Err(InterpreterError::VariableNotDefined(identifier.clone()));
        }
        variable.unwrap().clone()
      }
      Value::Expression(expr) => match expr.get_operator() {
        Operator::Add => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator add").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Number(&left + &right),
            (Data::String(left), Data::String(right)) => Data::String(left + &right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 strings or 2 numbers when adding".to_string(),
              ))
            }
          }
        }
        Operator::Subtract => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator subtract").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Number(&left - &right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 numbers when subtracting".to_string(),
              ))
            }
          }
        }
        Operator::Multiply => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator multiply").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Number(&left * &right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 numbers when multiplying".to_string(),
              ))
            }
          }
        }
        Operator::Divide => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator divide").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Number(&left / &right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 numbers when dividing".to_string(),
              ))
            }
          }
        }
        Operator::Modulo => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator modulo").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Number(&left % &right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 numbers when taking modulo".to_string(),
              ))
            }
          }
        }
        Operator::Equal => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator equal").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Boolean(left == right),
            (Data::String(left), Data::String(right)) => Data::Boolean(left == right),
            (Data::Boolean(left), Data::Boolean(right)) => Data::Boolean(left == right),
            _ => Data::Boolean(false),
          }
        }
        Operator::NotEqual => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator not equal").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Boolean(left != right),
            (Data::String(left), Data::String(right)) => Data::Boolean(left != right),
            (Data::Boolean(left), Data::Boolean(right)) => Data::Boolean(left != right),
            _ => Data::Boolean(true),
          }
        }
        Operator::LessThan => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator less than").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Boolean(left < right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 numbers ".to_string(),
              ))
            }
          }
        }
        Operator::LessThanOrEqual => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator less than or equal").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Boolean(left <= right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 numbers ".to_string(),
              ))
            }
          }
        }
        Operator::GreaterThan => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator greater than").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Boolean(left > right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 numbers ".to_string(),
              ))
            }
          }
        }
        Operator::GreaterThanOrEqual => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator greater than or equal").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Boolean(left >= right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 numbers ".to_string(),
              ))
            }
          }
        }
        Operator::And => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator and").as_str()),
          )?;
          match (left, right) {
            (Data::Boolean(left), Data::Boolean(right)) => Data::Boolean(left && right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 booleans ".to_string(),
              ))
            }
          }
        }
        Operator::Or => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator or").as_str()),
          )?;
          match (left, right) {
            (Data::Boolean(left), Data::Boolean(right)) => Data::Boolean(left || right),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 booleans ".to_string(),
              ))
            }
          }
        }
        Operator::Not => {
          let left = self.evaluate_value(&expr.get_left())?;
          match left {
            Data::Boolean(left) => Data::Boolean(!left),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 1 boolean ".to_string(),
              ))
            }
          }
        }

        Operator::Exponent => {
          let left = self.evaluate_value(&expr.get_left())?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator exponent").as_str()),
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Number(left.pow(&right)),
            _ => {
              return Err(InterpreterError::TypeMismatch(
                "Expected 2 numbers when taking exponent".to_string(),
              ))
            }
          }
        }
        Operator::Brackets => self.evaluate_value(&expr.get_left())?,
        Operator::Assign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable(left, right)?
        }
        Operator::AddAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Add)?
        }
        Operator::SubtractAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Subtract)?
        }
        Operator::MultiplyAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Multiply)?
        }
        Operator::DivideAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Divide)?
        }
        Operator::ModuloAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Modulo)?
        }
      },
    };
    Ok(data)
  }

  fn assign_variable(&mut self, left: &Value, right: &Value) -> Result<Data, InterpreterError> {
    if !match left {
      Value::Identifier(_) => true,
      _ => false,
    } {
      return Err(InterpreterError::TypeMismatch(
        "Expected identifier on left side of assignment".to_string(),
      ));
    }
    let name = match left {
      Value::Identifier(name) => name,
      _ => unreachable!(),
    };
    let data = self.evaluate_value(right)?;
    self.variables.insert(name.clone(), data.clone());
    Ok(data)
  }
  fn assign_variable_with_operator(
    &mut self,
    left: &Value,
    right: &Value,
    operator: Operator,
  ) -> Result<Data, InterpreterError> {
    if !match left {
      Value::Identifier(_) => true,
      _ => false,
    } {
      return Err(InterpreterError::TypeMismatch(
        "Expected identifier on left side of assignment".to_string(),
      ));
    }
    let name = match left {
      Value::Identifier(name) => name,
      _ => unreachable!(),
    };
    let operator = match operator {
      Operator::AddAssign => Operator::Add,
      Operator::SubtractAssign => Operator::Subtract,
      Operator::MultiplyAssign => Operator::Multiply,
      Operator::DivideAssign => Operator::Divide,
      Operator::ModuloAssign => Operator::Modulo,
      _ => operator,
    };
    let expression = Expression::new(operator, left.clone(), right.clone());
    let value = Value::Expression(Box::new(expression));
    let data = self.evaluate_value(&value)?;
    self.variables.insert(name.clone(), data.clone());
    Ok(data)
  }
}

#[derive(Debug, Clone)]
enum Data {
  Number(Number),
  String(String),
  Boolean(bool),
}

impl Data {
  fn to_string(&self) -> String {
    match self {
      Data::Number(number) => number.to_string(),
      Data::String(string) => string.clone(),
      Data::Boolean(boolean) => boolean.to_string(),
    }
  }
}
