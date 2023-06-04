use std::fs;

mod keyword;
mod lexer;

use lexer::Lexer;

fn main() {
   let file = fs::read_to_string("test/int64/basic.pye").expect("file does not exist");
   println!("{:?}", file);

   let lexer = Lexer::new(&file);
   println!("{:?}", lexer.read());
}
