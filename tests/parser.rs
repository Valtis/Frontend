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
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:4"));
    }
  }
}


#[test]
fn parser_errors_on_parameterless_function_without_opened_block() {
  let tokens = tokenize("fn func() }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:11"));
    }
  }
}

#[test]
fn parser_errors_on_parameterless_function_without_closed_block() {
  let tokens = tokenize("fn func() { ").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("end-of-file"));
    }
  }
}


#[test]
fn parser_errors_on_parameterless_function_without_left_parenthesis() {
  let tokens = tokenize("fn func) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:8"));
    }
  }
}


#[test]
fn parser_errors_on_parameterless_function_without_right_parenthesis() {
  let tokens = tokenize("fn func( { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:10"));
    }
  }
}



#[test]
fn parser_accepts_function_with_single_parameter() {
  let tokens = tokenize("fn func(a:int) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}

#[test]
fn parser_accepts_function_with_multiple_parameters() {
  let tokens = tokenize("fn func(a:int, b:double, c:float, d:bool) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}

#[test]
fn parser_accepts_function_with_void_type() {
  let tokens = tokenize("fn func(a:int, b:double, c:float, d:bool) : void { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}


#[test]
fn parser_accepts_function_with_int_type() {
  let tokens = tokenize("fn func(a:int, b:double, c:float, d:bool) : int { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}


#[test]
fn parser_accepts_function_with_bool_type() {
  let tokens = tokenize("fn func(a:int, b:double, c:float, d:bool) : bool { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}


#[test]
fn parser_accepts_function_with_string_type() {
  let tokens = tokenize("fn func(a:int, b:double, c:float, d:bool) : string { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false)
  }
}

#[test]
fn parser_errors_on_function_with_void_parameter() {
  let tokens = tokenize("fn func(a:int, b:void) {}").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:18"));
    }
  }
}

#[test]
fn parser_errors_on_function_with_missing_parameter() {
  let tokens = tokenize("fn func(a:int, ) {}").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:16"));
    }
  }
}


#[test]
fn parser_errors_on_function_with_parameter_separator_but_no_parameters() {
  let tokens = tokenize("fn func(,) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:9"));
    }
  }
}


#[test]
fn parser_errors_on_function_with_parameter_missing_type_and_colon() {
  let tokens = tokenize("fn func(a:int, b:double, c, d:bool) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:27"));
    }
  }
}

#[test]
fn parser_errors_on_function_with_parameter_missing_type() {
  let tokens = tokenize("fn func(a:int, b:double, c:, d:bool) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:28"));
    }
  }
}

#[test]
fn parser_errors_on_function_with_parameter_missing_colon() {
  let tokens = tokenize("fn func(a:int, bdouble ) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:24"));
    }
  }
}

#[test]
fn parser_errors_on_function_with_parameters_and_missing_left_parenthesis() {
  let tokens = tokenize("fn func a:int, b:double, c:float, d:bool) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:9"));
    }
  }
}

#[test]
fn parser_errors_on_function_with_parameters_and_missing_right_parenthesis() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("1:43"));
    }
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
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) {\n  let a:int = 5 }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("2:17"));
    }
  }
}

#[test]
fn parse_errors_on_variable_declaration_with_missing_type() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) {\n  let a = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("2:9"));
    }
  }
}

#[test]
fn parse_errors_on_variable_declaration_with_missing_name() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) {\n  let :int = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("2:7"));
    }
  }
}

#[test]
fn parse_errors_on_variable_declaration_with_missing_colon() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) {\n  let aint = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("2:12"));
    }
  }
}

#[test]
fn parser_errors_on_variable_declaration_with_void_type() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) {\n  let a:void = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("2:9"));
    }
  }
}

#[test]
fn parse_errors_on_variable_declaration_with_missing_let() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) {\n  a:int = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("2:3"));
    }
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
fn parser_gives_corret_error_messages_on_two_different_invalid_function_declarations() {
  let tokens = tokenize("fn invalid_dec(b:int, ) {\n let a:int = 5; }\nfn (a:int) { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(2, err.len());
      assert!(err[0].contains("1:23"));
      assert!(err[1].contains("3:4"));
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


#[test]
fn parser_accepts_arithmetic_expression() {
  let tokens = tokenize("fn foo() { let a:int = 4 + +2 - -5 + 6*(7+1) - b; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(err) => assert!(false),
  }
}

#[test]
fn parser_errors_on_arithmetic_expression_with_missing_operator() {
  let tokens = tokenize("fn foo() { let a:int = 5 6*(7+1) - b; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {

      println!("{}", err[0]);
      assert_eq!(1, err.len());
      assert!(err[0].contains("1:26"));
    }
  }
}

#[test]
fn parser_errors_on_arithmetic_expression_with_too_many_left_parenthesis() {
  let tokens = tokenize("fn foo() { let a:int = 5 + 6*((7+1) - b; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(1, err.len());
      assert!(err[0].contains("1:40"));
    }
  }
}

#[test]
fn parser_errors_on_arithmetic_expression_with_too_many_right_parenthesis() {
  let tokens = tokenize("fn foo() { let a:int = 5 + 6*(7+1)) - b; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(1, err.len());
      assert!(err[0].contains("1:35"));
    }
  }
}



#[test]
fn parser_gives_correct_error_messages_on_two_different_arithmetic_expression_with_errors() {
  let tokens = tokenize("fn foo() { let a:int = 5 + 6*(7+1)) - b;\nlet b:int = 5 }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(2, err.len());
      assert!(err[0].contains("1:35"));
      assert!(err[1].contains("2:15"));
    }
  }
}
