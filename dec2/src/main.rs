
use std::fs::File;
use std::io::{BufReader, BufRead};


fn decrypt_1<'a>(opponent: &'a str, me: &'a str) -> (&'a str, &'a str){
    // Decrypt opponent and me keys and return as Strings
    let opponent_decrypt = match opponent {
        "A" => "Rock",
        "B" => "Paper",
        "C" => "Scissors",
        &_ => panic!("Unknown opponent key: {}", opponent),
    };
    let me_decrypt = match me {
        "X" => "Rock",
        "Y" => "Paper",
        "Z" => "Scissors",
        &_ => panic!("Unkonwn me key: {}", me)
    };

    return (opponent_decrypt, me_decrypt)
}

fn score_of_round(opponent: &str, me: &str) -> i32 {
    // Assign initial score based on hand
    let mut score = match me {
        // 1 for Rock, 2 for Paper, and 3 for Scissors
        "Rock" => 1,
        "Paper" => 2,
        "Scissors" => 3,
        &_ => panic!("Unrecognizable shape {}", me),
    };

    if opponent == me {
        score += 3;
    } else {
        score += match (opponent, me) {
            ("Rock", "Paper") | ("Paper", "Scissors") | ("Scissors", "Rock") => 6,
            ("Paper", "Rock") | ("Scissors", "Paper") | ("Rock", "Scissors") => 0,
            (&_, _) => panic!("Unknown case: {} <> {}", opponent, me),
        };
    }
    
    return score
}


fn decrypt_2<'a>(opponent: &'a str, desired_result: &'a str) -> (&'a str, &'a str){
    // Decrypt opponent and me keys and return as Strings
    let opponent_decrypt = match opponent {
        "A" => "Rock",
        "B" => "Paper",
        "C" => "Scissors",
        &_ => panic!("Unknown opponent key: {}", opponent),
    };

    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
    let desired_result_decrypt = match desired_result {
        "X" => "Loose",
        "Y" => "Draw",
        "Z" => "Win",
        &_ => panic!("Unknown desired outcome: {}", desired_result)
    };

    return (opponent_decrypt, desired_result_decrypt)
}


fn calculate_shape<'a>(opponent: &'a str, desired_result: &'a str) -> &'a str {
    return if desired_result == "Draw" {
        opponent
    } else {
        match (opponent, desired_result) {
            ("Rock", "Win") | ("Scissors", "Loose") => "Paper",
            ("Paper", "Win") | ("Rock", "Loose") => "Scissors",
            ("Scissors", "Win") | ("Paper", "Loose") => "Rock",
            (&_, _) => panic!("Unkonwn calc shape input: {} {}", opponent, desired_result),
        }
    };
}


fn main() {
    {  // Scope to ensure that files are closed
        // Open a buffered reader on the puzzle input
        let f = File::open("puzzle_input.txt").unwrap();
        let buffered_reader = BufReader::new(f);

        // Loop over lines in that file and calc score
        let mut total_score = 0;
        for line_result in buffered_reader.lines(){
            let line = match line_result {
                Err(_e) => break,
                Ok(v) => v,
            };

            // Extract opponent and my 
            let mut fields = line.split(" ");
            let mut opponent = fields.next().unwrap();
            let mut me = fields.next().unwrap();

            (opponent, me) = decrypt_1(opponent, me);

            total_score += score_of_round(opponent, me);
        }

        println!("Total score with decryption 1 is: {}", total_score);
    }

{  // Scope to ensure that files are closed
        // Open a buffered reader on the puzzle input
        let f = File::open("puzzle_input.txt").unwrap();
        let buffered_reader = BufReader::new(f);

        // Loop over lines in that file and calc score
        let mut total_score = 0;
        for line_result in buffered_reader.lines(){
            let line = match line_result {
                Err(_e) => break,
                Ok(v) => v,
            };

            // Extract opponent and my 
            let mut fields = line.split(" ");
            let mut opponent = fields.next().unwrap();
            let mut desired_result = fields.next().unwrap();

            (opponent, desired_result) = decrypt_2(opponent, desired_result);
            let me = calculate_shape(opponent, desired_result);
            // println!("{} {} {}", opponent, desired_result, me);
            total_score += score_of_round(opponent, me);
        }

        println!("Total score with decryption 2 is: {}", total_score);
    }

}
