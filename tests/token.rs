extern crate compiler;

use compiler::token::SyntaxToken;
use compiler::token::TokenType;
use compiler::token::TokenSubType;
use compiler::token::Tokens;

#[test]
fn can_create_new_tokens() {
  let token = SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0);
  assert_eq!(TokenType::If, token.t_type);
  assert_eq!(TokenSubType::NoSubType, token.t_subtype);
}

#[test]
fn floating_token_equality_works() {
  let first_token = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber(0.1234), 0, 0);
  let second_token = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber(0.1234), 0, 0);
  let third_token = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber(12.12), 0, 0);
  assert_eq!(first_token, second_token);
  assert_eq!(second_token, first_token);
  assert!(first_token != third_token);

}

#[test]
fn peek_on_empty_token_queue_returns_none() {
  let tokens = Tokens::new();

  match tokens.peek() {
    Some(..) => assert!(false),
    None => assert!(true),
  }
}

#[test]
fn peek_retuns_the_wanted_token_from_queue() {
  let tokens = create_queue();
  let expected = SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0);

  match tokens.peek() {
    Some(actual) => assert_eq!(expected, *actual),
    None => assert!(false),
  }
}

#[test]
fn calling_peek_multiple_times_does_not_advance_the_queue() {
  let tokens = create_queue();
  let expected = SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0);

  tokens.peek();
  tokens.peek();
  tokens.peek();
  tokens.peek();

  match tokens.peek() {
    Some(actual) => assert_eq!(expected, *actual),
    None => assert!(false),
  }
}

#[test]
fn next_on_empty_queue_returns_none() {

  let mut tokens = Tokens::new();

  match tokens.next() {
    Some(..) => assert!(false),
    None => assert!(true),
  }
}

#[test]
fn next_advances_queue() {

  let mut tokens = create_queue();

  let first_expected = SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0);
  let second_expected = SyntaxToken::new(TokenType::LParen, TokenSubType::NoSubType, 0, 0);
  let third_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("abc".to_string()), 0, 0);

  match tokens.next() {
    Some(actual) => assert_eq!(first_expected, *actual),
    None => assert!(false),
  }

  match tokens.next() {
    Some(actual) => assert_eq!(second_expected, *actual),
    None => assert!(false),
  }

  match tokens.next() {
    Some  (actual) => assert_eq!(third_expected, *actual),
    None => assert!(false),
  }
}


#[test]
fn expect_on_empty_queue_returns_error() {
  let mut tokens = Tokens::new();

  match tokens.expect(TokenType::If) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn expect_with_wrong_token_type_returns_error() {
  let mut  tokens = create_queue();

  match tokens.expect(TokenType::While) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn expect_with_wrong_token_subtype_returns_error() {
  let mut tokens = create_queue();

  match tokens.expect(TokenType::SemiColon) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn expect_with_right_values_returns_the_token() {
  let mut  tokens = create_queue();

  let expected = SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0);

  match tokens.expect(TokenType::If) {
    Ok(actual) => assert_eq!(expected, *actual),
    Err(..) => assert!(false),
  }
}

#[test]
fn expect_advances_queue() {
  let mut tokens = create_queue();

  let first_expected = SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0);
  let second_expected = SyntaxToken::new(TokenType::LParen, TokenSubType::NoSubType, 0, 0);
  let third_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("abc".to_string()), 0, 0);

  match tokens.expect(TokenType::If) {
    Ok(actual) => assert_eq!(first_expected, *actual),
    Err(..) => assert!(false),
  }

  match tokens.expect(TokenType::LParen) {
    Ok(actual) => assert_eq!(second_expected, *actual),
    Err(..) => assert!(false),
  }

  match tokens.expect(TokenType::Identifier) {
    Ok(actual) => assert_eq!(third_expected, *actual),
    Err(..) => assert!(false),
  }
}


#[test]
fn expect_failure_followed_by_correct_values_advances_queue() {

  let mut tokens = create_queue();

  let first_expected = SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0);
  let second_expected = SyntaxToken::new(TokenType::LParen, TokenSubType::NoSubType, 0, 0);


  match tokens.expect(TokenType::Identifier) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }

  match tokens.expect(TokenType::If) {
    Ok(actual) => assert_eq!(first_expected, *actual),
    Err(..) => assert!(false),
  }

  match tokens.expect(TokenType::LParen) {
    Ok(actual) => assert_eq!(second_expected, *actual),
    Err(..) => assert!(false),
  }
}

#[test]
fn peek_returns_correct_value_after_pops_and_expects() {
  let mut tokens = create_queue();

  let first_expected = SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0);
  let second_expected = SyntaxToken::new(TokenType::LParen, TokenSubType::NoSubType, 0, 0);
  let third_expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("abc".to_string()), 0, 0);

  match tokens.expect(TokenType::If) {
    Ok(actual) => assert_eq!(first_expected, *actual),
    Err(..) => assert!(false),
  }

  match tokens.next() {
    Some(actual) => assert_eq!(second_expected, *actual),
    None => assert!(false),
  }

  match tokens.peek() {
    Some(actual) => assert_eq!(third_expected, *actual),
    None => assert!(false),
  }
}

fn create_queue() -> Tokens {

  let mut tokens = Tokens::new();

  // push tokens if (abc == 5) { def = 2; }
  tokens.push(SyntaxToken::new(TokenType::If, TokenSubType::NoSubType, 0, 0));
  tokens.push(SyntaxToken::new(TokenType::LParen, TokenSubType::NoSubType, 0, 0));
  tokens.push(SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("abc".to_string()), 0, 0));
  tokens.push(SyntaxToken::new(TokenType::CompOp, TokenSubType::Equals, 0, 0));
  tokens.push(SyntaxToken::new(TokenType::Number, TokenSubType::IntegerNumber(5), 0, 0));
  tokens.push(SyntaxToken::new(TokenType::RParen, TokenSubType::NoSubType, 0, 0));
  tokens.push(SyntaxToken::new(TokenType::LBrace, TokenSubType::NoSubType, 0, 0));
  tokens.push(SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier("def".to_string()), 0, 0));
  tokens.push(SyntaxToken::new(TokenType::Assign, TokenSubType::NoSubType, 0, 0));
  tokens.push(SyntaxToken::new(TokenType::Number, TokenSubType::IntegerNumber(2), 0, 0));
  tokens.push(SyntaxToken::new(TokenType::SemiColon, TokenSubType::NoSubType, 0, 0));
  tokens.push(SyntaxToken::new(TokenType::RBrace, TokenSubType::NoSubType, 0, 0));

  tokens
}