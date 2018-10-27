use std::io;

fn main() {
    println!("- guessing game -");

    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect(":/ couldn't read line");
    
    // removes trailing new-line char
    guess.pop();
    
    println!("[{}]", guess);
}
