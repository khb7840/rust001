/*
 * File: main.rs
 * Project: src
 * Created: 2021-05-18 19:10:51
 * Author: Hyunbin Kim (khb7840@gmail.com)
 * Description:
 *     This code is written as part of project "src".
 * ---
 * Last Modified: 2021-05-18 19:32:39
 * Modified By: Hyunbin Kim (khb7840@gmail.com)
 * ---
 * Copyright © 2021 Hyunbin Kim, All rights reserved
 */

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hi! Make a guess!");

    let mut guess = String::new();
    // let foo = 5;
    // let mut bar = 10;
    // println!("Foo {}, Bar {}", foo, bar);
    // bar = 20;
    // println!("Foo {}, Bar {}", foo, bar);

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("Secret number: {}", secret_number);
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number");


    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),

    }

}
