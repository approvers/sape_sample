use crate::exp::SyntaxError;

#[derive(Debug, Clone)]
pub enum Input {
  Plus,
  Minus,
  Cross,
  Division,
  LParen,
  RParen,
  Num(f64),
}

pub trait Controller {
  fn get_inputs(&self) -> Vec<Input>;
}

pub trait Presenter {
  fn show_error(&self, error: SyntaxError);
  fn show_result(&self, calculated: f64);
}
