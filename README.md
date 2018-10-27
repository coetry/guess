# guess

🦀  simple guessing game in rust, wip

```rust
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    loop {
        println!("- guessing game -");

        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1, 101);
        
        io::stdin().read_line(&mut guess)
            .expect(":/ couldn't read line");

        guess.pop();
        
        println!("[{}:{}]", guess, secret_number);
    }
}
```
