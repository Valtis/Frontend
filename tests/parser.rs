extern crate compiler;

use compiler::lexer::tokenize;
use compiler::parser::parse;


#[test]
fn parser_accepts_parameterless_function_with_empty_block() {
  let tokens = tokenize("fn func() { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false),
  }
}

#[test]
fn parser_errors_on_parameterless_function_without_identifier() {
  let tokens = tokenize("fn () { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}


#[test]
fn parser_errors_on_parameterless_function_without_opened_block() {
  let tokens = tokenize("fn func()  }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn parser_errors_on_parameterless_function_without_closed_block() {
  let tokens = tokenize("fn func() { ").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}


#[test]
fn parser_errors_on_parameterless_function_without_left_parenthesis() {
  let tokens = tokenize("fn func) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}


#[test]
fn parser_errors_on_parameterless_function_without_right_parenthesis() {
  let tokens = tokenize("fn func( { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}
