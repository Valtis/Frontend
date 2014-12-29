
use token::Tokens;
use token::SyntaxToken;
use token::TokenType;
use token::TokenSubType;
use std::str;
use std::char;
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
        tokens.push(try!(create_token(ch, &mut iter)));
      }
      None => break
    }
  }

  Ok(tokens)
}


fn create_token(ch: char, iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {
  if starts_operator(ch) {
    handle_operators(ch, iter)
  } else if starts_identifier(ch) {
    handle_identifier(ch, iter)
  }/* else if starts_number(ch) {
    handle_number(ch, iter)
  } else if starts_string(ch) {
    handle_string(ch, iter)
  }*/
  else {
    Err(format!("Unexpected symbol {}", ch))
  }
}

fn starts_operator(ch: char) -> bool {
  match ch {
    '+' | '-' | '*' | '/' => true,
    _ => false,
  }
}

fn handle_operators(ch: char, iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {
  // Todo: Add support for += and !=
  match ch {
    '+' => Ok(SyntaxToken::new(TokenType::ArithOp, TokenSubType::Plus)),
    '-' => Ok(SyntaxToken::new(TokenType::ArithOp, TokenSubType::Minus)),
    '*' => Ok(SyntaxToken::new(TokenType::ArithOp, TokenSubType::Multiply)),
    '/' => Ok(SyntaxToken::new(TokenType::ArithOp, TokenSubType::Divide)),
    _ => Err(format!("Not an operator: {}", ch))
  }
}


fn starts_identifier(ch: char) -> bool {
  char::is_alphabetic(ch) || ch == '_'
}

fn is_valid_identifier_character(ch: char) -> bool {
  char::is_alphanumeric(ch) || ch == '_'
}

fn handle_identifier(ch: char, iter: &mut iter::Peekable<char, str::Chars>) -> Result<SyntaxToken, String> {

  let mut identifier: String = ch.to_string();

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

  Ok(SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(identifier)))
}

/*
fn starts_number(ch: char) -> bool {
  is_number(ch) || ch == '.'
}


fn starts_string(ch: char) -> bool {
  ch == '"'
}



fn is_number(ch: char) -> bool {
  (ch >= '0' && ch <= '9')
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

fn handle_string(ch: char, iter: &mut str::Chars) -> Result<SyntaxToken, String> {

  let mut value: String = String::new();
  let mut escape_char_read = false;
  loop {
    match iter.next() {
      Some(ch) => {

          if !escape_char_read && ch == '\\' {
            escape_char_read = true;
            continue;
          }

          if escape_char_read {
            escape_char_read = false;

            match ch {
              '"' => value.push('"'),
              _ => return Err(format!("Invalid escape sequence \\{}", ch)),

            }
            continue;
          }

          if ch == '"' {
            break;
          }

          value.push(ch);

        }
        None => return Err("Unterminated string".to_string()),
      }
    }

  Ok(SyntaxToken::new(TokenType::Text, TokenSubType::Text, value))
}


fn gather_characters(ch: char,
    iter: &mut str::Chars,
    checker: |char| -> bool,
    err_msg: &str) -> Result<String, String> {

  let mut value: String = ch.to_string();

  loop {
    match iter.next() {
      Some(ch) => {

        if checker(ch) {
          value.push(ch);
        } else {
          if ch == ' ' {
            break;
          }
          return Err(format!("{}: {}", err_msg, ch));
        }
      }
      None => break,
    }
  }

  Ok(value)
}


*/










#[test]
fn arithmetic_operators_are_tokenized_correctly() {

  let value = "+-*/\t/\n*    -       +";

  match tokenize(value) {
    Ok(mut tokens) => {
      assert_eq!(8, tokens.token_count());
      assert!(operator_helper(&mut tokens, TokenSubType::Plus));
      assert!(operator_helper(&mut tokens, TokenSubType::Minus));
      assert!(operator_helper(&mut tokens, TokenSubType::Multiply));
      assert!(operator_helper(&mut tokens, TokenSubType::Divide));
      assert!(operator_helper(&mut tokens, TokenSubType::Divide));
      assert!(operator_helper(&mut tokens, TokenSubType::Multiply));
      assert!(operator_helper(&mut tokens, TokenSubType::Minus));
      assert!(operator_helper(&mut tokens, TokenSubType::Plus));
    }
    Err(..) => assert!(false)
  }
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
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(ident.to_string()));
      assert!(identifier_helper(&mut tokens, &expected));
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
      let first_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(ident1.to_string()));
      let second_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(ident2.to_string()));

      assert!(identifier_helper(&mut tokens, &first_expected));
      assert!(identifier_helper(&mut tokens, &second_expected));
    }
    Err(..) => assert!(false)
  }
}


#[test]
fn lexer_tokenizes_identfiers_separated_by_operators_correctly() {
  let ident1 = "_this_is_an_identifier";
  let ident2 = "ident2345";
  let ident3 = "iddqd";
  let src = format!("{}+{}* {}", ident1, ident2, ident3);

  match tokenize(src.as_slice()) {
    Ok(mut tokens) => {
      assert_eq!(5, tokens.token_count());
      let first_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(ident1.to_string()));
      let second_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(ident2.to_string()));
      let third_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(ident3.to_string()));

      assert!(identifier_helper(&mut tokens, &first_expected));
      assert!(operator_helper(&mut tokens, TokenSubType::Plus));
      assert!(identifier_helper(&mut tokens, &second_expected));
      assert!(operator_helper(&mut tokens, TokenSubType::Multiply));
      assert!(identifier_helper(&mut tokens, &third_expected));

    }
    Err(..) => assert!(false)
  }

}

#[test]
fn invalid_identifier_character_causes_an_error() {
  let not_ident = "abc#!½!";

  match tokenize(not_ident) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }

}

#[test]
fn lexer_tokenizes_multiple_identifiers_with_lots_of_whitespace_between_correctly() {
  let ident1 = "_this_is_an_identifier";
  let ident2 = "ident2345";
  let src = format!("    {}\t\n  {}         ", ident1, ident2);

  match tokenize(src.as_slice()) {
    Ok(mut tokens) => {
      assert_eq!(2, tokens.token_count());
      let first_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(ident1.to_string()));
      let second_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(ident2.to_string()));

      assert!(identifier_helper(&mut tokens, &first_expected));
      assert!(identifier_helper(&mut tokens, &second_expected));
    }
    Err(..) => assert!(false)
  }
}

/*
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
  let ident1 = "_this_IS_an_Identifier";
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
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn number_starting_with_dot_and_with_multiple_dots_causes_an_error() {
  let not_number = ".123.4";

  match tokenize(not_number) {
    Ok(..) => assert!(false),
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
    Ok(..) => assert!(false),
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

#[test]
fn string_is_tokenized_correctly() {
  let string = "\"this is text\"";
  match tokenize(string) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Text, TokenSubType::Text, "this is text".to_string());
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn string_followed_by_identifier_without_whitespace_is_tokenized_correctly() {
  let string = "\"this is text 1234 _ öö\"and_this_is_identifier";

  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(2, tokens.token_count());
      let first_expected = SyntaxToken::new(TokenType::Text, TokenSubType::Text("this is text 1234 _ öö".to_string()));
      let second_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("and_this_is_identifier".to_string()));

      match tokens.pop() {
        Some(actual) => assert_eq!(first_expected, *actual),
        None => assert!(false),
      }

      match tokens.pop() {
        Some(actual) => assert_eq!(second_expected, *actual),
        None => assert!(false),
      }
    }
    Err(err) => { println!("{}", err); assert!(false); },
  }
}


#[test]
fn string_with_escaped_quote_is_tokenized_correctly() {
  let string = "\"this is text with \\\" an escaped quote\"";

  match tokenize(string) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Text, TokenSubType::Text("this is text with \" an escaped quote".to_string()));
      match tokens.peek() {
        Some(actual) => assert_eq!(expected, *actual),
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn string_with_invalid_escape_sequence_causes_an_error() {
  let err_string = "\"This string has invalid escape sequence\\!\"";
  match tokenize(err_string) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn unterminated_string_causes_an_error() {
  let err_string = "\"this is an error";
  match tokenize(err_string) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}*/

fn operator_helper(tokens: &mut Tokens, subtype:TokenSubType) -> bool {
  match tokens.expect(TokenType::ArithOp) {
    Ok(token) => token.t_subtype == subtype,
    Err(..) => false,
  }
}


fn identifier_helper(tokens: &mut Tokens, expected: &SyntaxToken) -> bool {
  match tokens.next() {
    Some(actual) => expected == actual,
    None => false
  }
}
