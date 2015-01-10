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



#[test]
fn parser_accepts_function_with_single_parameter_correctly() {
  let tokens = tokenize("fn func(a:int) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}

#[test]
fn parser_accepts_function_with_multiple_parameters_correctly() {
  let tokens = tokenize("fn func(a:int, b:double, c:float, d:bool) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}


#[test]
fn parser_errors_on_function_with_missing_parameter() {
  let tokens = tokenize("fn func(a:int, ) {}").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}


#[test]
fn parser_errors_on_function_with_parameter_separator_but_no_parameters() {
  let tokens = tokenize("fn func(,)").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}


#[test]
fn parser_errors_on_function_with_parameter_missing_type() {
  let tokens = tokenize("fn func(a:int, b:double, c, d:bool)").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}

#[test]
fn parser_errors_on_function_with_parameter_missing_colon() {
  let tokens = tokenize("fn func(a:int, bdouble )").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}

#[test]
fn parser_errors_on_function_with_parameters_and_missing_left_parenthesis() {
  let tokens = tokenize("fn func a:int, b:double, c:float, d:bool) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}

#[test]
fn parser_errors_on_function_with_parameters_and_missing_right_parenthesis() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}

#[test]
fn parse_parses_single_variable_declaration_with_constant_value_correctly() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) { let a:int = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}

#[test]
fn parse_parses_multiple_variable_declarations_with_constant_values_correctly() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool)
    { let a:int = 5; let b:double = 0.434; let c:float = .343f;
    let d:string = \"dasdad\"; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}

#[test]
fn parse_errors_on_variable_declaration_with_missing_semicolon() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) { let a:int = 5 }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}

#[test]
fn parse_errors_on_variable_declaration_with_missing_type() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) { let a = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}

#[test]
fn parse_errors_on_variable_declaration_with_missing_name() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) { let :int = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}

#[test]
fn parse_errors_on_variable_declaration_with_missing_colon() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) { let aint = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}


#[test]
fn parse_errors_on_variable_declaration_with_missing_let() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) { a:int = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true)
  }
}

#[test]
fn parser_parses_blocks_inside_blocks_correctly() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) { let a:int = 5;  { let b:double = \"as\"; { } { let c:float=.232; }}}").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}

#[test]
fn parser_gives_correct_error_message_on_invalid_function_definition() {
  let tokens = tokenize("invalid_dec(b:int) { }\nfn func (a:int) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(1, err.len());
      assert!(err[0].contains("1:1"));
    }
  }
}

#[test]
fn parser_gives_correct_error_message_on_invalid_function_argument_definition_and_invalid_variable_declaration() {
  let tokens = tokenize("fn invalid_dec(b:int, ) {\n let a:= 5; }\nfn func (a:int) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(2, err.len());
      assert!(err[0].contains("1:23"));
      assert!(err[1].contains("2:8"));
    }
  }
}

#[test]
fn parser_gives_correct_error_message_on_invalid_function_definition_and_invalid_variable_declaration_in_next_function() {
  let tokens = tokenize("invalid_dec(b:int) {\n }\nfn func (a:int) {\n let a; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(2, err.len());
      assert!(err[0].contains("1:1"));
      assert!(err[1].contains("4:7"));
    }
  }
}
