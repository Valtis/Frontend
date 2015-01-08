use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;


pub fn parse(tokens: &Tokens) -> Result<String, Vec<String>> {
  let mut parser = Parser::new(tokens);
  let mut errors: Vec<String> = Vec::new();

  match parser.parse() {
    Ok(..) => { /* do nothing for now*/ }
    Err(err) => {
      errors.push(err);
    }
  }

  if errors.is_empty() {
    Ok("This is a placeholder for abstract syntax tree".to_string())
  } else {
    Err(errors)
  }
}



struct Parser<'a> {
  tokens :&'a Tokens,
}


impl<'a> Parser<'a> {
  fn new(tokens: &'a Tokens) -> Parser {
    Parser { tokens: tokens }
  }

  fn parse(&mut self) -> Result<String, String> {

    Ok("Placeholder".to_string())
  }
}
