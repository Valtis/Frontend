
#[deriving(Eq, PartialEq, Show)]
pub enum TokenType {
  ReservedWord,
  Number,
  Text,
  Identifier,
  Symbol,
}


#[deriving(Eq, PartialEq, Show)]
pub enum TokenSubType {
  If,
  While,
  Let,
  Function,
  Text,
  FloatNumber,
  DoubleNumber,
  IntegerNumber,
  Identifier,
  LParen,
  RParen,
  LBrace,
  RBrance,
  LBracket,
  RBracket,
  Equals,
  Lesser,
  Greater,
  Assign,
  Plus,
  Minus,
  Multiply,
  Divide,
  Quote,
  Comment,
}

#[deriving(Show)]
pub struct SyntaxToken {
  pub t_type: TokenType,
  pub t_subtype: TokenSubType,
  pub t_value: String,
}

impl SyntaxToken {
  pub fn new(token_type: TokenType, subtype: TokenSubType, value: String) -> SyntaxToken {
    SyntaxToken { t_type: token_type, t_subtype: subtype, t_value: value }
  }
}



#[test]
fn can_create_new_tokens() {
  let token = SyntaxToken::new(TokenType::ReservedWord, TokenSubType::If, "if".to_string());
  assert_eq!(TokenType::ReservedWord, token.t_type);
  assert_eq!(TokenSubType::If, token.t_subtype);
  assert_eq!("if", token.t_value.as_slice());
}
