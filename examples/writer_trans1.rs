//! examples/writer_trans1.rs
//!
//! you may set the logger type 
//! by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type 
//! where String is the default if omitted
//! as in `let res : WriterT< _, Log > = wrdo!{...}`
//!
//! `censor(), listen() and listens()` can be used as functions or as methods of a Writer bloc

#[allow(unused_imports)]
use monadic::{wrt_mdo, monad::Monad, writer_trans::{WriterT, tell, tell_str, tell_array, censor, listen}};
use monadic::util::concat_string_str;
use partial_application::partial;
use num::Integer;

/// mandatory type alias Log as it is used in the macro
/// to save you type annotations
type Log = String;

fn main() {
    
    let modify_log = partial!( concat_string_str => _, "log2");
    
    let bloc = wrt_mdo!{  // : WriterT< Vec<_>, Log>
    
        _ <- tell_str "log1" ;
        
        x <- lift (5..9).collect::<Vec<_>>() ;
        
        guard x.is_odd() ;
        let z = x + 1;
        
        // run a subbloc and modify its log afterwards
        pair <- censor( modify_log,
                        wrt_mdo!{
                            _ <- tell_str "sub";
                            pure 2
                        }.listen()
                      );
                    
        pure (z, pair.0, pair.1)
        }.listen() ;
        
    // unwrap() returns the nested monad structure       
    let res = bloc.unwrap(); 
    
    println!("result: {:?}", res); 
}
