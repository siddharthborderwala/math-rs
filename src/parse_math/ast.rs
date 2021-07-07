use std::error::Error;

#[derive(Debug, Clone)]
pub enum Node {
  Add(Box<Node>, Box<Node>),
  Multiply(Box<Node>, Box<Node>),
  Subtract(Box<Node>, Box<Node>),
  Divide(Box<Node>, Box<Node>),
  Caret(Box<Node>, Box<Node>),
  Negative(Box<Node>),
  Number(f64),
}

pub fn eval(expr: Node) -> Result<f64, Box<dyn Error>> {
  use self::Node::*;

  match expr {
    Number(i) => Ok(i),
    Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
    Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
    Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
    Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
    Negative(expr1) => Ok(-(eval(*expr1)?)),
    Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
  }
}
