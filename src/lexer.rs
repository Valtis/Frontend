use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;
use std::str;


pub fn tokenize(content: &str) -> Result<Tokens, String> {
  let mut tokens = Tokens::new();

  let mut iter = content.chars();
  loop {
    match iter.next() {
      Some(ch) => {
        if ch == ' ' {
          continue;
        }

        let token = try!(create_token(ch, &mut iter));
        tokens.push(token);
      },
      None => break,
    }

  }

  Ok(tokens)
}


fn create_token(ch: char, iter: &mut str::Chars) -> Result<SyntaxToken, String> {
  if starts_identifier(ch) {
    handle_identifier(ch, iter)
  } else if starts_number(ch) {
    handle_number(ch, iter)
  }
  else {
    Err(format!("Unexpected symbol {}", ch))
  }
}

fn starts_identifier(ch: char) -> bool {
  (ch >= 'a' && ch <= 'z') || ch == '_'
}

fn starts_number(ch: char) -> bool {
  is_number(ch) || ch == '.'
}

fn valid_identifier_character(ch: char) -> bool {
  starts_identifier(ch) || is_number(ch)
}

fn is_number(ch: char) -> bool {
  (ch >= '0' && ch <= '9')
}

fn handle_identifier(ch: char, iter: &mut str::Chars) -> Result<SyntaxToken, String> {
  let value = try!(gather_characters(ch, iter, valid_identifier_character, "Not valid integer character"));

  Ok(SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, value))
}

fn handle_number(ch: char, iter: &mut str::Chars) -> Result<SyntaxToken, String> {

  // uh, kinda starting to feel hacky/stupid. Probably should reconsider this

  let mut subtype = TokenSubType::IntegerNumber;
  let mut encourtered_dot = false;
  let mut encourtered_type_char = false;
  let mut value: String;

  if ch == '.' {
    subtype = TokenSubType::DoubleNumber;
    encourtered_dot = true;
  }

  {

    let number_checker = |ch: char| {

      if encourtered_type_char {
        return false;
      }

      if ch == 'd' {
        subtype = TokenSubType::DoubleNumber;
        encourtered_type_char = true;
        true
      } else if ch == 'f' {
        subtype = TokenSubType::FloatNumber;
        encourtered_type_char = true;
        true
      }
      else if ch == '.' {
        if !encourtered_dot {
          encourtered_dot = true;
          subtype = TokenSubType::DoubleNumber;
          true
        } else {
          false
        }
      } else {
        is_number(ch)
      }
    };

    value = try!(gather_characters(ch, iter, number_checker, "Not a valid number"));
  }

  if encourtered_type_char {
    value.pop();
  }

  Ok(SyntaxToken::new(TokenType::Number, subtype, value))
}


fn gather_characters(ch: char,
    iter: &mut str::Chars,
    checker: |char| -> bool,
    err_msg: &str) -> Result<String, String> {

  let mut value: String = ch.to_string();

  loop {
    match iter.next() {
      Some(ch) => {

        if ch == ' ' {
          break;
        }

        if checker(ch) {
          value.push(ch);
        } else {
          return Err(format!("{}: {}", err_msg, ch));
        }
      }
      None => break,
    }
  }

  Ok(value)
}

#[test]
fn lexer_handles_empty_string_correctly() {
  match tokenize("") {
    Ok(tokens) => assert_eq!(0, tokens.token_count()),
    Err(..) => assert!(false),
  }
}

#[test]
fn lexer_tokenizes_identifier_correctly() {
  let ident = "_this_is_an_identifier";

  match tokenize(ident) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, ident.to_string());
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn lexer_tokenizes_multiple_identifier_correctly() {
  let ident1 = "_this_is_an_identifier";
  let ident2 = "ident2345";
  let src = format!("{} {}", ident1, ident2);

  match tokenize(src.as_slice()) {
    Ok(mut tokens) => {
      assert_eq!(2, tokens.token_count());
      let first_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, ident1.to_string());
      let second_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, ident2.to_string());

      match tokens.pop() {
        Some(actual) => assert_eq!(first_expected, *actual),
        None => assert!(false),
      }

      match tokens.pop() {
        Some(actual) => assert_eq!(second_expected, *actual),
        None => assert!(false),
      }

    }
    Err(..) => assert!(false)
  }
}

#[test]
fn invalid_identifier_character_causes_an_error() {
  let not_ident = "abc#!½!";

  match tokenize(not_ident) {
    Ok(tokens) => assert!(false),
    Err(..) => assert!(true),
  }

}

#[test]
fn lexer_tokenizes_multiple_identifiers_with_lots_of_whitespace_between_correctly() {
  let ident1 = "_this_is_an_identifier";
  let ident2 = "ident2345";
  let src = format!("    {}   {}         ", ident1, ident2);

  match tokenize(src.as_slice()) {
    Ok(mut tokens) => {
      assert_eq!(2, tokens.token_count());
      let first_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, ident1.to_string());
      let second_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, ident2.to_string());

      match tokens.pop() {
        Some(actual) => assert_eq!(first_expected, *actual),
        None => assert!(false),
      }

      match tokens.pop() {
        Some(actual) => assert_eq!(second_expected, *actual),
        None => assert!(false),
      }

    }
    Err(..) => assert!(false)
  }
}

#[test]
fn lexer_tokenizes_integer_correctly() {
  let integer = "12431";

  match tokenize(integer) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Number, TokenSubType::IntegerNumber, integer.to_string());
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn lexer_tokenizes_identifier_and_integer_correctly() {
  let ident1 = "_this_is_an_identifier";
  let integer = "03290";
  let src = format!("{} {}", ident1, integer);

  match tokenize(src.as_slice()) {
    Ok(mut tokens) => {
      assert_eq!(2, tokens.token_count());
      let first_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, ident1.to_string());
      let second_expected = SyntaxToken::new(TokenType::Number, TokenSubType::IntegerNumber, integer.to_string());

      match tokens.pop() {
        Some(actual) => assert_eq!(first_expected, *actual),
        None => assert!(false),
      }

      match tokens.pop() {
        Some(actual) => assert_eq!(second_expected, *actual),
        None => assert!(false),
      }

    }
    Err(..) => assert!(false)
  }
}

#[test]
fn lexer_tokenizes_double_correctly() {
  let double = "124.314";

  match tokenize(double) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber, double.to_string());
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn lexer_tokenizes_double_that_starts_with_dot_correctly() {
  let double = ".314";

  match tokenize(double) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber, double.to_string());
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn number_with_multiple_dots_causes_an_error() {
  let not_number = "123.31.4";

  match tokenize(not_number) {
    Ok(tokens) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn number_starting_with_dot_and_with_multiple_dots_causes_an_error() {
  let not_number = ".123.4";

  match tokenize(not_number) {
    Ok(tokens) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn integer_with_double_identifier_char_is_tokenized_correctly() {
  let number = "12431d";

  match tokenize(number) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber, "12431".to_string());
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn double_number_with_double_identifier_char_is_tokenized_correctly() {
  let double = ".314d";

  match tokenize(double) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber, ".314".to_string());
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn number_with_identifier_char_in_wrong_spot_causes_an_error() {
  let not_number = ".314d124";

  match tokenize(not_number) {
    Ok(tokens) => assert!(false),
    Err(..) => assert!(true)
  }
}

#[test]
fn integer_with_float_identifier_char_is_tokendized_correctly() {
  let number = "12431f";

  match tokenize(number) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Number, TokenSubType::FloatNumber, "12431".to_string());
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn decimal_number_with_float_identifier_char_is_tokenized_correctly() {
  let double = ".314f";

  match tokenize(double) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Number, TokenSubType::FloatNumber, ".314".to_string());
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}
