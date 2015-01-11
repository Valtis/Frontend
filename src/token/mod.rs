use std::num::Float;
use std::fmt;
#[derive(Eq, PartialEq, Show, Copy)]
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
  Comma,
  SemiColon,
  Colon,
  If,
  Else,
  While,
  For,
  Let,
  Fn,
  Return,
  Boolean,
  New,
  Class,
  Public,
  Protected,
  Private,
  VarType,
}

impl fmt::String for TokenType {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    fmt::String::fmt(
      match *self {
        TokenType::Assign => "Assign",
        TokenType::ArithOp => "Arithmetic operator",
        TokenType::CompOp => "Comparison operator",
        TokenType::Number => "Number",
        TokenType::Text => "Text",
        TokenType::Identifier => "Identifier",
        TokenType::LParen => "Left parenthesis",
        TokenType::RParen => "Right parenthesis",
        TokenType::LBrace => "Left brace",
        TokenType::RBrace => "Right brace",
        TokenType::LBracket => "Left bracket",
        TokenType::RBracket => "Right bracket",
        TokenType::Comma => "Comma",
        TokenType::SemiColon => "Semicolon",
        TokenType::Colon => "Colon",
        TokenType::If => "If",
        TokenType::Else => "Else",
        TokenType::While => "While",
        TokenType::For => "For",
        TokenType::Let => "Let",
        TokenType::Fn => "Fn",
        TokenType::Return => "Return",
        TokenType::Boolean => "Boolean",
        TokenType::New => "New",
        TokenType::Class => "Class",
        TokenType::Public => "Public",
        TokenType::Protected => "Protected",
        TokenType::Private => "Private",
        TokenType::VarType => "Type",

      }, formatter)
  }
}

#[derive(PartialEq, Show, Copy)]
pub enum TokenSubType {
  Text(usize), // index to text table
  FloatNumber(f32),
  DoubleNumber(f64),
  IntegerNumber(i32),
  Identifier(usize), // index to text table
  BooleanValue(bool),
  FloatType,
  DoubleType,
  IntegerType,
  BooleanType,
  VoidType,
  StringType,
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

impl fmt::String for TokenSubType {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

    write!(formatter, "{}", match *self {
        TokenSubType::Text(index) => "".to_string(),
        TokenSubType::FloatNumber(value) => format!("{}f", value.to_string()),
        TokenSubType::DoubleNumber(value) => format!("{}d", value.to_string()),
        TokenSubType::IntegerNumber(value) => value.to_string(),
        TokenSubType::Identifier(index) => "".to_string(),
        TokenSubType::BooleanValue(value) => value.to_string(),
        TokenSubType::FloatType => "float".to_string(),
        TokenSubType::DoubleType => "double".to_string(),
        TokenSubType::IntegerType => "int".to_string(),
        TokenSubType::BooleanType => "bool".to_string(),
        TokenSubType::VoidType => "void".to_string(),
        TokenSubType::StringType => "string".to_string(),
        TokenSubType::Equals => "==".to_string(),
        TokenSubType::Lesser => "<".to_string(),
        TokenSubType::Greater => ">".to_string(),
        TokenSubType::GreaterOrEq => ">=".to_string(),
        TokenSubType::LesserOrEq => "<=".to_string(),
        TokenSubType::NotEq => "!=".to_string(),
        TokenSubType::Assign => "=".to_string(),
        TokenSubType::Plus => "+".to_string() ,
        TokenSubType::Minus => "-".to_string(),
        TokenSubType::Multiply => "*".to_string(),
        TokenSubType::Divide => "/".to_string(),
        TokenSubType::NoSubType => "".to_string(),

      })
  }
}


#[derive(Show, Copy)]
pub struct SyntaxToken {
  pub t_type: TokenType,
  pub t_subtype: TokenSubType,
  pub line: i32,
  pub pos_at_line: i32
}


// do not check for line numbers or positions; only check for type\subtype equality
// also, special cases for floating point comparisons
impl PartialEq for SyntaxToken {

  fn eq(&self, other: &SyntaxToken) -> bool {
    if self.t_type == other.t_type {
      match self.t_subtype {
        TokenSubType::FloatNumber(self_val) => {
          match other.t_subtype {
            TokenSubType::FloatNumber(other_val) => (self_val - other_val).abs() < 0.0001,
            _=> false
          }
        }
        TokenSubType::DoubleNumber(self_val) => {
          match other.t_subtype {
            TokenSubType::DoubleNumber(other_val) => (self_val - other_val).abs() < 0.0001,
            _=> false
          }
        }

        _ => self.t_subtype == other.t_subtype
      }

    } else {
      false
    }
  }

}

impl SyntaxToken {
  pub fn new(token_type: TokenType, subtype: TokenSubType, line: i32, pos_at_line: i32) -> SyntaxToken {
    SyntaxToken { t_type: token_type, t_subtype: subtype, line: line, pos_at_line: pos_at_line }
  }
}

pub struct Tokens {
  text_table:Vec<String>,
  tokens: Vec<SyntaxToken>,
  pos: usize,
}

impl Tokens {

  pub fn new() -> Tokens {
    Tokens{ text_table: vec![], tokens: vec![], pos: 0}
  }

  pub fn set_text_table(&mut self, text_table:Vec<String>) {
    self.text_table = text_table;
  }

  pub fn get_text(&self, index: usize) -> &str {
    self.text_table[index].as_slice()
  }

  pub fn push(&mut self, token: SyntaxToken) {
    self.tokens.push(token);
  }

  pub fn peek(&self) -> Option<SyntaxToken> {
    if !self.invalid_pos() {
      Some(self.tokens[self.pos])
    } else {
      None
    }
  }

  pub fn next(&mut self) -> Option<SyntaxToken> {
    if !self.invalid_pos() {
      self.pos += 1;
      Some(self.tokens[self.pos-1])
    } else {
      None
    }
  }

  pub fn token_count(&self) -> usize {
    self.tokens.len()
  }

  fn invalid_pos(&self) -> bool {
    return self.pos >= self.tokens.len()
  }

  // text\identifier requires information from text table, so this is annoying
  // workaround. Otherwise could just directly use fmt::String-trait
  // (TODO: Figure out if subtype could carry a reference to the string instead of index)
  pub fn to_string(&self, token:&SyntaxToken) -> String {
    let subtype_string = match token.t_subtype {
      TokenSubType::Text(index) | TokenSubType::Identifier(index) => {
        self.text_table[index].clone()
      },
      _ => format!("{}", token.t_subtype)
    };
    format!("{} ({})", subtype_string, token.t_type )
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

  tokens.push(SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0));
  tokens.push(SyntaxToken::new(TokenType::LParen, TokenSubType::NoSubType, 0, 0));

  assert_eq!(false, tokens.invalid_pos());
}

#[test]
fn invalid_position_returns_true_when_queue_has_been_emptied() {

  let mut tokens = Tokens::new();

  tokens.push(SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0));
  tokens.push(SyntaxToken::new(TokenType::LParen, TokenSubType::NoSubType, 0 ,0));

  loop {
    match tokens.next() {
      Some(..) => { },
      None => break,
    }
  }

  assert_eq!(true, tokens.invalid_pos());
}
