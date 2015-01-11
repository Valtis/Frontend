use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;
/*
  Recursive descent parser that for now merely checks if input conforms to
  grammar. No syntax tree is built.
*/


// first string param serves as a placeholder for abstract syntax tree
pub fn parse(tokens: Tokens) -> Result<String, Vec<String>> {
  let mut parser = Parser::new(tokens);
  parser.parse()
}



struct Parser {
  tokens: Tokens,
  errors: Vec<String>,
}


impl Parser {
  fn new(tokens: Tokens) -> Parser {
    Parser { tokens: tokens, errors: Vec::new() }
  }

  fn parse(&mut self) -> Result<String, Vec<String>> {

    loop {
      let mut next_token:Option<SyntaxToken>;
      {
        next_token = self.tokens.next();
      }

      match next_token {
        Some(token) => self.parse_start_token(token),
        None => break,
      }
    }

    if self.errors.is_empty() {
      Ok("Placeholder".to_string())
    } else {
      Err(self.errors.clone())
    }
  }

  fn parse_start_token(&mut self, token: SyntaxToken) {
    match token.t_type {
      TokenType::Fn => { self.parse_function(); },
      _ => {
        self.register_error_and_skip_to(
          format!("Invalid token {}. Expected token {}", token, TokenType::Fn),
          &token,
          vec![TokenType::Fn]);
      },
    }
  }


  fn parse_function(&mut self) {
    self.expect(TokenType::Identifier);

    self.parse_optional_function_arguments();
    self.parse_block();
  }

  fn parse_optional_function_arguments(&mut self) {

    if !self.expect(TokenType::LParen) {
      self.skip_to_one_of(vec![TokenType::LBrace]);
      return;
    }

    match self.tokens.peek() {
      Some(token) => {
        match token.t_type {
          TokenType::RParen => {
            self.tokens.next();
          },
          _ => {
            self.parse_function_argument_list();
            if !self.expect(TokenType::RParen) {
              self.skip_to_one_of(vec![TokenType::LBrace]);
            }

          },
        }
      },
      None => {
        self.errors.push(format!("Unexpected end of file: Expected token {}",
          TokenType::RParen));
        self.skip_to_one_of(vec![TokenType::LBrace]);
      }
    }
  }

  fn parse_function_argument_list(&mut self) {
    self.parse_function_parameter();

    match self.tokens.peek() {
      Some(token) => {
        if token.t_type == TokenType::Comma {
          self.tokens.next();
          self.parse_function_argument_list()
        }
      },
      None => { },
    }
  }

  fn parse_function_parameter(&mut self) {

    if !self.expect(TokenType::Identifier) {
      self.skip_to_one_of(vec![TokenType::Comma, TokenType::RParen]);
      return;
    }

    if !self.expect(TokenType::Colon) {
      self.skip_to_one_of(vec![TokenType::Comma, TokenType::RParen]);
      return;
    }

    self.parse_value_type();
  }


  fn parse_block(&mut self)  {

    if !self.expect(TokenType::LBrace) {
      self.skip_to_one_of(vec![TokenType::RBrace]);

    }
    self.parse_statements();

    self.expect(TokenType::RBrace);
  }

  fn parse_statements(&mut self)  {

    match self.tokens.peek() {
      Some(token) => {
        match (token.t_type) {
          TokenType::SemiColon => { self.tokens.next(); /* empty statement, skip */}
          TokenType::Let => { self.parse_variable_declaration(); },
          TokenType::LBrace => { self.parse_block(); }
          TokenType::RBrace => { return; /* end of block, return*/}
          _ => {
              self.register_error_and_skip_to(
                format!("Unexpected token {} when expecting start of statement",
                  token),
                &token,
                vec![TokenType::RBrace, TokenType::SemiColon]);
            }
          }
      },
      None => { return;/* empty statement list, end. Let the above level handle it*/ }
    }

    self.parse_statements();
  }

  fn parse_variable_declaration(&mut self) {
    if !self.expect(TokenType::Let) {
      self.skip_to_one_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return;
    }
    if !self.expect(TokenType::Identifier) {
      self.skip_to_one_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return;
    }
    if !self.expect(TokenType::Colon) {
      self.skip_to_one_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return;
    }

    if !self.parse_value_type() {
      self.skip_to_one_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return;
    }

    if !self.expect(TokenType::Assign) {
      self.skip_to_one_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return;
    }

    self.parse_expression();

    if !self.expect(TokenType::SemiColon) {
      self.skip_to_one_of(vec![TokenType::RBrace, TokenType::SemiColon]);
    }

  }

  fn parse_expression(&mut self) {

    match self.tokens.peek() {
      Some(token) => {
        match (token.t_type) {
          TokenType::Number | TokenType::Text | TokenType::Boolean  => { self.tokens.next(); return; },
          _ => self.register_error_and_skip_to(
            format!("Unexpected token {} when expecting start of expression", token),
            &token,
            vec![TokenType::SemiColon, TokenType::RBrace]),
        }
      },
      None => {
        self.errors.push("Unexpected end of file: Expected expression".to_string());
      }
    }
  }

  fn expect(&mut self, expected_type: TokenType) -> bool {
    match self.tokens.peek() {
      Some(token) => {
        if expected_type == token.t_type {
          self.tokens.next();
          true
        } else {
          self.register_error(
            format!("Token was not of expected type {}. Was actually {}",
              expected_type, token),
            &token);

          false
        }
      },
      None => {
        self.errors.push(
          format!("Expected token of type {}. Instead found end-of-file",
            expected_type));

        false
      },
    }
  }

  fn parse_any_type(&mut self) {
    self.expect(TokenType::VarType);
  }

  fn parse_value_type(&mut self) -> bool {
    match self.tokens.next() {
      Some(token) => {
        if token.t_type == TokenType::VarType {
          if token.t_subtype == TokenSubType::VoidType {
            self.register_error(
              format!("Expected a value type parameter, instead found {}",
              token),
              &token);
              false
          } else {
            true
          }
        } else {
          self.register_error(
            format!("Expected a type parameter, instead found {}", token),
            &token);
            false
        }
      },
      None => {
        self.errors
            .push("Expected a type parameter, instead found end-of-file".to_string());
        false
      }
    }
  }

  fn register_error_and_skip_to(&mut self, msg: String, err_token: &SyntaxToken,
     skip_tokens: Vec<TokenType>) {

    self.register_error(msg, err_token);
    self.skip_to_one_of(skip_tokens);
  }

  fn register_error(&mut self, msg:String, err_token: &SyntaxToken) {
    self.errors.push(format!("Error at {}:{}: {}",
      err_token.line, err_token.pos_at_line, msg));
  }

  fn skip_to_one_of(&mut self, skip_tokens: Vec<TokenType>) {
    loop {
      match self.tokens.peek() {
        Some(token) => {
          if skip_tokens.contains(&token.t_type) {
            break;
          } else {
            self.tokens.next();
          }
        },
        None => break,
      }
    }
  }
}
