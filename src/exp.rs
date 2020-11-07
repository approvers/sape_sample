#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
}

#[derive(Debug, Clone)]
pub enum Term {
  Num(f64),
  Binary(Box<Term>, BinaryOp, Box<Term>),
}

impl Term {
  pub fn calc(&self) -> f64 {
    use BinaryOp::*;
    use Term::*;
    match &self {
      Num(num) => *num,
      Binary(left, Add, right) => left.calc() + right.calc(),
      Binary(left, Sub, right) => left.calc() - right.calc(),
      Binary(left, Mul, right) => left.calc() * right.calc(),
      Binary(left, Div, right) => left.calc() / right.calc(),
    }
  }
}

#[test]
fn test_term() -> Result<(), Box<dyn std::error::Error>> {
  use Term::*;
  let terms = &[
    Num(1423523.08),
    Binary(Num(0.1).into(), BinaryOp::Add, Num(0.2).into()),
    Binary(Num(0.5).into(), BinaryOp::Sub, Num(1.).into()),
    Binary(Num(10.).into(), BinaryOp::Mul, Num(0.5).into()),
    Binary(Num(10.).into(), BinaryOp::Div, Num(0.5).into()),
  ];
  let expecteds = &[1423523.08, 0.1 + 0.2, -0.5, 5., 20.];
  for (term, expected) in terms.iter().zip(expecteds.iter()) {
    assert_eq!(term.calc(), *expected);
  }
  Ok(())
}

#[derive(Debug)]
pub struct SyntaxError;
