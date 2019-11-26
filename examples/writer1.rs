//! you may set the logger type by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type
//! as in `let res : Writer<(i32,i32),String = wrdo!{...}`

use monadic::{wrdo, writer::*};
use partial_application::partial;

// example function for use in `censor` function through a partial partial_application

fn concat_strings( mut s1: String, s2: &str) -> String {
   s1.push_str( s2);
   s1
}

fn main() {
    
    let res = wrdo!{ 
        _ <- tell_str( "log1") ;
        x <-  Writer::pure(1) ;
        let z = x+1;
        pure (x, z)
    }.censor( partial!( concat_strings => _, "log2")
            ).listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
