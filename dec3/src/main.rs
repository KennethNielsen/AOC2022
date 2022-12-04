
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("This program takes the puzzle input file as a parameter");
        return
    }
    let input_file = &args[1];
    println!("Input: {}", input_file);

    let f = fs::File::open(input_file).unwrap();
    let buffered_reader = BufReader::new(f);

    // For part 1
    let mut combined_priority: u32 = 0;
    // For part 2
    let mut combined_group_priority = 0;
    let mut group: Vec<String> = Vec::new();

    for line_result in buffered_reader.lines(){
        let line = line_result.unwrap();

        // Find item in both compartments and assign priority
        let in_both = identify_item_in_both_compartments(&line);
        combined_priority += assign_priority(&in_both);
    
        // Build the groups
        group.push(line);
        if group.len() == 3 {
            let badge = identify_badge(group);
            combined_group_priority += assign_priority(&badge);
            group = Vec::new();
        }

    }

    println!("The combined priority is: {}", combined_priority);
    println!("The combined group priority is: {}", combined_group_priority);

}


fn identify_item_in_both_compartments(items: &String) -> char {
    let split_point = items.len() / 2;
    let first_half = &items[..split_point];
    let second_half = &items[split_point..];
    assert!(first_half.len() == second_half.len(), "Odd number of elements");

    for c in first_half.chars(){
        if second_half.contains(c){
            return c
        }
    }
    panic!("Didn't find repeat item");
}


fn assign_priority(item: &char) -> u32{
    /* Convert to priority where a-z = 1-26 and A-Z = 27-52 */
    // ASCII a is 97 and A is 65
    //
    let ascii_value = *item as u32;
    return match ascii_value {
        (65..=90) => ascii_value - 38,
        (97..=122) => ascii_value - 96,
        _ => panic!("Not in a-z or A-Z")
    }
}

fn identify_badge(group: Vec<String>) -> char {
    assert!(group.len() == 3, "Not a group of 3");
    for c in group[0].chars(){
        if group[1].contains(c) && group[2].contains(c) {
            return c
        }
    }
    panic!("Badge not found");
}