extern crate compiler;

use compiler::lexer::tokenize;
use compiler::token::SyntaxToken;
use compiler::token::TokenType;
use compiler::token::TokenSubType;
use compiler::token::Tokens;


#[test]
fn arithmetic_operators_are_tokenized_correctly() {

  let value = "+-*/\t/\n*    -       +";

  match tokenize(value) {
    Ok(mut tokens) => {
      assert_eq!(8, tokens.token_count());
      assert!(arith_op_helper(&mut tokens, TokenSubType::Plus));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Minus));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Multiply));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Divide));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Divide));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Multiply));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Minus));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Plus));
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
      assert!(identifier_helper(&mut tokens, ident));
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

      assert!(identifier_helper(&mut tokens, ident1));
      assert!(identifier_helper(&mut tokens, ident2));
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

      assert!(identifier_helper(&mut tokens, ident1));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Plus));
      assert!(identifier_helper(&mut tokens, ident2));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Multiply));
      assert!(identifier_helper(&mut tokens, ident3));

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

      assert!(identifier_helper(&mut tokens, ident1));
      assert!(identifier_helper(&mut tokens, ident2));
    }
    Err(..) => assert!(false)
  }
}


#[test]
fn lexer_tokenizes_integer_correctly() {
  let integer = "12431";

  match tokenize(integer) {
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());

      assert!(integer_helper(&mut tokens, 12431));

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
      assert!(identifier_helper(&mut tokens, ident1));
      assert!(integer_helper(&mut tokens, 3290));

    }
    Err(..) => assert!(false)
  }
}
#[test]
fn lexer_tokenizes_numbers_and_operators_correctly() {

  let src = "13*14+15 *16 / 16    -1";
  match tokenize(src.as_slice()) {
    Ok(mut tokens) => {
      assert_eq!(11, tokens.token_count());

      assert!(integer_helper(&mut tokens, 13));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Multiply));
      assert!(integer_helper(&mut tokens, 14));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Plus));
      assert!(integer_helper(&mut tokens, 15));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Multiply));
      assert!(integer_helper(&mut tokens, 16));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Divide));
      assert!(integer_helper(&mut tokens, 16));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Minus));
      assert!(integer_helper(&mut tokens, 1));

    }
    Err(..) => assert!(false)
  }
}


#[test]
fn lexer_tokenizes_double_correctly() {
  let double = "124.314";

  match tokenize(double) {
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      assert!(double_helper(&mut tokens, 124.314));
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn lexer_tokenizes_double_that_starts_with_dot_correctly() {
  let double = ".314";

  match tokenize(double) {
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      assert!(double_helper(&mut tokens, 0.314));
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
fn numer_with_invalid_identifier_char_causes_an_error() {
  let not_number = ".1234x";

  match tokenize(not_number) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn integer_with_invalid_type_character_causes_an_error() {
  let not_number = "133r";

  match tokenize(not_number) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}

#[test]
fn number_followed_by_identifier_that_starts_with_identifier_char_is_handled_correctly() {
  let src = ".1234 fluffy";

  match tokenize(src) {
    Ok(mut tokens) => {
      assert_eq!(2, tokens.token_count());
      assert!(double_helper(&mut tokens, 0.1234));
      assert!(identifier_helper(&mut tokens, "fluffy"));

    },

    Err(..) => assert!(false),
  }
}

#[test]
fn integer_with_double_identifier_char_is_tokenized_correctly() {
  let number = "12431d";

  match tokenize(number) {
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      assert!(double_helper(&mut tokens, 12431f64));
      },
    Err(..) => assert!(false),
  }
}


#[test]
fn double_number_with_double_identifier_char_is_tokenized_correctly() {
  let number = ".314d";

  match tokenize(number) {
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      assert!(double_helper(&mut tokens, 0.314));
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
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      assert!(float_helper(&mut tokens, 12431f32));
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn decimal_number_with_float_identifier_char_is_tokenized_correctly() {
  let number = ".314f";

  match tokenize(number) {
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      assert!(float_helper(&mut tokens, 0.314f32));
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn multiple_decimal_numbers_with_operators_works_correctly() {
  let src="1.23*32f + 12 + 1.343d * .123f";

  match tokenize(src) {
    Ok(mut tokens) => {
      assert_eq!(9, tokens.token_count());
      assert!(double_helper(&mut tokens, 1.23));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Multiply));
      assert!(float_helper(&mut tokens, 32f32));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Plus));
      assert!(integer_helper(&mut tokens, 12));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Plus));
      assert!(double_helper(&mut tokens, 1.343));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Multiply));
      assert!(float_helper(&mut tokens, 0.123f32));
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn string_is_tokenized_correctly() {
  let string = "\"this is text\"";
  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      assert!(string_helper(&mut tokens, "this is text"));
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn string_followed_by_identifier_is_handled_correctly() {
  let string = "\"this is text 1234 _ öö\" and_this_is_identifier";

  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(2, tokens.token_count());
      assert!(string_helper(&mut tokens, "this is text 1234 _ öö"));
      assert!(identifier_helper(&mut tokens, "and_this_is_identifier"));
    }
    Err(err) => { println!("{}", err); assert!(false); },
  }
}

#[test]
fn string_followed_by_identifier_without_whitespace_causes_an_error() {
  let string = "\"this is text 1234 _ öö\"and_this_is_identifier";

  match tokenize(string) {
    Ok(..) => assert!(false),
    Err(..) => assert!(true),
  }
}


#[test]
fn string_with_escaped_quote_is_tokenized_correctly() {
  let string = "\"this is text with \\\" an escaped quote\"";

  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      assert!(string_helper(&mut tokens, "this is text with \" an escaped quote"));
    }
    Err(..) => assert!(false)
  }
}

#[test]
fn string_with_newline_and_tab_are_handled_correctly() {
  let string = "\"this is text with new lines \\n and \\t tabs\"";

  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(1, tokens.token_count());
      assert!(string_helper(&mut tokens, "this is text with new lines \n and \t tabs"));
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
}

#[test]
fn string_followed_by_operator_is_handled_correctly() {
  let string = "+\"hello\"+";
  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(3, tokens.token_count());
      assert!(arith_op_helper(&mut tokens, TokenSubType::Plus));
      assert!(string_helper(&mut tokens, "hello"));
      assert!(arith_op_helper(&mut tokens, TokenSubType::Plus));
    }
    Err(..) => assert!(true),
  }
}



#[test]
fn parenthesis_brackets_braces_are_tokenized_correctly() {
  let string = "[( )]{  } ";
  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(6, tokens.token_count());
      assert!(generic_helper(&mut tokens, TokenType::LBracket));
      assert!(generic_helper(&mut tokens, TokenType::LParen));
      assert!(generic_helper(&mut tokens, TokenType::RParen));
      assert!(generic_helper(&mut tokens, TokenType::RBracket));
      assert!(generic_helper(&mut tokens, TokenType::LBrace));
      assert!(generic_helper(&mut tokens, TokenType::RBrace));
    },
    Err(..) => assert!(false),
  }
}

#[test]
fn comma_semi_colon_and_colon_are_tokenized_correctly() {
  let string = ";:, ";
  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(3, tokens.token_count());
      assert!(generic_helper(&mut tokens, TokenType::SemiColon));
      assert!(generic_helper(&mut tokens, TokenType::Colon));
      assert!(generic_helper(&mut tokens, TokenType::Comma));
    },
    Err(..) => assert!(false),
    }
  }

#[test]
fn assignment_and_comparison_operators_are_tokenized_correctly() {
  let string = "= == < > <= >= < = > = = = !=";

  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(13, tokens.token_count());
      assert!(generic_helper(&mut tokens, TokenType::Assign));
      assert!(comp_op_helper(&mut tokens, TokenSubType::Equals));
      assert!(comp_op_helper(&mut tokens, TokenSubType::Lesser));
      assert!(comp_op_helper(&mut tokens, TokenSubType::Greater));
      assert!(comp_op_helper(&mut tokens, TokenSubType::LesserOrEq));
      assert!(comp_op_helper(&mut tokens, TokenSubType::GreaterOrEq));
      assert!(comp_op_helper(&mut tokens, TokenSubType::Lesser));
      assert!(generic_helper(&mut tokens, TokenType::Assign));
      assert!(comp_op_helper(&mut tokens, TokenSubType::Greater));
      assert!(generic_helper(&mut tokens, TokenType::Assign));
      assert!(generic_helper(&mut tokens, TokenType::Assign));
      assert!(generic_helper(&mut tokens, TokenType::Assign));
      assert!(comp_op_helper(&mut tokens, TokenSubType::NotEq));

    },
    Err(..) => assert!(false),
  }
}

#[test]
fn function_call_syntax_is_tokenized_correctly() {
  let string = "foo(ident_1, ident_2);";

  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(7, tokens.token_count());
      assert!(identifier_helper(&mut tokens, "foo"));
      assert!(generic_helper(&mut tokens, TokenType::LParen));
      assert!(identifier_helper(&mut tokens, "ident_1"));
      assert!(generic_helper(&mut tokens, TokenType::Comma));
      assert!(identifier_helper(&mut tokens, "ident_2"));
      assert!(generic_helper(&mut tokens, TokenType::RParen));
      assert!(generic_helper(&mut tokens, TokenType::SemiColon));
    },
    Err(..) => assert!(false),
  }
}

#[test]
fn line_and_line_position_information_is_set_correctly_to_tokens() {
  let string = "ident 123\n[value+123]\n    ident2";

  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(8, tokens.token_count());
      assert!(line_helper(&mut tokens, 1, 1));
      assert!(line_helper(&mut tokens, 1, 7));
      assert!(line_helper(&mut tokens, 2, 1));
      assert!(line_helper(&mut tokens, 2, 2));
      assert!(line_helper(&mut tokens, 2, 7));
      assert!(line_helper(&mut tokens, 2, 8));
      assert!(line_helper(&mut tokens, 2, 11));
      assert!(line_helper(&mut tokens, 3, 5));
    },
    Err(..) => assert!(false),
  }
}

#[test]
fn keywords_are_tokenized_correctly() {
  let string = "if else while for let fn class new return public protected private true false int float double bool void";

  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(19, tokens.token_count());
      assert!(generic_helper(&mut tokens, TokenType::If));
      assert!(generic_helper(&mut tokens, TokenType::Else));
      assert!(generic_helper(&mut tokens, TokenType::While));
      assert!(generic_helper(&mut tokens, TokenType::For));
      assert!(generic_helper(&mut tokens, TokenType::Let));
      assert!(generic_helper(&mut tokens, TokenType::Fn));
      assert!(generic_helper(&mut tokens, TokenType::Class));
      assert!(generic_helper(&mut tokens, TokenType::New));
      assert!(generic_helper(&mut tokens, TokenType::Return));
      assert!(generic_helper(&mut tokens, TokenType::Public));
      assert!(generic_helper(&mut tokens, TokenType::Protected));
      assert!(generic_helper(&mut tokens, TokenType::Private));
      assert!(boolean_helper(&mut tokens, true));
      assert!(boolean_helper(&mut tokens, false));
      assert!(type_helper(&mut tokens, TokenSubType::IntegerType));
      assert!(type_helper(&mut tokens, TokenSubType::FloatType));
      assert!(type_helper(&mut tokens, TokenSubType::DoubleType));
      assert!(type_helper(&mut tokens, TokenSubType::BooleanType));
      assert!(type_helper(&mut tokens, TokenSubType::VoidType));
    },
    Err(..) => assert!(false),
  }
}

#[test]
fn comments_are_ignored_correctly() {
  let string="ident_1// This is comment\nident2";
  match tokenize(string) {
    Ok(mut tokens) => {
      assert_eq!(2, tokens.token_count());
      assert!(identifier_helper(&mut tokens, "ident_1"));
      assert!(identifier_helper(&mut tokens, "ident2"));

    },
    Err(..) => assert!(false),
  }
}


fn arith_op_helper(tokens: &mut Tokens, subtype:TokenSubType) -> bool {

  let expected = SyntaxToken::new(TokenType::ArithOp, subtype, 0 ,0);
  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false,
  }
}

fn comp_op_helper(tokens: &mut Tokens, subtype:TokenSubType) -> bool {

  let expected = SyntaxToken::new(TokenType::CompOp, subtype, 0 ,0);
  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false,
  }
}

fn identifier_helper(tokens: &mut Tokens, expected_text: &str) -> bool {

  let expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(expected_text.to_string()), 0, 0);
  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false
  }
}

fn integer_helper(tokens: &mut Tokens, expected_number: i32) -> bool {

  let expected = SyntaxToken::new(TokenType::Number, TokenSubType::IntegerNumber(expected_number), 0, 0);
  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false,
  }
}

fn double_helper(tokens: &mut Tokens, expected_number: f64) -> bool {
  let expected = SyntaxToken::new(TokenType::Number, TokenSubType::DoubleNumber(expected_number), 0, 0);

  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false
  }
}

fn float_helper(tokens: &mut Tokens, expected_number: f32) -> bool {

  let expected = SyntaxToken::new(TokenType::Number, TokenSubType::FloatNumber(expected_number), 0, 0);
  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false
  }
}

fn boolean_helper(tokens: &mut Tokens, expected_value: bool) -> bool {

  let expected = SyntaxToken::new(TokenType::Boolean, TokenSubType::BooleanValue(expected_value), 0, 0);
  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false
  }
}


fn string_helper(tokens: &mut Tokens, expected_string: &str) -> bool {

  let expected = SyntaxToken::new(TokenType::Text, TokenSubType::Text(expected_string.to_string()), 0, 0);

  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false
  }
}

fn generic_helper(tokens: &mut Tokens, token_type: TokenType) -> bool {

  let expected = SyntaxToken::new(token_type, TokenSubType::NoSubType, 0 ,0);

  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false
  }
}

fn line_helper(tokens: &mut Tokens,  line_number: i32, line_pos: i32) -> bool {

  match tokens.next() {
    Some(token) => token.line == line_number && token.pos_at_line == line_pos,
    None => false
  }
}

fn type_helper(tokens: &mut Tokens, subtype: TokenSubType) -> bool {

  let expected = SyntaxToken::new(TokenType::VarType, subtype, 0 ,0);

  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false
  }
}
