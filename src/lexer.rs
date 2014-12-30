
use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;
use std::str;
use std::iter;


pub fn tokenize(content: &str) -> Result<Tokens, String> {

  let mut tokens = Tokens::new();
  let mut lexer = Lexer::new(content);

  loop {
    match lexer.read_token() {
      Some(res) => match res {
        Ok(token) => tokens.push(token),
        Err(err_str) => {
          return Err(format!("Error at {}:{}: {}", lexer.cur_line_number, lexer.cur_line_pos, err_str));
        }
      },
      None => break,
    }
  }

  Ok(tokens)
}

struct Lexer<'a> {
  cur_line_number: i32,
  cur_line_pos: i32,
  token_start_line_number: i32,
  token_start_line_pos: i32,
  iter: iter::Peekable<char, str::Chars<'a>>,
}



impl<'a> Lexer<'a> {

  pub fn new(content: &'a str) -> Lexer<'a> {
    Lexer {
      cur_line_number: 1,
      cur_line_pos: 1,
      token_start_line_number: 1,
      token_start_line_pos: 1,
      iter: content.chars().peekable()
    }
  }

  pub fn read_token(&mut self) -> Option<Result<SyntaxToken, String>> {
    self.skip_whitespace();

    self.token_start_line_number = self.cur_line_number;
    self.token_start_line_pos = self.cur_line_pos;
    
    match self.next_char() {
      Some(ch) => {
        if Lexer::starts_symbol(ch) {
          Some(self.handle_symbols(ch))
        } else if Lexer::starts_identifier(ch) {
          Some(self.handle_identifier(ch))
        } else if self.starts_number(ch) {
          Some(self.handle_number(ch))
        } else if Lexer::starts_string(ch) {
          Some(self.handle_string())
        } else {
          Some(Err(format!("Unexpected symbol {}", ch)))
        }
      }
      None => None,
    }
  }


  fn starts_symbol(ch: char) -> bool {
    match ch {
      '+' | '-' | '*' | '/' | '[' | ']' | '{' | '}' | '(' | ')' => true,
      _ => false,
    }
  }

  fn handle_symbols(&mut self, ch: char) -> Result<SyntaxToken, String> {

    match ch {
      '+' => Ok(self.create_token(TokenType::ArithOp, TokenSubType::Plus)),
      '-' => Ok(self.create_token(TokenType::ArithOp, TokenSubType::Minus)),
      '*' => Ok(self.create_token(TokenType::ArithOp, TokenSubType::Multiply)),
      '/' => Ok(self.create_token(TokenType::ArithOp, TokenSubType::Divide)),
      '[' => Ok(self.create_token(TokenType::LBracket, TokenSubType::NoSubType)),
      ']' => Ok(self.create_token(TokenType::RBracket, TokenSubType::NoSubType)),
      '{' => Ok(self.create_token(TokenType::LBrace, TokenSubType::NoSubType)),
      '}' => Ok(self.create_token(TokenType::RBrace, TokenSubType::NoSubType)),
      '(' => Ok(self.create_token(TokenType::LParen, TokenSubType::NoSubType)),
      ')' => Ok(self.create_token(TokenType::RParen, TokenSubType::NoSubType)),
      _ => Err(format!("Not an operator: {}", ch))
    }
  }


  fn starts_identifier(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
  }

  fn is_valid_identifier_character(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
  }

  fn handle_identifier(&mut self, ch: char) -> Result<SyntaxToken, String> {

    let mut identifier = ch.to_string();

    loop {
      // workaround for multiple mutable borrows
      let mut value: Option<char>;
      {
        value = match self.iter.peek() {
          Some(ch) => Some(*ch),
          None => None,
        }
      }

      match value {
        Some(ch) => {
          if Lexer::is_valid_identifier_character(ch) {
            identifier.push(ch);
            self.next_char();
          } else {
            break;
          }
        }
        None => break
      }
    }

    Ok(self.create_token(TokenType::Identifier, TokenSubType::Identifier(identifier)))
  }

  // either a number, or dot followed by a number
  fn starts_number(&mut self, ch: char) -> bool {
    if ch.is_digit(10) {
      true
    } else if ch == '.' {
      match self.iter.peek() {
        Some(new_ch) => {
          new_ch.is_digit(10)
        }
        None => {
          false
        }
      }
    } else {
      false
    }
  }

  fn handle_number(&mut self, ch: char) -> Result<SyntaxToken, String> {

    let mut number_str = ch.to_string();

    if ch == '.' {
      return self.handle_decimal_number(number_str);
    }
    loop {

      // workaround for multiple mutable borrows
      let mut value: Option<char>;
      {
        value = match self.iter.peek() {
          Some(ch) => Some(*ch),
          None => None,
        }
      }

      match value {
        Some(ch) => {
          if ch.is_digit(10) {
            number_str.push(ch);
            self.next_char();
          } else if ch == '.' {
            number_str.push(ch);
            self.next_char();
            return self.handle_decimal_number(number_str);
          } else if ch.is_alphabetic() {
            self.next_char();
            return self.handle_number_type_char(ch, number_str);
          } else {
            break;
          }
        }
        None => break
      }

    }

    match number_str.parse() {
      Some(number) => Ok(self.create_token(TokenType::Number, TokenSubType::IntegerNumber(number))),
      None => Err("Internal error - non-numeric characters in number token".to_string()),
    }
  }

  fn handle_decimal_number(&mut self, mut number_str: String) -> Result<SyntaxToken, String> {
    loop {
      // workaround for multiple mutable borrows
      let mut value: Option<char>;
      {
        value = match self.iter.peek() {
          Some(ch) => Some(*ch),
          None => None,
        }
      }

      match value {
        Some(ch) => {
          if ch.is_digit(10) {
            number_str.push(ch);
            self.next_char();
          } else if ch.is_alphabetic() {
            self.next_char();
            return self.handle_number_type_char(ch, number_str);
          } else if ch == '.' {
            return Err("Multiple decimal separators in number".to_string());
          } else {
            break;
          }
        }
        None => break
      }
    }

    println!("Number: {}", number_str);

    match number_str.parse() {
      Some(number) => Ok(self.create_token(TokenType::Number, TokenSubType::DoubleNumber(number))),
      None => Err("Internal error - non-numeric characters in number token".to_string()),
    }
  }

  fn handle_number_type_char(&mut self, type_char: char, number_str: String) -> Result<SyntaxToken, String> {
    // check that character following the type char is not alphanumeric
    match self.iter.peek() {
      Some(ch) => {
        if ch.is_alphanumeric() {
          return Err(format!("Invalid character following number type character: {}", ch));
        }
      }
      None => { /* do nothing */}
    }

    match type_char {
      'd' => match number_str.parse() {
        Some(number) => Ok(self.create_token(TokenType::Number, TokenSubType::DoubleNumber(number))),
        None => Err("Internal error - non-numeric characters in number token".to_string()),
      },
      'f' => match number_str.parse() {
        Some(number) => Ok(self.create_token(TokenType::Number, TokenSubType::FloatNumber(number))),
        None => Err("Internal error - non-numeric characters in number token".to_string()),
      },
      _ => Err(format!("Invalid type character: {}", type_char)),
    }

  }


  fn starts_string(ch: char) -> bool {
    ch == '"'
  }

  fn handle_string(&mut self) -> Result<SyntaxToken, String> {

    let mut value: String = String::new();

    loop {
      match self.next_char() {
        Some(ch) => {
          if ch == '\\' {
            value.push(try!(self.handle_escape_sequence()));
          } else if ch == '"' {
            // check that there are no alphanumeric characters following the '"'
            match self.iter.peek() {
              Some(ch) => {
                if ch.is_alphanumeric() {
                  return Err(format!("Invalid character following closing\" in string: {}", ch));
                }
              },
              None => { /* do nothing*/}
            }
            break;
          } else {
            value.push(ch);
          }
        }
        None => return Err("Unterminated string".to_string()),
      }
    }

    Ok(self.create_token(TokenType::Text, TokenSubType::Text(value)))
  }

  fn handle_escape_sequence(&mut self) -> Result<char, String> {
    match self.next_char() {
      Some(ch) => match ch {
        'n' => Ok('\n'),
        't' => Ok('\t'),
        '\\' => Ok('\\'),
        '"' => Ok('"'),
        _ => Err(format!("Invalid escape sequence \\{}", ch))
      },
      None => Err("Invalid escape sequence - no character following \\".to_string()),
    }
  }

  fn create_token(&self, token_type: TokenType, token_subtype: TokenSubType) -> SyntaxToken {
    SyntaxToken::new(token_type, token_subtype, self.token_start_line_number, self.token_start_line_pos)
  }


  fn next_char(&mut self) -> Option<char> {
    match self.iter.next() {
      Some(ch) => {
        if ch == '\n' {
          self.cur_line_number += 1;
          self.cur_line_pos = 1;
        } else if ch == '\t' { // TODO: Handle better
          self.cur_line_pos += 4;
        }
        else {
          self.cur_line_pos += 1;
        }

        Some(ch)
      }
      None => None,
    }
  }

  fn skip_whitespace(&mut self) {
    loop {
      // workaround for multiple mutable borrows
      let mut value: Option<char>;
      {
        value = match self.iter.peek() {
          Some(ch) => Some(*ch),
          None => None,
        }
      }

      match value {
        Some(ch) => match ch {
          ' ' | '\n' | '\t' => self.next_char(),
          _ => break,
        },
        None => break,
      };
    }
  }
}
