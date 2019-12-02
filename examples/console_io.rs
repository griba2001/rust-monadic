// example console io

use monadic::{mdo, monad::{Bind, Monad}, mio::{read_line, print_str, stdout_flush}};

fn main() {
    let res =mdo!{
    
                x <- pure 1;
                let y = x + 1;
                _ <- print_str("enter i32>");
                _ <- stdout_flush();
                li1 <- read_line();
                z <- li1.trim().parse::<i32>() ;
                pure (y, z)
                
              }.collect::<Vec<_>>();

    println!("result: {:?}", res);              
}
