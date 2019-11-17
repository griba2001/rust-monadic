# rust-monadic

A macro to write Haskell style monadic code

for [**IntoIterator**](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) (iterables) as monads

Each step monad expression is [flat_mapped](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map) with the rest into a lazy *FlatMap* expression which implements *IntoIterator* with lambdas as [*move* closures](https://doc.rust-lang.org/1.30.0/book/first-edition/closures.html#move-closures) capturing the environment and argument. The lambda body type should also be an instance of *IntoIterator* and it will be recursively parsed as monadic.

You can use: 
* ```Option::pure( return_expresion)```  to return an expression value
* ```v <- monadic_expression```  to use the monad result
* ```_ <- monadic_expression```  to ignore the monad result
* ```let z = expression```       to combine monad results
* ```guard boolean_expression``` to filter results

Note: *let*, within the macro, introduces an expression, not a block.


Example: monadic comprehensions à la Haskell

```
// examples/comprehension.rs

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
```bash
$ cargo run --example comprehension

result: [(2, 2), (3, 2), (4, 2), (4, 4), (5, 2), (5, 4), (6, 2), (6, 4), (6, 6)]
```
