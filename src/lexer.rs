
use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;
use std::str;
use std::iter;

pub fn tokenize(content: &str) -> Result<Tokens, String> {

  let mut tokens = Tokens::new();

  let mut iter = content.chars().peekable();
  loop {
    match iter.next() {
      Some(ch) => {
        // skip whitespace
        match ch {
          ' ' | '\n' | '\t' => continue,
          _ => { /* do nothing*/}
        }
        tokens.push(try!(read_token(ch, &mut iter)));
      }
      None => break
    }
  }

  Ok(tokens)
}

fn create_token(token_type: TokenType, token_subtype: TokenSubType) -> SyntaxToken {
  SyntaxToken::new(token_type, token_subtype, 0, 0)
}

fn read_token(ch: char, iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {
  if starts_symbol(ch) {
    handle_symbols(ch, iter)
  } else if starts_identifier(ch) {
    handle_identifier(ch, iter)
  } else if starts_number(ch, iter) {
    handle_number(ch, iter)
  } else if starts_string(ch) {
    handle_string(iter)
  }
  else {
    Err(format!("Unexpected symbol {}", ch))
  }
}

fn starts_symbol(ch: char) -> bool {
  match ch {
    '+' | '-' | '*' | '/' | '[' | ']' | '{' | '}' | '(' | ')' => true,
    _ => false,
  }
}

fn handle_symbols(ch: char, iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {

  match ch {
    '+' => Ok(create_token(TokenType::ArithOp, TokenSubType::Plus)),
    '-' => Ok(create_token(TokenType::ArithOp, TokenSubType::Minus)),
    '*' => Ok(create_token(TokenType::ArithOp, TokenSubType::Multiply)),
    '/' => Ok(create_token(TokenType::ArithOp, TokenSubType::Divide)),
    '[' => Ok(create_token(TokenType::LBracket, TokenSubType::NoSubType)),
    ']' => Ok(create_token(TokenType::RBracket, TokenSubType::NoSubType)),
    '{' => Ok(create_token(TokenType::LBrace, TokenSubType::NoSubType)),
    '}' => Ok(create_token(TokenType::RBrace, TokenSubType::NoSubType)),
    '(' => Ok(create_token(TokenType::LParen, TokenSubType::NoSubType)),
    ')' => Ok(create_token(TokenType::RParen, TokenSubType::NoSubType)),
    _ => Err(format!("Not an operator: {}", ch))
  }
}


fn starts_identifier(ch: char) -> bool {
  ch.is_alphabetic() || ch == '_'
}

fn is_valid_identifier_character(ch: char) -> bool {
  ch.is_alphanumeric() || ch == '_'
}

fn handle_identifier(ch: char, iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {

  let mut identifier = ch.to_string();

  loop {
    // workaround for multiple mutable borrows
    let mut value: Option<char>;
    // new block so that mutable borrow ends before new borrow at iter.next()
    {
      value = match iter.peek() {
        Some(ch) => Some(*ch),
        None => None,
      }
    }

    match value {
      Some(ch) => {
        if is_valid_identifier_character(ch) {
          identifier.push(ch);
          iter.next();
        } else {
          break;
        }
      }
      None => break
    }
  }

  Ok(create_token(TokenType::Identifier, TokenSubType::Identifier(identifier)))
}


fn starts_number(ch: char, iter: &mut iter::Peekable<char, str::Chars>) -> bool {
  if ch.is_digit(10) {
    true
  } else if ch == '.' {
    match iter.peek() {
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

fn handle_number(ch: char, iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {

  let mut number_str = ch.to_string();

  if ch == '.' {
    return handle_decimal_number(number_str, iter);
  }
  loop {

    // workaround for multiple mutable borrows
    let mut value: Option<char>;
    // new block so that mutable borrow ends before new borrow at iter.next()
    {
      value = match iter.peek() {
        Some(ch) => Some(*ch),
        None => None,
      }
    }

    match value {
      Some(ch) => {
        if ch.is_digit(10) {
          number_str.push(ch);
          iter.next();
        } else if ch == '.' {
          number_str.push(ch);
          iter.next();
          return handle_decimal_number(number_str, iter);
        } else if ch.is_alphabetic() {
          iter.next();
          return handle_number_type_char(ch, number_str, iter);
        } else {
          break;
        }
      }
      None => break
    }

  }

  match number_str.parse() {
    Some(number) => Ok(create_token(TokenType::Number, TokenSubType::IntegerNumber(number))),
    None => Err("Internal error - non-numeric characters in number token".to_string()),
  }
}

fn handle_decimal_number(mut number_str: String, iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {
  loop {
    // workaround for multiple mutable borrows
    let mut value: Option<char>;
    // new block so that mutable borrow ends before new borrow at iter.next()
    {
      value = match iter.peek() {
        Some(ch) => Some(*ch),
        None => None,
      }
    }

    match value {
      Some(ch) => {
        if ch.is_digit(10) {
          number_str.push(ch);
          iter.next();
        } else if ch.is_alphabetic() {
          iter.next();
          return handle_number_type_char(ch, number_str, iter);
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
    Some(number) => Ok(create_token(TokenType::Number, TokenSubType::DoubleNumber(number))),
    None => Err("Internal error - non-numeric characters in number token".to_string()),
  }
}

fn handle_number_type_char(type_char: char, number_str: String, iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {
  // check that character following the type char is not alphanumeric
  match iter.peek() {
    Some(ch) => {
      if ch.is_alphanumeric() {
        return Err(format!("Invalid character following number type character: {}", ch));
      }
    }
    None => { /* do nothing */}
  }

  match type_char {
    'd' => match number_str.parse() {
      Some(number) => Ok(create_token(TokenType::Number, TokenSubType::DoubleNumber(number))),
      None => Err("Internal error - non-numeric characters in number token".to_string()),
    },
    'f' => match number_str.parse() {
      Some(number) => Ok(create_token(TokenType::Number, TokenSubType::FloatNumber(number))),
      None => Err("Internal error - non-numeric characters in number token".to_string()),
    },
    _ => Err(format!("Invalid type character: {}", type_char)),
  }

}


fn starts_string(ch: char) -> bool {
  ch == '"'
}

fn handle_string(iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {

  let mut value: String = String::new();

  loop {
    match iter.next() {
      Some(ch) => {
        if ch == '\\' {
          value.push(try!(handle_escape_sequence(iter)));
        } else if ch == '"' {
          // check that there are no alphanumeric characters following the '"'
          match iter.peek() {
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

  Ok(create_token(TokenType::Text, TokenSubType::Text(value)))
}

fn handle_escape_sequence(iter: &mut iter::Peekable<char, str::Chars>) -> Result<char, String> {
  match iter.next() {
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
