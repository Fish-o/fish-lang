use std::env;

mod interpreter;
mod number;
mod parser;
mod tokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();
  let wd = std::env::current_dir()?;
  if args.len() != 2 {
    println!("Usage: {} <file>", args[0]);
    return Ok(());
  }
  let path = std::path::Path::new(&args[1]);
  if !path.exists() {
    println!("File '{}' does not exist", path.display());
    return Ok(());
  }
  let code = std::fs::read_to_string(path)?;

  // let code = r#"
  //   if ((1+5) > (1*2)) {
  //     print("Hello, world!");
  //   }
  // "#;
  let tokens = tokenizer::tokenize(&code);
  if tokens.is_err() {
    println!("Error tokenizing code: {:?}", tokens.err().unwrap());
    return Ok(());
  }
  let tokens = tokens.unwrap();

  let instructions = parser::parse(tokens);
  if instructions.is_err() {
    println!("Error parsing code: {:?}", instructions.err().unwrap());
    return Ok(());
  }
  let instructions = instructions.unwrap();

  let res = interpreter::interpret(instructions);
  if res.is_err() {
    println!("Error interpreting code: {:?}", res.err().unwrap());
    return Ok(());
  }
  Ok(())
}
