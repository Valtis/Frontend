
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
  NoSubType,
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


  // TODO: Move peek & next to trait implementations
  pub fn peek(&self) -> Option<&SyntaxToken> {
    if !self.invalid_pos() {
      Some(&self.tokens[self.pos])
    } else {
      None
    }
  }

  pub fn next(&mut self) -> Option<&SyntaxToken> {
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
fn invalid_position_returns_true_on_empty_queue() {
  let tokens = Tokens::new();
  assert_eq!(true, tokens.invalid_pos());
}


#[test]
fn invalid_position_returns_false_on_non_empty_queue() {
  let mut tokens = Tokens::new();

  tokens.push(SyntaxToken::new(TokenType::If, TokenSubType::NoSubType));
  tokens.push(SyntaxToken::new(TokenType::LParen, TokenSubType::NoSubType));

  assert_eq!(false, tokens.invalid_pos());
}

#[test]
fn invalid_position_returns_true_when_queue_has_been_emptied() {

  let mut tokens = Tokens::new();

  tokens.push(SyntaxToken::new(TokenType::If, TokenSubType::NoSubType));
  tokens.push(SyntaxToken::new(TokenType::LParen, TokenSubType::NoSubType));

  loop {
    match tokens.next() {
      Some(..) => { },
      None => break,
    }
  }

  assert_eq!(true, tokens.invalid_pos());
}
