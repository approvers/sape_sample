use crate::abst::{Controller, Input, Presenter};
use crate::exp::{BinaryOp, SyntaxError, Term};

pub fn calculate(controller: &impl Controller, presenter: &impl Presenter) {
  let mut inputs = controller.get_inputs();
  match parse_add_sub(&mut inputs) {
    Ok((term, _)) => {
      eprintln!("{:?}", term);
      presenter.show_result(term.calc())
    }
    Err(err) => presenter.show_error(err),
  }
}

type ParseResult<'a> = Result<(Term, &'a [Input]), SyntaxError>;

/*
  AddSub :=
    | MulDiv + MulDiv
    | MulDiv - MulDiv
    | MulDiv

  MulDiv :=
    | Factor * Factor
    | Factor / Factor
    | Factor

  Factor :=
    | LParen AddSub RParen
    | Num
*/

fn parse_add_sub(tokens: &[Input]) -> ParseResult {
  eprintln!("AddSub: {:?}", tokens);
  let (mut term, mut tokens) = parse_mul_div(tokens)?;
  loop {
    match tokens {
      [Input::Plus, rest @ ..] => {
        let (right, rest) = parse_mul_div(rest)?;
        term = Term::Binary(term.into(), BinaryOp::Add, right.into());
        tokens = rest;
      }
      [Input::Minus, rest @ ..] => {
        let (right, rest) = parse_mul_div(rest)?;
        term = Term::Binary(term.into(), BinaryOp::Sub, right.into());
        tokens = rest;
      }
      _ => break Ok((term, tokens)),
    }
  }
}

fn parse_mul_div(tokens: &[Input]) -> ParseResult {
  eprintln!("MulDiv: {:?}", tokens);
  let (mut term, mut tokens) = parse_factor(tokens)?;
  loop {
    match tokens {
      [Input::Cross, rest @ ..] => {
        let (right, rest) = parse_factor(rest)?;
        term = Term::Binary(term.into(), BinaryOp::Mul, right.into());
        tokens = rest;
      }
      [Input::Division, rest @ ..] => {
        let (right, rest) = parse_factor(rest)?;
        term = Term::Binary(term.into(), BinaryOp::Div, right.into());
        tokens = rest;
      }
      _ => break Ok((term, tokens)),
    }
  }
}

fn parse_factor(tokens: &[Input]) -> ParseResult {
  eprintln!("Factor: {:?}", tokens);
  match tokens {
    [Input::Num(n), ..] => {
      return Ok((Term::Num(*n), &tokens[1..]));
    }
    [Input::LParen, rest @ ..] => {
      let (term, rest) = parse_add_sub(rest)?;
      if let [Input::RParen, ..] = rest {
        Ok((term, &rest[1..]))
      } else {
        Err(SyntaxError)
      }
    }
    _ => parse_add_sub(tokens),
  }
}
