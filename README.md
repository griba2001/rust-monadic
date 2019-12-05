# rust-monadic

* [A monad bloc macro based on Bind and Monad as supertraits of IntoIterator (iterables)](#mdo)
* [A Reader monad bloc macro](#rdrdo)
* [A Writer monad bloc macro](#wrdo)
* [A State monad bloc macro](#stdo)

### The macro mdo! <a name="mdo" id="mdo"></a>

A macro to write Haskell style monadic code

for [**IntoIterator**](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) (iterables) as monads

Each step monad expression is [flat_mapped](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map) with the rest into a lazy *FlatMap* expression which implements *IntoIterator* with lambdas as [*move* closures](https://doc.rust-lang.org/1.30.0/book/first-edition/closures.html#move-closures) capturing the environment and argument. The lambda body will be recursively parsed as monadic, and its type should also be an instance of *IntoIterator*.

The traits **Bind** and **Monad** are defined in module *monad* as supertraits of IntoIterator.

Here is a table where a **monadic_expression** is one of a type instance of IntoIterator: 

<table>
<tr><td>* to return an expression value:</td> <th>`pure return_expresion`</th></tr>
<tr><td>* to end with a monadic expr.:</td> <th>`monadic_expression`</th></tr>
<tr><td>* to use the monad result:</td> <th>``v <- monadic_expression`</th></tr>
<tr><td>* to ignore the monad result:</td> <th>`_ <- monadic_expression`</th></tr>
<tr><td>* to combine monad results:</td> <th>`pure return_expresion`</th></tr>
<tr><td>* to filter results:</td> <th>`guard boolean_expression`</th></tr>
</table>

Note: *let*, within the macro, introduces an expression, not a Haskell block.


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
        
    }.collect::<Vec<_>>();
    
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
    
        &x <- &vec![1,2,3,4];   // with item ref pattern (&x) in the lambda argument position
        guard x.is_odd() ;
        let z = x + 1 ;
        pure (x, z)
        
    }.collect::<Vec<_>>();
    
    println!("result: {:?}", xs); 
}

```
Execution:

```bash
$ cargo run --example comprehension2

result: [(1, 2), (3, 4)]
```

Example: console io. If you want to return String variables, you may do it through cloning

```rust
// example console io

use monadic::{mdo, monad::{Bind, Monad}, 
                   mio::{read_line, print_str, stdout_flush}};

fn main() {
    let res =mdo!{
    
                x <- pure 1;
                let y = x + 1;
                
                _ <- print_str("enter integer i32>");
                _ <- stdout_flush();
                
                li1 <- read_line();
                z <- li1.trim().parse::<i32>() ;
                
                pure (y, z, li1.clone())
                
              }.collect::<Vec<_>>();

    println!("result: {:?}", res);              
}
```
```bash
$ cargo run --example console_io

enter integer i32>10
result: [(2, 10, "10")]

```

### The Reader monad macro rdrdo! <a name="rdrdo" id="rdrdo"></a>

A [Reader monad](https://wiki.haskell.org/All_About_Monads#The_Reader_monad) adaptation macro example

```rust
//! examples/reader1
//!
//! You must specify in a type restriction the type of the environment of the Reader bloc
//!
//! `local` can be used as a function or as a method

use monadic::{rdrdo, reader::{Reader, ask, local}};
use partial_application::partial;
use std::collections::HashMap;

type Env = HashMap<String, i32>;

fn immutable_insert( k_slice: &str, v: i32, dict: Env) -> Env {
   let mut dict1 = dict.clone();
   dict1.insert( String::from(k_slice), v);
   dict1
}

fn my_initial_env() -> Env {
   immutable_insert( "a", 1, HashMap::new())
}   


fn main() {

  let modify_env = partial!(immutable_insert => "b", 2, _);
  
  let bloc1: Reader<'_, Env, _>  = rdrdo!{
  
       env1 <- ask();
       
       // run a subbloc with a modified environment
       pair <- local( modify_env, rdrdo!{ 
       
               x <- pure 9;
               y <- ask();
               pure (x, y)
             }) ;
             
       pure (env1.clone(), pair.0, pair.1)      
    };


  let res = bloc1.initial_env( my_initial_env() );

  println!("result: {:?}", res);  
}
```
Execution:

```bash
$ cargo run --example reader1

result: ({"a": 1}, 9, {"b": 2, "a": 1})
```

### The Writer monad macro wrdo! <a name="wrdo" id="wrdo"></a>

A [Writer monad](https://wiki.haskell.org/All_About_Monads#The_Writer_monad) adaptation macro example with String as logger, from examples/writer1.rs

```rust
//! examples/writer1.rs
//!
//! you may set the logger type 
//! by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type 
//! where String is the default if omitted
//! as in `let res : Writer< _, String > = wrdo!{...}`
//!
//! `censor(), listen() and listens()` can be used as functions or as methods of a Writer bloc

#[allow(unused_imports)]
use monadic::{wrdo, writer::{Writer, tell, tell_str, censor, listen}};
use monadic::util::concat_string_str;
use partial_application::partial;

fn main() {
    
    let modify_log = partial!( concat_string_str => _, "log2");
    
    let res : Writer< _, String> = wrdo!{ 
    
        _ <- tell_str( "log1") ;
        
        // run a subbloc and modify the log afterwards
        censor( modify_log,
                   wrdo!{
                        listen( Writer::pure( 2))
                    })
        }.listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
```
Exec:

```bash
$ cargo run --example writer1

result: ((2, ""), "log1log2")

```
Example 2 with Vec as logger from examples/writer2.rs

```rust
//! examples/writer2.rs
//! 
//! you may set the logger type 
//! by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type 
//! where String is the default if omitted
//! as in `let res : Writer< _, Vec<_> > = wrdo!{...}`
//!
//! `censor(), listen() and listens()` can be used as functions or as methods of a Writer bloc

use monadic::{wrdo, writer::{Writer, tell, censor, listen}};
use monadic::util::concat_vec_array;
use partial_application::partial;


fn main() {

    let modify_log = partial!( concat_vec_array => _, &[4,5,6]);
    
    let res : Writer< _, Vec<_>> = wrdo!{ 
    
        _ <- tell( vec![1,2,3]) ;
        
        // run a subbloc and modify the log afterwards
        censor( modify_log,
                   wrdo!{
                        listen( Writer::pure( 2))
                    })
        }.listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
```

```bash
$ cargo run --example writer2

result: ((2, []), [1, 2, 3, 4, 5, 6])

```
### The State monad macro stdo! <a name="stdo" id="stdo"></a>

A [State monad](https://wiki.haskell.org/All_About_Monads#The_State_monad) adaptation macro example from examples/state1.rs

```rust
//! examples/state1.rs
//!
//! You may specify in a type restriction the type of the State bloc
//! or apply it directly to an initial_state without the type restriction

use monadic::{stdo, state::{State, get, put}};

fn main() {
  let bloc: State<'_, i32, _> = stdo!{
  
       x <- pure 9;
       y <- get();
       _ <- put( 1);
       z <- get(); 
       pure (x, y, z) 
       
    };
    
  let res = bloc.initial_state(0);  

  println!("result: {:?}", res);  
}
```

```bash
$ cargo run --example state1

result: ((9, 0, 1), 1)

```
Some tests:

```bash
$ cargo test
running 10 tests
test monad::tests::prop_monad_comprehension_vs_iteration ... ok
test monad::tests::prop_option_mplus_associative ... ok
test monad::tests::prop_option_mplus_left_catch ... ok
test monad::tests::prop_option_mplus_left_identity ... ok
test monad::tests::prop_option_mplus_left_zero ... ok
test monad::tests::prop_option_mplus_right_identity ... ok
test monad::tests::prop_vec_mplus_associative ... ok
test monad::tests::prop_vec_mplus_left_identity ... ok
test monad::tests::prop_vec_mplus_left_distribution ... ok
test monad::tests::prop_vec_mplus_right_identity ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Changes:
v. 0.4.2: added MonadPlus with **quickcheck** tests

v. 0.4.1: console_io example showing String return through cloning

v. 0.4.0: 
* renamed writer function `censor_do` as censor
* added writer function listen() and listens()
* renamed local_do() as local()
* removed intoiter module as it duplicates functionality without added applicability, use module monad's `mdo` macro instead

v. 0.3.14: added writer function `censor_do`

v. 0.3.13: added reader function `local_do`

v. 0.3.12: example reader1 simplification.

v. 0.3.11: suppressed  the form "&v <- ..." from Writer and State monads.

v. 0.3.10: Added the Reader macro. It runs good over clonable environments e.g. HashMap. 
The State macro has been updated, using a non static lifetime for the boxed closure
           
v. 0.3.9: Added (<-) rhs `pure`.


