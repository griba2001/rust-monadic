//! you may set the logger type by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type where String is the default if omitted
//! as in `let res : Writer<(i32,i32),String> = wrdo!{...}`

use monadic::{wrdo, writer::*};
use monadic::util::concat_string_str;
use partial_application::partial;

fn main() {
    
    let res = wrdo!{ 
        _ <- tell_str( "log1") ;
        x <-  Writer::pure(1) ;
        let z = x+1;
        pure (x, z)
    }.censor( partial!( concat_string_str => _, "log2")
            ).listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
