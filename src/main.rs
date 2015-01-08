extern crate compiler;


#[cfg(not(test))]
use std::io::File;
#[cfg(not(test))]
use std::str::from_utf8;

#[cfg(not(test))]
fn main() {
  let tokens = tokenize_file("file");
  parse_tokens(&tokens);

}

#[cfg(not(test))]
fn read_file(name: &str) -> String {
  let contents = File::open(&Path::new(name)).read_to_end();
  match contents {
    Ok(res) => {
      match from_utf8(res.as_slice()) {
        Ok(utf_res) => return utf_res.to_string(),
        Err(err) => panic!("Conversion error: {}", err),
      }
    }
    Err(err) => panic!("Io Error: {}", err),
  }
}

#[cfg(not(test))]
fn tokenize_file(name: &str) -> compiler::token::Tokens {

  let content = read_file(name);
  match compiler::lexer::tokenize(content.as_slice()) {
    Ok(tokens) => {
      return tokens;
    },
    Err(errors) => {
      print_errors(errors);
      panic!("Terminating process due to previous error(s)");
    }
  };
}

#[cfg(not(test))]
fn parse_tokens(tokens: &compiler::token::Tokens) {
  match compiler::parser::parse(tokens) {
    Ok(..) => println!("Parsing succeeded"),
    Err(errors) => {
      print_errors(errors);
      panic!("Terminating process due to previous error(s)");
    }
  }
}

#[cfg(not(test))]
fn print_errors(errors: Vec<String>) {
  println!("Error(s) were found:");
  for error in errors.iter() {
    println!("{}", error)
  }
  panic!("Terminating process due to previous error(s)");
}
