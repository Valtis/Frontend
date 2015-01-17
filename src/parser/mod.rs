use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;
/*
  Recursive descent parser that for now merely checks if input conforms to
  grammar. No syntax tree is built.

  Check documentation for grammar.
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
        let token_str = self.tokens.to_string(&token);

        self.register_error_and_skip_to(
          format!(
              "Invalid token {}. Expected token {}", token_str, TokenType::Fn),
            &token,
            vec![TokenType::Fn]);
      },
    }
  }


  fn parse_function(&mut self) -> bool {
    let mut success = true;
    if !self.parse_function_declaration() {
      self.skip_to_first_of(vec![TokenType::LBrace,
        TokenType::Fn]);
      success = false;
      // if next token is lbrace, we can check the block for syntax issues
      // otherwise code is sufficiently broken that we should just bail out here

      if !self.next_token_is(TokenType::LBrace) {
        return false;
      }
    }

    self.parse_block() && success
  }

  fn parse_function_declaration(&mut self) -> bool {
    let mut success = true;

    if !self.expect(TokenType::Identifier) {
      // skip to start of block or start of parameter list
      self.skip_to_first_of(vec![TokenType::Fn, TokenType::LBrace,
        TokenType::LParen]);
      success = false;
      // if next token is not start of parameter list, bail out.
      // Otherwise, continue parsing in order to see if there are any
      // additional syntax issues
      if !self.next_token_is(TokenType::LParen) {
        return false;
      }
    }

    if !self.expect(TokenType::LParen) {
      self.skip_to_first_of(vec![TokenType::Fn, TokenType::LBrace,
        TokenType::RParen, TokenType::Identifier]);
      success = false;
      // same logic as above
      // empty if block for clarity, as I felt the alternative was a bit confusing
      if self.next_token_is(TokenType::RParen) ||
         self.next_token_is(TokenType::Identifier) {
          /* continue */
      } else {
        return false;
      }
    }

    if !self.parse_function_parameters() {
      self.skip_to_first_of(vec![TokenType::Fn, TokenType::LBrace,
        TokenType::RParen]);
      success = false;
      // same logic as above
      if !self.next_token_is(TokenType::RParen) {
        return false;
      }
    }

    if !self.expect(TokenType::RParen) {
      self.skip_to_first_of(vec![TokenType::Fn, TokenType::LBrace,
        TokenType::Colon]);
      success = false;
      // same logic as above
      if !self.next_token_is(TokenType::Colon) {
        return false;
      }
    }

    self.parse_optional_return_type() && success
  }

  fn parse_function_parameters(&mut self) -> bool {
    if self.next_token_is(TokenType::RParen) {
      true
    } else {
      self.parse_function_parameter_list()
    }
  }

  fn parse_function_parameter_list(&mut self) -> bool {

    let mut success = true;

    if !self.parse_function_parameter() {
      success = false;
      self.skip_to_first_of(vec![TokenType::Fn, TokenType::Comma,
        TokenType::RParen, TokenType::LBrace]);

      // skipped whole list and either reached start of next block
      // or start of function -> bail out
      if self.next_token_is(TokenType::LBrace) ||
         self.next_token_is(TokenType::Fn) {
        return false;
      }
    }

    self.parse_additional_parameters() && success
  }

  fn parse_additional_parameters(&mut self) -> bool {
    if self.next_token_is(TokenType::RParen) {
      return true;
    }

    if !self.expect(TokenType::Comma) {
      self.skip_to_first_of(vec![TokenType::Fn, TokenType::Comma,
        TokenType::RParen, TokenType::LBrace]);

      if self.next_token_is(TokenType::Comma) {
        self.parse_additional_parameters();
      }

      return false;
    }

    let mut success = true;
    if !self.parse_function_parameter() {
      success = false;
      self.skip_to_first_of(vec![TokenType::Fn, TokenType::Comma,
        TokenType::RParen, TokenType::LBrace]);
      if !self.next_token_is(TokenType::Comma) {
          return false;
      }
    }

    self.parse_additional_parameters() && success
  }


  fn parse_function_parameter(&mut self) -> bool {

    if !self.expect(TokenType::Identifier) {
      return false;
    }

    if !self.expect(TokenType::Colon) {
      return false;
    }

    self.expect(TokenType::VarType)
  }


  fn parse_optional_return_type(&mut self) -> bool {

    if self.next_token_is(TokenType::Colon) {
      self.tokens.next();

      if !self.expect(TokenType::VarType) {
        return false;
      }
    }

    true
  }

  fn parse_block(&mut self) -> bool {

    if !self.expect(TokenType::LBrace) {
      self.skip_to_first_of(vec![TokenType::RBrace]);
      self.tokens.next();
      return false;
    }
    self.parse_statements();

    self.expect(TokenType::RBrace)
  }

  fn parse_statements(&mut self)  {
    if self.next_token_is(TokenType::RBrace) {
      return;
    }

    if !self.parse_statement() {
      self.skip_to_first_of(vec![TokenType::RBrace, TokenType::SemiColon, TokenType::Fn]);

      // next token is not rbrace or semicolon (so either fn or end-of-file -> bail out)
      if !self.next_token_is(TokenType::RBrace) && !self.next_token_is(TokenType::SemiColon) {
        return;
      }
    }

    self.parse_statements();
  }


  fn parse_statement(&mut self) -> bool {
    match self.tokens.peek() {
      Some(token) => {
        match (token.t_type) {
        TokenType::SemiColon => { self.tokens.next(); true } // empty statement
        TokenType::Let => self.parse_variable_declaration() && self.expect(TokenType::SemiColon),
        TokenType::LBrace => self.parse_block(),
        TokenType::Identifier =>
            self.parse_variable_assignment_or_function_call() && self.expect(TokenType::SemiColon),
        TokenType::For => self.parse_for_loop(),
        _ => {
          let token_str = self.tokens.to_string(&token);
          self.register_error_and_skip_to(
            format!("Unexpected token {} when expecting start of statement",
            token_str),
            &token,
            vec![TokenType::RBrace, TokenType::SemiColon]);
            false
          }
        }
        },
      None => { false /* empty statement list, end. Let the above level handle it*/ }
    }
  }


  fn parse_variable_declaration(&mut self) -> bool {
    if !self.expect(TokenType::Let) {
      self.skip_to_first_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return false;
    }
    if !self.expect(TokenType::Identifier) {
      self.skip_to_first_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return false;
    }
    if !self.expect(TokenType::Colon) {
      self.skip_to_first_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return false;
    }

    if !self.expect(TokenType::VarType) {
      self.skip_to_first_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return false;
    }

    if !self.expect(TokenType::Assign) {
      self.skip_to_first_of(vec![TokenType::RBrace, TokenType::SemiColon]);
      return false;
    }

    self.parse_expression();

    true
  }

  fn parse_variable_assignment_or_function_call(&mut self) -> bool {
    match self.tokens.peek_2() {
      Some(token) => match token.t_type {
        TokenType::LParen => {
          if !self.parse_function_call() {
            self.skip_to_first_of(vec![TokenType::SemiColon]);
            false
          } else {
            true
          }
        },
        TokenType::Assign => if !self.parse_variable_assignment() {
          self.skip_to_first_of(vec![TokenType::SemiColon, TokenType::LBrace, TokenType::RBrace,
            TokenType::Fn]);
          false
          } else {
            true
          },
        _ => {
          let token_str = self.tokens.to_string(&token);

          self.register_error_and_skip_to(
            format!("Unexpected token {}. Expected {} for variable assignment or {}
                     for function call",
              token_str, TokenType::Assign, TokenType::LParen),
            &token,
            vec![TokenType::RBrace, TokenType::SemiColon]);
            false
        }
      },
      _ => { self.errors.push("Unexpected end-of-line".to_string()); false },
    }
  }

  fn parse_function_call(&mut self) -> bool {

    if !self.expect(TokenType::Identifier) {
      return false;
    }

    if !self.expect(TokenType::LParen) {
      return false;
    }

    if !self.parse_optional_function_call_argument_list() {
        return false;
    }

    if !self.expect(TokenType::RParen) {
      self.skip_to_first_of(vec![TokenType::SemiColon]);
      return false;
    }

    true
  }

  fn parse_optional_function_call_argument_list(&mut self) -> bool {
    if self.next_token_is(TokenType::RParen) {
      return true;
    }

    self.parse_function_call_argument_list()
  }

  fn parse_function_call_argument_list(&mut self) -> bool {
    let mut success = true;

    if !self.parse_expression() {
      success = false;
      self.skip_to_first_of(vec![TokenType::SemiColon, TokenType::Comma,
         TokenType::RBrace, TokenType::LBrace, TokenType::Fn]);

      // if we reached comma, continue parse, otherwise bail out
      if !self.next_token_is(TokenType::Comma) {
        return false;
      }
    }

    self.parse_additional_function_call_arguments() && success
  }

  fn parse_additional_function_call_arguments(&mut self) -> bool {
    if self.next_token_is(TokenType::RParen) {
      return true;
    }

    let mut success = true;

    if !self.expect(TokenType::Comma) {
      self.skip_to_first_of(vec![TokenType::SemiColon, TokenType::Comma,
        TokenType::RBrace, TokenType::LBrace, TokenType::Fn]);

      if self.next_token_is(TokenType::Comma) {
        self.parse_additional_function_call_arguments();
      }

      return false;
    }

    if !self.parse_expression() {
      success = false;
      self.skip_to_first_of(vec![TokenType::SemiColon, TokenType::Comma,
        TokenType::RBrace, TokenType::LBrace, TokenType::Fn]);
      if !self.next_token_is(TokenType::Comma) {
        return false;
      }
    }

    self.parse_additional_function_call_arguments() && success
  }

  fn parse_variable_assignment(&mut self) -> bool {


    if !self.expect(TokenType::Identifier) {
      return false;
    }

    if !self.expect(TokenType::Assign) {
      return false;
    }

    if !self.parse_expression() {
      return false;
    }

    true
  }

  fn parse_for_loop(&mut self) -> bool {
    if !self.expect(TokenType::For) {
      return false;
    }

    if !self.expect(TokenType::LParen) {

      self.skip_to_first_of(vec![TokenType::LBrace, TokenType::SemiColon, TokenType::Fn]);
      return false;
    }


    if !self.next_token_is(TokenType::SemiColon) {
      self.parse_optional_variable_declaration_or_assignment();
    }

    self.expect(TokenType::SemiColon);

    if !self.next_token_is(TokenType::SemiColon) {
      self.parse_expression();
    }

    self.expect(TokenType::SemiColon);

    if !self.next_token_is(TokenType::RParen) {
      self.parse_optional_variable_assignment();
    }

    if !self.expect(TokenType::RParen) {
      self.skip_to_first_of(vec![TokenType::LBrace, TokenType::SemiColon, TokenType::Fn]);
      return false;
    }

    self.parse_block()
  }

  fn parse_optional_variable_declaration_or_assignment(&mut self) -> bool {
    if self.next_token_is(TokenType::Let) {
      self.parse_variable_declaration()
      } else {
        self.parse_optional_variable_assignment()
      }
    }

  fn parse_optional_variable_assignment(&mut self) -> bool {
    if self.next_token_is(TokenType::Identifier) {
      self.parse_variable_assignment()
    } else {
      true
    }
  }




  fn parse_expression(&mut self) -> bool {
    self.parse_expression_2() && self.parse_equality_expression()
  }

  fn parse_equality_expression(&mut self) -> bool {
    match self.tokens.peek() {
      Some(token) => {
        if token.t_type == TokenType::CompOp && token.t_subtype == TokenSubType::Equals {
          self.tokens.next();
          self.parse_expression_2() && self.parse_equality_expression()
        } else {
          true
        }
      },
      None => { true },
    }
  }



  fn parse_expression_2(&mut self) -> bool {
    self.parse_expression_3() && self.parse_less_more_expression()
  }

  fn parse_less_more_expression(&mut self) -> bool {
    match self.tokens.peek() {
      Some(token) => {
        if token.t_type == TokenType::CompOp {
          match (token.t_subtype) {
            TokenSubType::Lesser | TokenSubType::Greater | TokenSubType::GreaterOrEq |
            TokenSubType::LesserOrEq => {
              self.tokens.next();
              self.parse_expression_3() && self.parse_less_more_expression()
            },
            _ => true
          }
        } else {
          true
        }
      }
      None => { true },
    }
  }

  // see grammar for better description. I need to figure out better naming
  fn parse_expression_3(&mut self) -> bool {
    self.parse_term() && self.parse_plus_minus_expression()
  }

  fn parse_plus_minus_expression(&mut self) -> bool {
    match self.tokens.peek() {
      Some(token) => match token.t_subtype {
        TokenSubType::Plus | TokenSubType::Minus => {
          self.tokens.next();
          self.parse_term() && self.parse_plus_minus_expression()
        },
        _ => { true }
      },
      None => { true },
    }
  }

  fn parse_term(&mut self) -> bool {
    self.parse_factor() && self.parse_mult_div_term()
  }

  fn parse_mult_div_term(&mut self) -> bool {
    match self.tokens.peek() {
      Some(token) => match token.t_subtype {
        TokenSubType::Multiply | TokenSubType::Divide => {
          self.tokens.next();
          self.parse_factor() && self.parse_mult_div_term()
          },
        _ => { true }
        },
        None => { true },
      }
  }

  fn parse_factor(&mut self) -> bool {
    // helper function
    let factor_err =  |&: parser:&mut Parser, token:&SyntaxToken| -> bool {

      let token_str = parser.tokens.to_string(token);

      parser.register_error(
        format!("Invalid token {}. Expected an expression",
                 token_str),
      token);
      false
    };

    match self.tokens.peek() {
      Some(token) => match token.t_type {
        // check if op is + or -, and if it is followed by a number. If so, accept.
        TokenType::ArithOp => {
          self.tokens.next();
          self.parse_plus_minus_number(&token, factor_err)
        },
        TokenType::Identifier => { self.tokens.next(); true },
        TokenType::Number | TokenType::Text | TokenType::Boolean => { self.tokens.next(); true },
        TokenType::LParen => {
          self.tokens.next();
          self.parse_expression();
          self.expect(TokenType::RParen)
        }
        _ => factor_err(self, &token),
      },
      None => {
        self.errors.push(
          "Unexpected end of file when parsing expression".to_string());
          false
        },
    }
  }

  fn parse_plus_minus_number<F: Fn(&mut Parser, &SyntaxToken) -> bool>
        (&mut self, token:&SyntaxToken, factor_err: F) -> bool {

    match token.t_subtype {
      TokenSubType::Plus | TokenSubType::Minus => {
        match self.tokens.peek() {
          Some(peek_token) => match peek_token.t_type {
            TokenType::Number => { self.tokens.next(); true },
            _ => factor_err(self, token),
            },
            None => factor_err(self, token),
          }
        }
        _ => factor_err(self, token),
      }
  }

  fn expect(&mut self, expected_type: TokenType) -> bool {
    match self.tokens.peek() {
      Some(token) => {
        if expected_type == token.t_type {
          self.tokens.next();
          true
        } else {
          let token_str = self.tokens.to_string(&token);
          self.register_error(
            format!("Token was not of expected type {}. Was actually {}",
              expected_type, token_str),
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

  fn next_token_is(&mut self, token_type: TokenType) -> bool {
    match self.tokens.peek() {
      Some(token) => {
        token.t_type == token_type
      }
      None => false,
    }
  }

  fn register_error_and_skip_to(&mut self, msg: String, err_token: &SyntaxToken,
     skip_tokens: Vec<TokenType>) {

    self.register_error(msg, err_token);
    self.skip_to_first_of(skip_tokens);
  }

  fn register_error(&mut self, msg:String, err_token: &SyntaxToken) {
    self.errors.push(format!("Error at {}:{}: {}",
      err_token.line, err_token.pos_at_line, msg));
  }

  fn skip_to_first_of(&mut self, skip_tokens: Vec<TokenType>) {
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
