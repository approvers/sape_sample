mod abst;
mod exp;
mod op;
mod skin;

use op::calculate;
use skin::Console;

fn main() {
  let console = Console;
  calculate(&console, &console);
}

#[test]
fn case0() {
  use abst::Input::*;
  let mock = skin::Mock::new(vec![Num(2.0), Plus, Num(1.0)], 3.0);
  calculate(&mock, &mock);
}

#[test]
fn case1() {
  use abst::Input::*;
  let mock = skin::Mock::new(
    vec![
      Num(4.0),
      Cross,
      Num(3.0),
      Plus,
      Num(2.0),
      Division,
      Num(1.0),
    ],
    14.0,
  );
  calculate(&mock, &mock);
}

#[test]
fn case2() {
  use abst::Input::*;
  let mock = skin::Mock::new(
    vec![Num(4.0), Cross, LParen, Num(2.0), Plus, Num(1.0), RParen],
    12.0,
  );
  calculate(&mock, &mock);
}

#[test]
fn case3() {
  use abst::Input::*;
  let mock =
    skin::Mock::new(vec![Num(5.0), Cross, Num(3.0), Division, Num(2.0)], 7.5);
  calculate(&mock, &mock);
}

#[test]
fn case4() {
  use abst::Input::*;
  let mock =
    skin::Mock::new(vec![Num(1.0), Plus, Num(2.0), Minus, Num(3.0)], 0.0);
  calculate(&mock, &mock);
}
