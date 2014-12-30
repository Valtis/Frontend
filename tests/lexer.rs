extern crate compiler;

use compiler::lexer::tokenize;
use compiler::token::SyntaxToken;
use compiler::token::TokenType;
use compiler::token::TokenSubType;
use compiler::token::Tokens;

use std::num::Float;

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
      assert!(operator_helper(&mut tokens, TokenSubType::Plus));
      assert!(identifier_helper(&mut tokens, ident2));
      assert!(operator_helper(&mut tokens, TokenSubType::Multiply));
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
      assert!(operator_helper(&mut tokens, TokenSubType::Multiply));
      assert!(integer_helper(&mut tokens, 14));
      assert!(operator_helper(&mut tokens, TokenSubType::Plus));
      assert!(integer_helper(&mut tokens, 15));
      assert!(operator_helper(&mut tokens, TokenSubType::Multiply));
      assert!(integer_helper(&mut tokens, 16));
      assert!(operator_helper(&mut tokens, TokenSubType::Divide));
      assert!(integer_helper(&mut tokens, 16));
      assert!(operator_helper(&mut tokens, TokenSubType::Minus));
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
      assert!(operator_helper(&mut tokens, TokenSubType::Multiply));
      assert!(float_helper(&mut tokens, 32f32));
      assert!(operator_helper(&mut tokens, TokenSubType::Plus));
      assert!(integer_helper(&mut tokens, 12));
      assert!(operator_helper(&mut tokens, TokenSubType::Plus));
      assert!(double_helper(&mut tokens, 1.343));
      assert!(operator_helper(&mut tokens, TokenSubType::Multiply));
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
      assert!(operator_helper(&mut tokens, TokenSubType::Plus));
      assert!(string_helper(&mut tokens, "hello"));
      assert!(operator_helper(&mut tokens, TokenSubType::Plus));
    }
    Err(..) => assert!(true),
  }
}


fn operator_helper(tokens: &mut Tokens, subtype:TokenSubType) -> bool {

  let expected = SyntaxToken::new(TokenType::ArithOp, subtype);
  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false,
  }
}


fn identifier_helper(tokens: &mut Tokens, expected_text: &str) -> bool {

  let expected = SyntaxToken::new(TokenType::Identifier, TokenSubType::Identifier(expected_text.to_string()));
  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false
  }
}

fn integer_helper(tokens: &mut Tokens, expected_number: i32) -> bool {

  let expected = SyntaxToken::new(TokenType::Number, TokenSubType::IntegerNumber(expected_number));
  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false,
  }
}

fn double_helper(tokens: &mut Tokens, expected_number: f64) -> bool {
  match tokens.next() {
    Some(actual) => {
      if actual.t_type == TokenType::Number {
        match actual.t_subtype {
          TokenSubType::DoubleNumber(actual_number) => (actual_number - expected_number).abs() < 0.0001,
          _ => false,
        }

        } else {
          false
        }
      }
    None => false
  }
}

fn float_helper(tokens: &mut Tokens, expected_number: f32) -> bool {
  match tokens.next() {
    Some(actual) => {
      if actual.t_type == TokenType::Number {
        match actual.t_subtype {
          TokenSubType::FloatNumber(actual_number) => (actual_number - expected_number).abs() < 0.0001,
          _ => false,
        }

        } else {
          false
        }
      }
    None => false
  }
}


fn string_helper(tokens: &mut Tokens, expected_string: &str) -> bool {

  let expected = SyntaxToken::new(TokenType::Text, TokenSubType::Text(expected_string.to_string()));

  match tokens.next() {
    Some(actual) => expected == *actual,
    None => false
  }
}
