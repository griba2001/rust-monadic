use monadic::{wrdo, writer::*};
use partial_application::partial;

fn concat_vecs<T: Clone>( mut s1: Vec<T>, mut s2: Vec<T>) -> Vec<T> {

   s1.append( &mut s2);
   s1
}

fn main() {

    let res = wrdo!{ 
        _ <- tell_vec( vec![1,2,3]) ;
        x <-  Writer::pure(1) ;
        let z = x+1;
        pure (x, z)
    }.censor( partial!( concat_vecs => _, vec![4,5,6])
            ).listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
