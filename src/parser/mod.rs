use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;


pub fn parse(tokens: &Tokens) -> Result<String, Vec<String>> {
  let mut parser = Parser::new(tokens);
  parser.parse();


  Ok("This is a placeholder for abstract syntax tree".to_string())
}



struct Parser<'a> {
  tokens :&'a Tokens,
}


impl<'a> Parser<'a> {
  fn new(tokens: &'a Tokens) -> Parser {
    Parser { tokens: tokens }
  }

  fn parse(&mut self) {


  }
}
