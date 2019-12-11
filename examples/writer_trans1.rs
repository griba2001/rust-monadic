//! examples/writer_trans1.rs
//!
//! you may set the logger type 
//! by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type 
//! where String is the default if omitted
//! as in `let res : Writer< _, String > = wrdo!{...}`
//!
//! `censor(), listen() and listens()` can be used as functions or as methods of a Writer bloc

#[allow(unused_imports)]
use monadic::{wrt_mdo, monad::Monad, writer_trans::{WriterT, lift, tell, tell_str, censor, listen}};
use monadic::util::concat_string_str;
use partial_application::partial;
use num::Integer;

fn main() {
    
    let modify_log = partial!( concat_string_str => _, "log2");
    
    let bloc = wrt_mdo!{  // : WriterT< Vec<_>, String>
    
        _ <- tell_str( "log1") as WriterT< Vec<_>> ;
        x <- lift (5..9).collect::<Vec<_>>() ;
        guard x.is_odd() ;
        
        // run a subbloc and modify its log afterwards
        pair <- censor( modify_log,
                        wrt_mdo!{
                            _ <- tell_str("sub");
                            lift Vec::pure( 2)
                        }.listen()
                      );
                    
        lift Vec::pure( (x, pair.0, pair.1) )            
        }.listen() ;
        
    let res = bloc.unwrap();    
    
    println!("result: {:?}", res); 
}
