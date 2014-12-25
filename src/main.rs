#[cfg(not(test))]
use std::io::File;
#[cfg(not(test))]
use std::str::from_utf8;


mod lexer;
mod token;

#[cfg(not(test))]
fn main() {
  // just testing stuff for now
  let contents = File::open(&Path::new("file")).read_to_end();
  match contents {
    Ok(res) => {
      match from_utf8(res.as_slice()) {
        Ok(utf_res) => {
          println!("{}", utf_res)
        },
        Err(err) => println!("Conversion error: {}", err),
      }
    }
    Err(err) => println!("Io Error: {}", err),
  }
}
