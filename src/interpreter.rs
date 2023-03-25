use std::{cell::RefCell, collections::HashMap};

use crate::{
  number::Number,
  parser::{Expression, Instruction, Value},
  tokenizer::Operator,
};
pub fn interpret(instructions: Vec<Instruction>) -> Result<(), InterpreterError> {
  let mut vm = VM::new();
  vm.execute_new_instructions(&instructions)?;
  Ok(())
}
#[derive(Debug, Clone)]
struct StackFrame {
  variables: HashMap<String, Data>,
}

#[derive(Debug, Clone)]
pub enum InterpreterError {
  VariableNotDefined(String),
  TypeMismatch(String),
}

impl StackFrame {
  pub fn empty() -> Self {
    Self {
      variables: HashMap::new(),
    }
  }

  pub fn execute(
    &mut self,
    instructions: &Vec<Instruction>,
    vm: &mut VM,
  ) -> Result<(), InterpreterError> {
    self.run(&instructions, vm)
  }

  fn run(&mut self, instructions: &Vec<Instruction>, vm: &mut VM) -> Result<(), InterpreterError> {
    let mut should_execute_else: Option<bool> = None;
    for instruction in instructions {
      if let Some(should_execute) = should_execute_else {
        if !should_execute {
          should_execute_else = None;
        } else {
          should_execute_else = Some(false);
        }
      }
      match instruction {
        Instruction::Break => break,
        Instruction::Value { value } => {
          let _ = self.evaluate_value(&value, vm).unwrap();
        }
        Instruction::If {
          condition,
          instructions,
        } => {
          let condition = self.evaluate_value(&condition, vm)?;
          if let Data::Boolean(condition) = condition {
            if condition {
              vm.execute_new_instructions(instructions)?;
            } else {
              should_execute_else = Some(true);
            }
          } else {
            return Err(InterpreterError::TypeMismatch(
              "Expected boolean for if condition".to_string(),
            ));
          }
        }
        Instruction::Else { instructions } => {
          if let Some(_) = should_execute_else {
            vm.execute_new_instructions(instructions)?;
          }
        }
        Instruction::While {
          condition,
          instructions,
        } => {
          while {
            let condition = self.evaluate_value(&condition, vm)?;
            if let Data::Boolean(condition) = condition {
              condition
            } else {
              return Err(InterpreterError::TypeMismatch(
                "Expected boolean for if condition".to_string(),
              ));
            }
          } {
            vm.execute_new_instructions(instructions)?;
          }
        }
        Instruction::Scope { instructions } => {
          vm.execute_new_instructions(instructions)?;
        }
        Instruction::Print { message: value } => {
          let value = self.evaluate_value(&value, vm)?;
          let string = value.to_string();
          println!("{}", string);
        }
        Instruction::Input { variable } => {
          let mut input = String::new();
          std::io::stdin().read_line(&mut input).unwrap();
          let input = input.trim();
          let input = Data::String(input.to_string());
          vm.assign_variable(variable, input)?;
        }
      }
    }
    Ok(())
  }

  fn evaluate_value(&mut self, value: &Value, vm: &mut VM) -> Result<Data, InterpreterError> {
    let data = match value {
      Value::Number(number) => Data::Number(*number),
      Value::String(string) => Data::String(string.clone()),
      Value::Boolean(boolean) => Data::Boolean(*boolean),
      Value::Identifier(identifier) => {
        let variable = vm.get_variable(identifier);
        if variable.is_none() {
          return Err(InterpreterError::VariableNotDefined(identifier.clone()));
        }
        variable.unwrap().0.clone()
      }
      Value::Expression(expr) => match expr.get_operator() {
        Operator::Add => {
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator add").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator subtract").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator multiply").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator divide").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator modulo").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator equal").as_str()),
            vm,
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Boolean(left == right),
            (Data::String(left), Data::String(right)) => Data::Boolean(left == right),
            (Data::Boolean(left), Data::Boolean(right)) => Data::Boolean(left == right),
            _ => Data::Boolean(false),
          }
        }
        Operator::NotEqual => {
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator not equal").as_str()),
            vm,
          )?;
          match (left, right) {
            (Data::Number(left), Data::Number(right)) => Data::Boolean(left != right),
            (Data::String(left), Data::String(right)) => Data::Boolean(left != right),
            (Data::Boolean(left), Data::Boolean(right)) => Data::Boolean(left != right),
            _ => Data::Boolean(true),
          }
        }
        Operator::LessThan => {
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator less than").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator less than or equal").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator greater than").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator greater than or equal").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator and").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator or").as_str()),
            vm,
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
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
          let left = self.evaluate_value(&expr.get_left(), vm)?;
          let right = self.evaluate_value(
            &expr
              .get_right()
              .expect(format!("No right for operator exponent").as_str()),
            vm,
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
        Operator::Brackets => self.evaluate_value(&expr.get_left(), vm)?,
        Operator::Assign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable(left, right, vm)?
        }
        Operator::AddAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Add, vm)?
        }
        Operator::SubtractAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Subtract, vm)?
        }
        Operator::MultiplyAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Multiply, vm)?
        }
        Operator::DivideAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Divide, vm)?
        }
        Operator::ModuloAssign => {
          let left = expr.get_left();
          let right = expr.get_right().expect("No right for assignment");
          self.assign_variable_with_operator(left, right, Operator::Modulo, vm)?
        }
      },
    };
    Ok(data)
  }

  fn assign_variable(
    &mut self,
    left: &Value,
    right: &Value,
    vm: &mut VM,
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
    let data = self.evaluate_value(right, vm)?;
    Ok(vm.assign_variable(name, data)?.clone())
  }

  fn assign_variable_with_operator(
    &mut self,
    left: &Value,
    right: &Value,
    operator: Operator,
    vm: &mut VM,
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
    let data = self.evaluate_value(&value, vm)?;
    Ok(vm.assign_variable(name, data)?.clone())
  }

  fn get_frame_variable(&mut self, name: &str) -> Option<&Data> {
    self.variables.get(name)
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

struct VM {
  instructions: Vec<Instruction>,
  stack: Vec<StackFrame>,
}

impl VM {
  fn new() -> VM {
    VM {
      instructions: vec![],
      stack: vec![],
    }
  }
  fn get_variable(&mut self, name: &str) -> Option<(&Data, usize)> {
    for (i, frame) in self.stack.iter_mut().enumerate().rev() {
      if let Some(data) = frame.get_frame_variable(name) {
        return Some((data, i));
      }
    }
    None
  }

  fn assign_variable(&mut self, name: &str, data: Data) -> Result<&Data, InterpreterError> {
    let existing_variable = self.get_variable(name);
    if let Some((_, i)) = existing_variable {
      self.stack[i].variables.insert(name.to_string(), data);
      return Ok(&self.stack[i].variables[name]);
    } else {
      self.stack[0].variables.insert(name.to_string(), data);
      return Ok(&self.stack[0].variables[name]);
    }
  }

  fn execute_new_instructions(
    &mut self,
    instructions: &Vec<Instruction>,
  ) -> Result<(), InterpreterError> {
    let new_frame = StackFrame::empty();
    self.stack.insert(0, new_frame);
    unsafe {
      // get the last stack as a mutable reference, then get self as a mutable reference
      // this is safe because we just inserted a new stack frame
      let stack = &mut *(&mut self.stack as *mut Vec<StackFrame>);
      let stack = &mut stack[0];
      stack.execute(instructions, self)?;
    }
    self.stack.remove(0);
    Ok(())
  }
}
