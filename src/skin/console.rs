use crate::abst::{Controller, Input, Presenter};
use crate::exp::SyntaxError;

pub struct Console;

fn string_to_tokens(string: &[char]) -> Vec<Input> {
  let mut tokens = vec![];
  let mut num_buf = String::new();
  for raw_token in string {
    use Input::*;
    if raw_token.is_digit(10) || raw_token == &'.' {
      num_buf.push(*raw_token);
      continue;
    }
    if !num_buf.is_empty() {
      tokens.push(Input::Num(num_buf.parse().unwrap()));
      num_buf.clear();
    }
    tokens.push(match raw_token {
      '+' => Plus,
      '-' => Minus,
      '*' => Cross,
      '/' => Division,
      '(' => LParen,
      ')' => RParen,
      ' ' => continue,
      _ => unimplemented!("Unimplemented token: {}", raw_token),
    });
  }
  if !num_buf.is_empty() {
    tokens.push(Input::Num(num_buf.parse().unwrap()));
    num_buf.clear();
  }
  tokens
}

impl Controller for Console {
  fn get_inputs(&self) -> Vec<Input> {
    println!("Input the expression with white-separated:");

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).expect("no input");

    let raw_tokens: Vec<_> = buffer.trim().chars().collect();
    string_to_tokens(&raw_tokens)
  }
}

impl Presenter for Console {
  fn show_error(&self, _: SyntaxError) {
    println!("Syntax error");
  }

  fn show_result(&self, result: f64) {
    println!("{}", result);
  }
}
