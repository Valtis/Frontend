
#[deriving(Eq, PartialEq, Show)]
pub enum TokenType {
  Assign,
  ArithOp,
  CompOp,
  Number,
  Text,
  Identifier,
  LParen,
  RParen,
  LBrace,
  RBrace,
  LBracket,
  RBracket,
  SemiColon,
  If,
  While
}


#[deriving(PartialEq, Show)]
pub enum TokenSubType {
  Text(String),
  FloatNumber(f32),
  DoubleNumber(f64),
  IntegerNumber(i32),
  Identifier(String),
  Equals,
  Lesser,
  Greater,
  GreaterOrEq,
  LesserOrEq,
  NotEq,
  Assign,
  Plus,
  Minus,
  Multiply,
  Divide,
  No_SubType,
}

#[deriving(Show, PartialEq)]
pub struct SyntaxToken {
  pub t_type: TokenType,
  pub t_subtype: TokenSubType
}

impl SyntaxToken {
  pub fn new(token_type: TokenType, subtype: TokenSubType) -> SyntaxToken {
    SyntaxToken { t_type: token_type, t_subtype: subtype}
  }
}

pub struct Tokens {
  tokens: Vec<SyntaxToken>,
  pos: uint,
}

impl Tokens {

  pub fn new() -> Tokens {
    Tokens{ tokens: vec![], pos: 0}
  }

  pub fn push(&mut self, token: SyntaxToken) {
    self.tokens.push(token);
  }

  pub fn peek(&self) -> Option<&SyntaxToken> {
    if !self.invalid_pos() {
      Some(&self.tokens[self.pos])
    } else {
      None
    }
  }

  pub fn pop(&mut self) -> Option<&SyntaxToken> {
    if !self.invalid_pos() {
      self.pos += 1;
      Some(&self.tokens[self.pos-1])
    } else {
      None
    }
  }

  pub fn expect(&mut self, token_type: TokenType) -> Result<&SyntaxToken, String> {
    if self.invalid_pos() {
      Err("Token queue is empty".to_string())
    } else if self.tokens[self.pos].t_type == token_type {
      self.pos += 1;
      Ok(&self.tokens[self.pos - 1])
    } else {
      Err(format!("Error: Token was not of expected type {}. Was actually {}",
      token_type, self.tokens[self.pos]))
    }
  }

  pub fn token_count(&self) -> uint {
    self.tokens.len()
  }

  fn invalid_pos(&self) -> bool {
    return self.pos >= self.tokens.len()
  }
}

























#[test]
fn can_create_new_tokens() {
  let token = SyntaxToken::new(TokenType::If, TokenSubType::No_SubType);
  assert_eq!(TokenType::If, token.t_type);
  assert_eq!(TokenSubType::No_SubType, token.t_subtype);
}

#[test]
fn floating_token_equality_works() {
  let first_token = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber(234.12));
  let second_token = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber(234.12));
  let third_token = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber(12.12));
  assert_eq!(first_token, second_token);
  assert_eq!(second_token, first_token);
  assert!(first_token != third_token);

}

#[test]
fn invalid_position_returns_true_on_empty_queue() {
  let tokens = Tokens::new();
  assert_eq!(true, tokens.invalid_pos());
}


#[test]
fn invalid_position_returns_false_on_non_empty_queue() {
  let tokens = create_queue();
  assert_eq!(false, tokens.invalid_pos());
}

#[test]
fn invalid_position_returns_true_when_queue_has_been_emptied() {

  let mut tokens = create_queue();

  loop {
    match tokens.pop() {
      Some(..) => { },
      None => break,
    }
  }

  assert_eq!(true, tokens.invalid_pos());
}

#[test]
fn peek_on_empty_token_queue_returns_none() {
  let tokens = Tokens::new();

  match tokens.peek() {
    Some(..) => assert!(false),
    None => assert!(true),
  }
}

#[test]
fn peek_retuns_the_wanted_token_from_queue() {
  let tokens = create_queue();
  let expected = SyntaxToken::new(TokenType::If, TokenSubType::No_SubType);

  match tokens.peek() {
    Some(actual) => assert_eq!(expected, *actual),
    None => assert!(false),
  }
}

#[test]
fn calling_peek_multiple_times_does_not_advance_the_queue() {
  let tokens = create_queue();
  let expected = SyntaxToken::new(TokenType::If, TokenSubType::No_SubType);

  tokens.peek();
  tokens.peek();
  tokens.peek();
  tokens.peek();

  match tokens.peek() {
    Some(actual) => assert_eq!(expected, *actual),
    None => assert!(false),
  }
}

#[test]
fn pop_on_empty_queue_returns_none() {

  let mut tokens = Tokens::new();

  match tokens.pop() {
    Some(..) => assert!(false),
    None => assert!(true),
  }
}

#[test]
fn pop_advances_queue() {

  let mut tokens = create_queue();

  let first_expected = SyntaxToken::new(TokenType::If, TokenSubType::No_SubType);
  let second_expected = SyntaxToken::new(TokenType::LParen, TokenSubType::No_SubType);
  let third_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("abc".to_string()));

  match tokens.pop() {
    Some(actual) => assert_eq!(first_expected, *actual),
    None => assert!(false),
  }

  match tokens.pop() {
    Some(actual) => assert_eq!(second_expected, *actual),
    None => assert!(false),
  }

  match tokens.pop() {
    Some  (actual) => assert_eq!(third_expected, *actual),
    None => assert!(false),
  }
}


#[test]
fn expect_on_empty_queue_returns_error() {
  let mut tokens = Tokens::new();

  match tokens.expect(TokenType::If) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn expect_with_wrong_token_type_returns_error() {
  let mut  tokens = create_queue();

  match tokens.expect(TokenType::While) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn expect_with_wrong_token_subtype_returns_error() {
  let mut tokens = create_queue();

  match tokens.expect(TokenType::SemiColon) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn expect_with_right_values_returns_the_token() {
  let mut  tokens = create_queue();

  let expected = SyntaxToken::new(TokenType::If, TokenSubType::No_SubType);

  match tokens.expect(TokenType::If) {
    Ok(actual) => assert_eq!(expected, *actual),
    Err(..) => assert!(false),
  }
}

#[test]
fn expect_advances_queue() {
  let mut tokens = create_queue();

  let first_expected = SyntaxToken::new(TokenType::If, TokenSubType::No_SubType);
  let second_expected = SyntaxToken::new(TokenType::LParen, TokenSubType::No_SubType);
  let third_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("abc".to_string()));

  match tokens.expect(TokenType::If) {
    Ok(actual) => assert_eq!(first_expected, *actual),
    Err(..) => assert!(false),
  }

  match tokens.expect(TokenType::LParen) {
    Ok(actual) => assert_eq!(second_expected, *actual),
    Err(..) => assert!(false),
  }

  match tokens.expect(TokenType::Identifier) {
    Ok(actual) => assert_eq!(third_expected, *actual),
    Err(..) => assert!(false),
  }
}


#[test]
fn expect_failure_followed_by_correct_values_advances_queue() {

  let mut tokens = create_queue();

  let first_expected = SyntaxToken::new(TokenType::If, TokenSubType::No_SubType);
  let second_expected = SyntaxToken::new(TokenType::LParen, TokenSubType::No_SubType);


  match tokens.expect(TokenType::Identifier) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }

  match tokens.expect(TokenType::If) {
    Ok(actual) => assert_eq!(first_expected, *actual),
    Err(..) => assert!(false),
  }

  match tokens.expect(TokenType::LParen) {
    Ok(actual) => assert_eq!(second_expected, *actual),
    Err(..) => assert!(false),
  }
}

#[test]
fn peek_returns_correct_value_after_pops_and_expects() {
  let mut tokens = create_queue();

  let first_expected = SyntaxToken::new(TokenType::If, TokenSubType::No_SubType);
  let second_expected = SyntaxToken::new(TokenType::LParen, TokenSubType::No_SubType);
  let third_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("abc".to_string()));

  match tokens.expect(TokenType::If) {
    Ok(actual) => assert_eq!(first_expected, *actual),
    Err(..) => assert!(false),
  }

  match tokens.pop() {
    Some(actual) => assert_eq!(second_expected, *actual),
    None => assert!(false),
  }

  match tokens.peek() {
    Some(actual) => assert_eq!(third_expected, *actual),
    None => assert!(false),
  }
}

fn create_queue() -> Tokens {

  let mut tokens = Tokens::new();

  // push tokens if (abc == 5) { def = 2; }
  tokens.push(SyntaxToken::new(TokenType::If, TokenSubType::No_SubType));
  tokens.push(SyntaxToken::new(TokenType::LParen, TokenSubType::No_SubType));
  tokens.push(SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("abc".to_string())));
  tokens.push(SyntaxToken::new(TokenType::CompOp, TokenSubType::Equals));
  tokens.push(SyntaxToken::new(TokenType::Number, TokenSubType::IntegerNumber(5)));
  tokens.push(SyntaxToken::new(TokenType::RParen, TokenSubType::No_SubType));
  tokens.push(SyntaxToken::new(TokenType::LBrace, TokenSubType::No_SubType));
  tokens.push(SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("def".to_string())));
  tokens.push(SyntaxToken::new(TokenType::Assign, TokenSubType::No_SubType));
  tokens.push(SyntaxToken::new(TokenType::Number, TokenSubType::IntegerNumber(2)));
  tokens.push(SyntaxToken::new(TokenType::SemiColon, TokenSubType::No_SubType));
  tokens.push(SyntaxToken::new(TokenType::RBrace, TokenSubType::No_SubType));

  tokens
}
