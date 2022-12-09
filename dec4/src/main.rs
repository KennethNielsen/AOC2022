
use std::env;
use std::io::{BufReader,BufRead};
use std::fs;


struct Area {
    start: i32,
    end: i32,
}

impl Area {
    fn one_within_other(&self, other: &Area) -> bool {
        if self.start == other.start || self.end == other.end {
            return true
        } else  {
            let other_in_self = self.start < other.start && other.end < self.end;
            let self_in_other = other.start < self.start && self.end < other.end;
            return other_in_self || self_in_other
        }
    }

    fn overlaps(&self, other: &Area) -> bool {
        /* There is some overlap if:
        
            One if fully within the other
                OR
            Either of ones endpoints is within the other

        */
        return self.one_within_other(other) ||
               (other.start <= self.start && self.start <= other.end) ||
               (other.start <= self.end && self.end <= other.end)
        
    }

    fn from_string(self_as_string: String) -> Area {
        let split = self_as_string.split("-");
        let split_vec = split.collect::<Vec<&str>>();
        assert!(split_vec.len() == 2, "There must be exactly one start and end in each interval");
        let start: i32 = split_vec[0].parse::<i32>().unwrap();
        let end: i32 = split_vec[1].parse::<i32>().unwrap();
        return Area {
            start: start,
            end: end,
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("This program takes exactly one input")
    }
    let input_file = &args[1];
    println!("Input file: {}", input_file);
    let file = fs::File::open(input_file).unwrap();
    let buf_reader = BufReader::new(file);

    let mut fully_overlapping_areas = 0;
    let mut some_overlapping_areas = 0;
    for line_result in buf_reader.lines(){
        let line = line_result.unwrap();

        let areas = line.split(",").collect::<Vec<&str>>();
        assert!(areas.len() == 2, "There must be two areas per line");
        let area1 = Area::from_string(areas[0].to_string());
        let area2 = Area::from_string(areas[1].to_string());
        if area1.one_within_other(&area2){   fully_overlapping_areas += 1;   }
        if area1.overlaps(&area2) {   some_overlapping_areas += 1;   }

        println!("{}  -  {}", line, area1.overlaps(&area2));
    }

    println!("Number of fully overlapping areas: {}", fully_overlapping_areas);
    println!("Number of some overlapping areas: {}", some_overlapping_areas);
}
