extern crate rand;

use std::{io, cmp::Ordering};
use rand::Rng;

fn main() 
{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("please input your guess");

        let mut guess = String::new();

        let res = io::stdin().read_line(&mut guess);
        if let Err(_) = res 
        {
            println!("input failed");
        }

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed : {}", secret_number);

        match guess.cmp(&secret_number)
        {
            Ordering::Less    => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal   => { 
                println!("You Win");
                break;
            },
        }
    }
}
