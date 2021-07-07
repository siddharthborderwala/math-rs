#[derive(PartialEq, Debug, Clone)]
pub enum Token {
  Add,
  Subtract,
  Multiply,
  Divide,
  Caret,
  LeftParen,
  RightParen,
  Num(f64),
  EOF,
}

impl Token {
  // get operator precedence based on the token
  pub fn get_operator_precedence(&self) -> OperatorPrecedence {
    use OperatorPrecedence::*;
    use Token::*;
    match *self {
      Add | Subtract => AddSub,
      Multiply | Divide => MulDiv,
      Caret => Power,
      _ => DefaultZero,
    }
  }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum OperatorPrecedence {
  DefaultZero,
  AddSub,
  MulDiv,
  Power,
  Negative,
}
