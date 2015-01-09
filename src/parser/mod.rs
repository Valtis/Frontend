use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;


// first string param serves as a placeholder for abstract syntax tree
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
      let mut next_token:Option<SyntaxToken>;

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
    match token.t_type {
      TokenType::Fn => self.parse_function(),
      _ => Err(format!("Invalid token {}. Expected token Fn",
          token.pos_at_line)),
    }
  }


  fn parse_function(&mut self) -> Result<String, String> {
    match self.tokens.expect(TokenType::Identifier) {
      Ok(..) => {
        let ret_val = try!(self.parse_optional_function_arguments());
        try!(self.tokens.expect(TokenType::LBrace));
        try!(self.tokens.expect(TokenType::RBrace));
        Ok(ret_val)
      },
      Err(err) => Err(err),
    }
  }

  fn parse_optional_function_arguments(&mut self) -> Result<String, String> {
    try!(self.tokens.expect(TokenType::LParen));

    match self.tokens.peek() {
      Some(token) => {
        match token.t_type {
          TokenType::RParen => {
            try!(self.tokens.expect(TokenType::RParen));
            Ok("Placeholder".to_string())
          },
          _ => self.result_for_function_argument_list_parsing(),
        }
      },
      None => Err("Unexpected end of file: Expected token RParen".to_string()),
    }
  }

  fn result_for_function_argument_list_parsing(&mut self) -> Result<String, String> {

    match self.parse_function_argument_list() {
      Ok(val) => {
        try!(self.tokens.expect(TokenType::RParen));
        Ok("placeholder".to_string())
      },
      Err(err) => Err(err),
    }
  }

  fn parse_function_argument_list(&mut self) -> Result<String, String> {
    try!(self.parse_function_parameter());

    match self.tokens.peek() {
      Some(token) => {
        match token.t_type {
          TokenType::Comma => { self.tokens.next(); self.parse_function_argument_list() },
          _ => Ok("Placeholder".to_string()),
        }
      }
      None => Err("Unexpected end of file".to_string()),
    }

  }

  fn parse_function_parameter(&mut self) -> Result<String, String> {

    try!(self.tokens.expect(TokenType::Identifier));
    try!(self.tokens.expect(TokenType::Colon));
    try!(self.tokens.expect(TokenType::VarType));

    Ok("placeholder".to_string())
  }

}
