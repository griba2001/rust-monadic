use monadic::{stt_mdo, state_trans::{StateT, get, put}};
use num::Integer;

// mandatory type alias as it is used within the macro for type annotations
type St = i32;

fn main() {
  let bloc = stt_mdo!{ // : StateT<'_, St, Vec<_>, _>    // StateT<'a, St, Monad, A>
  
       // x <- lift (5..9).collect::<Vec<_>>() ;
       x <- lift_iter 5..9 ;
       guard x.is_odd();
       
       y <- get() ; 
       
       _ <- put( 1) ; 
       z <- get() ;
       
       let v = x +1 ; 
       
       pure (v, y, z)
    };
  
  // returns the monad within the transformer boxed function (s -> m (a,s))
  let res = bloc.initial_state( 0);

  println!("result: {:?}", res);  
}
