use std::io;
//use std::io::Read;
// use std::fs::File;

extern crate rand;

use rand::Rng;

fn main() {
    loop {
        println!("- guessing game -");

     // let mut x = File::open("./x")?;
     // let mut buffer = [0; 16];
        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1, 101);
        

     // x.read(&mut buffer)?;
        
        io::stdin().read_line(&mut guess)
            .expect(":/ couldn't read line");

     // removes trailing new-line char
        guess.pop();
        
        println!("[{}:{}]", guess, secret_number);
     // println!("{:?}", buffer);
    }
}
