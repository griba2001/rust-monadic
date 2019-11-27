# rust-monadic

* [the macro mdo! based on Bind and Monad supertraits over IntoIterator (iterables)](#mdo)
* [the macro monadic! based directly on IntoIterator and Iterator methods](#monadic)
* [the macro wrdo! based on a Writer struct type as a Writer monad](#wrdo)

### The macro mdo! <a name="mdo" id="mdo"></a>

A macro to write Haskell style monadic code

for [**IntoIterator**](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) (iterables) as monads

Each step monad expression is [flat_mapped](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map) with the rest into a lazy *FlatMap* expression which implements *IntoIterator* with lambdas as [*move* closures](https://doc.rust-lang.org/1.30.0/book/first-edition/closures.html#move-closures) capturing the environment and argument. The lambda body will be recursively parsed as monadic, and its type should also be an instance of *IntoIterator*.

The traits **Bind** and **Monad** are defined in module *monad* as supertraits of IntoIterator.

You can use: 
* to return an expression value: `pure return_expresion`
* to use the monad result:       `v <- monadic_expression`
* to ignore the monad result:    `_ <- monadic_expression`
* to combine monad results:      `let z = expression`
* to filter results:             `guard boolean_expression` 

Note: *let*, within the macro, introduces an expression, not a block.


Example1: monadic comprehensions à la Haskell (file: examples/comprehension.rs)

```rust
use monadic::{mdo, monad::{Bind, Monad}};
use num::Integer;

fn main() {
    let xs = mdo!{ 
    
        x <- 1..7;
        y <- 1..x;
        guard (&y).is_odd() ;
        let z = match x.is_even() { 
                    true => &y + 1,
                    _ => &y - 1,
                };
        pure (x, z)
        
    }.collect::<Vec<(i32,i32)>>();
    
    println!("result: {:?}", xs); 
}

```
Execution:

```bash
$ cargo run --example comprehension

result: [(2, 2), (3, 0), (4, 2), (4, 4), (5, 0), (5, 2), (6, 2), (6, 4), (6, 6)]
```
Example2: variation with references to container and lambda argument position (file: examples/comprehension2.rs)

```rust
use monadic::{mdo, monad::{Bind, Monad}};
use num::Integer;

fn main() {
    let xs = mdo!{ 
    
        &x <- &vec![1,2,3,4];   // with item refs (&x) in the lambda argument position
        guard x.is_odd() ;
        let z = x + 1 ;
        pure (x, z)
        
    }.collect::<Vec<(i32,i32)>>();
    
    println!("result: {:?}", xs); 
}

```
Execution:

```bash
$ cargo run --example comprehension2

result: [(1, 2), (3, 4)]
```

### The original macro monadic! <a name="monadic" id="monadic"></a>

Same functionality as *mdo* using `IntoIterator` and `Iterator` methods directly, avoiding intermixed `Bind` and `Monad` traits definitions.

Here is example1 using it:

```rust
use monadic::monadic;
use num::Integer;

fn main() {
    let xs = monadic!{ 
    
        x <- 1..7;
        y <- 1..x;
        guard (&y).is_odd() ;
        let z = match x.is_even() { 
                    true => &y + 1,
                    _ => &y - 1,
                };
        pure (x, z)
        
    }.collect::<Vec<(i32,i32)>>();
    
    println!("result: {:?}", xs); 
}

```
### The Writer monad macro wrdo! <a name="wrdo" id="wrdo"></a>

A [Writer monad](https://wiki.haskell.org/All_About_Monads#The_Writer_monad) adaptation macro example with String as logger, from examples/writer1.rs

```rust
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
```
Exec:

```bash
$ cargo run --example writer1

result: ((1, 2), "log1log2")

```
Example 2 with Vec as logger from examples/writer2.rs

```rust
use monadic::{wrdo, writer::*};
use monadic::util::concat_vec_array;
use partial_application::partial;


fn main() {

    let res = wrdo!{ 
        _ <- tell_vec( vec![1,2,3]) ;
        x <-  Writer::pure(1) ;
        let z = x+1;
        pure (x, z)
    }.censor( partial!( concat_vec_array => _, &[4,5,6])
            ).listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}

```

```bash
$ cargo run --example writer2

result: ((1, 2), [1, 2, 3, 4, 5, 6])

```
