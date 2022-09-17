// Summera tal

use std::io;

fn main() {
    
    
    let mut numb1 = String::new();
    let mut number_line = String::new();

    
    io::stdin().read_line(&mut numb1).expect("Failed to read the number of numbers!");
    io::stdin().read_line(&mut number_line).expect("Failed to read numbers!");

    let mut first_number: usize = numb1.trim().parse().expect("Failed to parse the number of numbers!"); 
    
    
    if first_number % 2 == 1 { 
        first_number += 1; //make odd numbers even
    }

    let mut numbers: Vec<u32> = number_line // make a vector of the numbers
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Failed to parse the numbers!"))
        .collect();
    //sort the numbers from large to small
    numbers.sort(); 
    numbers.reverse(); 


    let mut answer: u32 = 0;
    for index in 0..(first_number / 2) {
        answer += numbers[index]; 
    }

    println!("{}", answer);

}