// example console io

use monadic::{mdo, monad::{Bind, Monad}, mio::{read_line, print_str, stdout_flush}};
use std::io;
use monadic::util::concat_string_str;

    fn my_block() -> io::Result<String> {
    
          let bres = mdo!{
          
                _ <- print_str("enter line>");
                _ <- stdout_flush();
                li1 <- read_line();
                let li2 = concat_string_str(li1, "def");
                pure li2
              }.collect::<Vec<String>>();
              
           Ok(bres[0].clone()) 
           
     }

fn main() {
              
    match my_block() {
      Ok( v) => println!("result: {:?}", v),
      Err( e) => println!("err: {:?}", e), 
    }
}
