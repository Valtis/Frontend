use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;


pub fn parse(tokens: Tokens) -> Result<String, Vec<String>> {
  let mut parser = Parser::new(tokens);
  parser.parse()
}



struct Parser {
  tokens: Tokens,
}


impl Parser {
  fn new(tokens: Tokens) -> Parser {
    Parser { tokens: tokens }
  }

  fn parse(&mut self) -> Result<String, Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    loop {
      let mut next_token:Option<SyntaxToken> = None;

      {
        next_token = self.tokens.next();
      }

      match next_token {
        Some(token) => match self.parse_start_token(token) {
          Ok(..) => { /* handle correct parse */ },
          Err(err) => {
            errors.push(err);
          }
        },
        None => break,
      }
    }

    if errors.is_empty() {
      Ok("Placeholder".to_string())
    } else {
      Err(errors)
    }
  }

  fn parse_start_token(&mut self, token: SyntaxToken) -> Result<String, String> {

    Ok("placeholder".to_string())
  }


  fn parse_function(&mut self) {

  }

}
