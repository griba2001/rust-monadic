# rust-monadic

Example: monadic comprehensions à la Haskell

```
use monadic::monadic;

fn main() {
    let xs = monadic!{ 
        x <- 1..8;
        y <- 1..=x;
        guard y < x / 2 ;
        let z = y + 1 ;
        Some((x, z)) 
    }.collect::<Vec<(i32,i32)>>();
    
    println!("result: {:?}", xs); 
}

```

result: [(4, 2), (5, 2), (6, 2), (6, 3), (7, 2), (7, 3)]
