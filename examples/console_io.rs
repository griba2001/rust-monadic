// example console io

use monadic::{mdo, monad::{Bind, Monad}, mio::{read_line, print_str, stdout_flush}};

fn main() {
    let res =mdo!{
                _ <- print_str("enter i32>");
                _ <- stdout_flush();
                li1 <- read_line();
                x <- li1.trim().parse::<i32>() ;
                pure (x, x+1, x+2)
              }.collect::<Vec<(_,_,_)>>();

    println!("result: {:?}", res);              
}
