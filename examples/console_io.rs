// example console io

use monadic::{mdo, monad::{Bind, Monad}, mio::{read_line, print_str, stdout_flush}};
use std::io;

    fn my_block() -> io::Result<String> {
    
          let bres = mdo!{
                _ <- print_str("enter line>");
                _ <- stdout_flush();
                
                _ <- read_line();
                _ <- print_str("the second line is the good one>");
                _ <- stdout_flush();
                li <- read_line();
                pure li
              }.collect::<Vec<String>>();
              
           Ok(bres[0].clone()) 
           
     }

fn main() {
              
    match my_block() {
      Ok( v) => println!("result: {:?}", v),
      Err( e) => println!("err: {:?}", e), 
    }
}
