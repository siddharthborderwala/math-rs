use super::ast::Node;
use super::token::{OperatorPrecedence, Token};
use super::tokenizer::Tokenizer;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
  UnableToParse(String),
  InvalidOperator(String),
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      self::ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
      self::ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
    }
  }
}

impl std::convert::From<Box<dyn std::error::Error>> for ParseError {
  fn from(_evalerr: Box<dyn std::error::Error>) -> Self {
    ParseError::UnableToParse("Unable to parse".into())
  }
}

pub struct Parser<'a> {
  tokenizer: Tokenizer<'a>,
  current_token: Token,
}

impl<'a> Parser<'a> {
  pub fn new(expr: &'a str) -> Result<Self, ParseError> {
    let mut lexer = Tokenizer::new(expr);
    let cur_token = match lexer.next() {
      Some(token) => token,
      None => return Err(ParseError::InvalidOperator("Invalid character".into())),
    };
    Ok(Parser {
      tokenizer: lexer,
      current_token: cur_token,
    })
  }

  // parses token returned by tokenizer, and computes the ast
  pub fn parse(&mut self) -> Result<Node, ParseError> {
    let ast = self.generate_ast(OperatorPrecedence::DefaultZero);
    match ast {
      Ok(ast) => Ok(ast),
      Err(e) => Err(e),
    }
  }

  // main method that is called recursively
  fn generate_ast(&mut self, operator_precedence: OperatorPrecedence) -> Result<Node, ParseError> {
    let mut left_expr = self.parse_number()?;

    while operator_precedence < self.current_token.get_operator_precedence() {
      if self.current_token == Token::EOF {
        break;
      }
      let right_expr = self.convert_token_to_node(left_expr.clone())?;
      left_expr = right_expr;
    }

    Ok(left_expr)
  }

  // create ast node for numbers, taking into account negative signs
  // and handling parenthesis
  fn parse_number(&mut self) -> Result<Node, ParseError> {
    let token = self.current_token.clone();
    match token {
      Token::Subtract => {
        self.get_next_token()?;
        let expr = self.generate_ast(OperatorPrecedence::Negative)?;
        Ok(Node::Negative(Box::new(expr)))
      }
      Token::Num(i) => {
        self.get_next_token()?;
        Ok(Node::Number(i))
      }
      Token::LeftParen => {
        self.get_next_token()?;
        let expr = self.generate_ast(OperatorPrecedence::DefaultZero)?;
        self.check_parenthesis(Token::RightParen)?;
        if self.current_token == Token::LeftParen {
          let right = self.generate_ast(OperatorPrecedence::MulDiv)?;
          return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
        }
        Ok(expr)
      }
      _ => Err(ParseError::UnableToParse("Unable to parse".into())),
    }
  }

  // parses operators and converts to ast
  fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
    match self.current_token {
      Token::Add => {
        self.get_next_token()?;
        //Get right-side expression
        let right_expr = self.generate_ast(OperatorPrecedence::AddSub)?;
        Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
      }
      Token::Subtract => {
        self.get_next_token()?;
        //Get right-side expression
        let right_expr = self.generate_ast(OperatorPrecedence::AddSub)?;
        Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
      }
      Token::Multiply => {
        self.get_next_token()?;
        //Get right-side expression
        let right_expr = self.generate_ast(OperatorPrecedence::MulDiv)?;
        Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
      }
      Token::Divide => {
        self.get_next_token()?;
        //Get right-side expression
        let right_expr = self.generate_ast(OperatorPrecedence::MulDiv)?;
        Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
      }
      Token::Caret => {
        self.get_next_token()?;
        //Get right-side expression
        let right_expr = self.generate_ast(OperatorPrecedence::Power)?;
        Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
      }
      _ => Err(ParseError::InvalidOperator(format!(
        "Please enter valid operator {:?}",
        self.current_token
      ))),
    }
  }

  // checks for matching parenthesis in the expression
  fn check_parenthesis(&mut self, expected: Token) -> Result<(), ParseError> {
    if expected == self.current_token {
      self.get_next_token()?;
      Ok(())
    } else {
      Err(ParseError::InvalidOperator(format!(
        "Expected {:?} got {:?}",
        expected, self.current_token
      )))
    }
  }

  // gets next token from tokenizer and sets current_token field
  fn get_next_token(&mut self) -> Result<(), ParseError> {
    let next_token = match self.tokenizer.next() {
      Some(token) => token,
      None => return Err(ParseError::InvalidOperator("Invalid character".into())),
    };
    self.current_token = next_token;
    Ok(())
  }
}
