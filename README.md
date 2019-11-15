# rust-monadic

A macro to write Haskell style monadic code

for [**IntoIterator**](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) (iterables) as monads

[*flat_mapping*](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map) with lambdas as [*move* closures](https://doc.rust-lang.org/1.30.0/book/first-edition/closures.html#move-closures) capturing the environment and argument.

Note: *let* within the macro, introduces an expression, not a block.

You can use 
* ```Some( returnValue)```       a monadic return
* ```v <- monadic_expression```  using the monad result
* ```_ <- monadic_expression```  ignoring the monad result
* ```let z = expression```       combining monad results
* ```guard boolean_expression``` to filter results



Example: monadic comprehensions à la Haskell

```
use monadic::monadic;
use num::Integer;

fn main() {
    let xs = monadic!{ 
    
        x <- 1..7;
        y <- 1..x;
        guard (&y).is_odd() ;
        let z = &y + 1 ;
        Some((x, z)) 
        
    }.collect::<Vec<(i32,i32)>>();
    
    println!("result: {:?}", xs); 
}

```

result: [(2, 2), (3, 2), (4, 2), (4, 4), (5, 2), (5, 4), (6, 2), (6, 4), (6, 6)]
