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
      assert_eq!(2, errors.len());
      assert!(errors[0].contains("1:9"));
      assert!(errors[1].contains("1:10"));
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
#[test]
fn parser_errors_with_correct_errors_with_multiple_errors_in_declaration() {
  let tokens = tokenize("fn  (aint, b:, float, d:bool { }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(5, errors.len());
      assert!(errors[0].contains("1:5"));
      assert!(errors[1].contains("1:10"));
      assert!(errors[2].contains("1:14"));
      assert!(errors[3].contains("1:16"));
      assert!(errors[4].contains("1:30"));
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
fn parse_errors_on_variable_declaration_with_missing_let() {
  let tokens = tokenize("fn func (a:int, b:double, c:float, d:bool) {\n  a:int = 5; }").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(errors) => {
      assert_eq!(1, errors.len());
      assert!(errors[0].contains("2:4"));
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

#[test]
fn parser_accepts_variable_assignments() {
  let tokens = tokenize("fn foo() { \na = 5;\nb = 4*a-5*(3*(7-4)); }  ").unwrap();

  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false),
  }
}

#[test]
fn parser_gives_correct_error_messages_on_invalid_assignments() {

  let tokens = tokenize("fn foo() { \ninvalid = ;\ncorrect=23;\ninvalid 4*a-5*(3*(7-4));\ncorrect=321;\ninvalid=23 }").unwrap();
  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(3, err.len());
      assert!(err[0].contains("2:11"));
      assert!(err[1].contains("4:9"));
      assert!(err[2].contains("6:12"));
    }
  }
}

#[test]
fn parser_accepts_function_call_syntax() {
  let tokens = tokenize("fn foo() { bar(); bar(1); bar(5, 6, 7, 8); bar(5*5+a-b/C, 2); }").unwrap();
  match parse(tokens) {
    Ok(..) => assert!(true),
    Err(..) => assert!(false),
  }
}

#[test]
fn parser_errors_on_missing_left_parenthesis_with_function_call() {
  let tokens = tokenize("fn foo() { \nbar5); }").unwrap();
  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(1, err.len());
      assert!(err[0].contains("2:5"));
    }
  }
}

#[test]
fn parser_errors_on_missing_right_parenthesis_with_function_call() {
  let tokens = tokenize("fn foo() { \nbar(5; }").unwrap();
  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(1, err.len());
      assert!(err[0].contains("2:6"));
    }
  }
}

#[test]
fn parser_errors_on_missing_identifier_with_function_call() {
  let tokens = tokenize("fn foo() { \n(5); }").unwrap();
  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(1, err.len());
      assert!(err[0].contains("2:1"));
    }
  }
}

#[test]
fn parser_errors_on_missing_parameter_with_function_call() {
  let tokens = tokenize("fn foo() { \nbar(5,); }").unwrap();
  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(1, err.len());
      assert!(err[0].contains("2:7"));
    }
  }
}

#[test]
fn parser_errors_when_only_comma_present_with_function_call() {
  let tokens = tokenize("fn foo() { \nbar(,); }").unwrap();
  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(2, err.len());
      assert!(err[0].contains("2:5"));
      assert!(err[1].contains("2:6"));
    }
  }
}

#[test]
fn parser_gives_error_messages_for_multiple_issues_with_arguments() {
  let tokens = tokenize("fn foo() { \nbar(a+*3, a-b+, vava+,); }").unwrap();
  match parse(tokens) {
    Ok(..) => assert!(false),
    Err(err) => {
      assert_eq!(4, err.len());
      assert!(err[0].contains("2:7"));
      assert!(err[1].contains("2:15"));
      assert!(err[2].contains("2:22"));
      assert!(err[3].contains("2:23"));
    }
  }
}
