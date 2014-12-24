use token::TokenType;
use token::TokenSubType;
use token::SyntaxToken;

mod token;

fn main() {
  let foo = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber, "32421.1".to_string());

  let equals = TokenType::Number == TokenType::Text;
  let equals2 = TokenType::Number == TokenType::Number;

  println!("Foo: {}", foo);
  println!("{}, {}", equals, equals2);
}
