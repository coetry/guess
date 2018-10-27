# guess

🦀  simple guessing game in rust, wip

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn main() -> io::Result<()> {
    println!("- guessing game -");

    let mut x = File::open("./x")?;
    let mut buffer = [0; 16];
    let mut guess = String::new();

    x.read(&mut buffer)?;
    
    io::stdin().read_line(&mut guess)
        .expect(":/ couldn't read line");
    
    // removes trailing new-line char
    guess.pop();
    
    println!("[{}]", guess);
    println!("{:?}", buffer);

    Ok(())
}
```
