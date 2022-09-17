
// A different problem



use std::io;
 use std::io::prelude::*;
 
 fn main() {

    let input = io::stdin();
    let lines = input 
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    for x in lines.iter(){
        //defining a vector for the pair of integers
        let pair_of_ints = x
            .split(" ")
            .map(|component| component.parse::<u64>().unwrap())
            .collect::<Vec<u64>>(); 

        //To check which number is greater than the other
            if pair_of_ints[1] > pair_of_ints[0] {
            let difference = pair_of_ints[1] - pair_of_ints[0];
            println!("{}", difference);
        }
        else {
            let difference = pair_of_ints[0] - pair_of_ints[1];
            println!("{}", difference);
        }
    }
 }