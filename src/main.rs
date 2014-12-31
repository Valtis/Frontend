extern crate compiler;

#[cfg(not(test))]
use std::io::File;
#[cfg(not(test))]
use std::str::from_utf8;

#[cfg(not(test))]
fn main() {


  // just testing stuff for now
  let contents = File::open(&Path::new("file")).read_to_end();
  match contents {
    Ok(res) => {
      match from_utf8(res.as_slice()) {
        Ok(utf_res) => {
          println!("{}", utf_res);
          println!("\n\nTokenization result: ");

          match compiler::lexer::tokenize(utf_res) {

            Ok(mut tokens) => {
              loop {
                match tokens.next() {
                  Some(token) => println!("{}", token),
                  None => break,
                };
              }
            }
            Err(err) => println!("Error: {}", err),
          };

        },
        Err(err) => println!("Conversion error: {}", err),
      }
    }
    Err(err) => println!("Io Error: {}", err),
  }
}
