use crate::abst::{Controller, Input, Presenter};

pub struct Mock {
  inputs: Vec<Input>,
  expected: f64,
}

impl Mock {
  #[allow(dead_code)]
  pub fn new(inputs: impl IntoIterator<Item = Input>, expected: f64) -> Self {
    Self {
      inputs: inputs.into_iter().collect(),
      expected,
    }
  }
}

impl Controller for Mock {
  fn get_inputs(&self) -> Vec<Input> {
    self.inputs.iter().cloned().collect()
  }
}

impl Presenter for Mock {
  fn show_error(&self, error: crate::exp::SyntaxError) {
    eprintln!("Occured an error: {:?}", error);
  }

  fn show_result(&self, calculated: f64) {
    assert_eq!(calculated, self.expected);
  }
}
