//! examples/writer_trans2.rs
//!
//! you may set the logger type 
//! by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type 
//! where String is the default if omitted
//! as in `let res : WriterT< _, Vec<_>> = wrdo!{...}`
//!
//! `censor(), listen() and listens()` can be used as functions or as methods of a Writer bloc

#[allow(unused_imports)]
use monadic::{wrt_mdo, monad::Monad, writer_trans::{WriterT, tell, tell_str, tell_array, censor, listen}};
use monadic::util::concat_vec_array;
use partial_application::partial;
use num::Integer;

type Log = Vec<i32>;

fn main() {
    
    let modify_log = partial!( concat_vec_array => _, &[4,5,6]);
    
    let bloc = wrt_mdo!{  // : WriterT< Vec<_>, Log>
    
        _ <- tell_array &[1,2,3] ;
        
        x <- lift (5..9).collect::<Vec<_>>() ;
        
        guard x.is_odd() ;
        let z = x + 1;
        
        // run a subbloc and modify its log afterwards
        pair <- censor( modify_log,
                        wrt_mdo!{
                            _ <- tell_array &[0];
                            pure 2
                        }.listen()
                      );
                    
        pure (z, pair.0, pair.1)            
        }.listen() ;
        
    // unwrap() returns the nested monad structure       
    let res = bloc.unwrap(); 
    
    println!("result: {:?}", res); 
}
