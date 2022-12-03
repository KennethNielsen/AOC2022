
// Code to solve the advent of code task for Dec 1

use std::fs;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("This program takes the puzzle input file as a parameter");
        return
    }
    let input_file = &args[1];
    println!("Input: {}", input_file);

    let content = fs::read_to_string(input_file).expect("No input file in this dir");

    let mut elves: Vec<i32> = Vec::new();
    // SKip elf 0, since the assignment is counting from 1
    elves.push(-1);
    // Initialize elf 1, since we never move into that elf
    elves.push(0);

    // Sum the calories for each elf
    let mut present_elf = 1;
    for item in content.split("\n"){
        if item == ""{
            present_elf += 1;
            elves.push(0);
            assert!(elves.len() == present_elf + 1);
            continue
        }
        let value: i32 = item.parse().unwrap();
        elves[present_elf] += value;
        // print!("This item: {}", item)
    }

    // Find the index of max
    let maximum_calories = elves.iter().max().unwrap();
    let maximum_calory_elf = elves.iter().position(|&x| x == *maximum_calories).unwrap();

    println!("Max cals is: {} for elf: {}", maximum_calories, maximum_calory_elf);

    // Part 2, find the sum of the top 3
    elves.sort();
    let top3_sum: i32 = elves.iter().rev().take(3).sum();
    println!("The sum of the top 3: {}", top3_sum);


}
