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
  } else {
    Err(format!("Unexpected symbol {}", ch))
  }
}

fn starts_identifier(ch: char) -> bool {
  (ch >= 'a' && ch <= 'z') || (ch >= '0' && ch <= '9') || ch == '_'
}

fn handle_identifier(ch: char, iter: &mut str::Chars) -> Result<SyntaxToken, String> {
  let mut value: String = ch.to_string();

  loop {
    match iter.next() {
      Some(ch) => {
        if (ch == ' ') {
          break;
        }
        value.push(ch);
      }
      None => break,
    }
  }

  Ok(SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, value))
}


#[test]
fn lexer_handles_empty_string_correctly() {
  match tokenize("") {
    Ok(tokens) => assert_eq!(0, tokens.token_count()),
    Err(..) => assert!(false),
  }
}

#[test]
fn lexer_lexes_identifier_correctly() {
  let ident = "_this_is_an_identifier";

  match tokenize(ident) {
    Ok(tokens) => {
      assert_eq!(1, tokens.token_count());
      let expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, ident.to_string());
      match tokens.peek() {
        Some(actual) => { println!("{}", actual); assert_eq!(expected, *actual); },
        None => assert!(false),
      }
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn lexer_lexes_multiple_identifier_correctly() {
  let ident1 = "_this_is_an_identifier";
  let ident2 = "ident2345";
  let src = format!("{} {}", ident1, ident2);

  match tokenize(src.as_slice()) {
    Ok(mut tokens) => {
      assert_eq!(2, tokens.token_count());
      let first_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, ident1.to_string());
      let second_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier, ident2.to_string());

      match tokens.pop() {
        Some(actual) => { println!("{}", actual); assert_eq!(first_expected, *actual); },
        None => assert!(false),
      }

      match tokens.pop() {
        Some(actual) => { println!("{}", actual); assert_eq!(second_expected, *actual); },
        None => assert!(false),
      }

    }
    Err(..) => assert!(false)
  }
}
