use std::{io, collections::HashMap};
fn main() {
    let mut numbers :Vec<i32> = Vec::new();
    println!("Input a number or \"finish\" to stop");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "finish"{
            break;
        }
        let input :i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Not a number");continue},
        };
        numbers.push(input);
    }
    if numbers.len() == 0{
        panic!("No number was given");
    }
    numbers.sort();
    let sum :i32 = numbers.iter().sum();
    let average :f32 = (sum as f32 / numbers.len() as f32) as f32;
    let median :f32 = match numbers.len() % 2 {
        0 => (
                match numbers.get(numbers.len() / 2){Some(res) => res, None => panic!()} 
                + 
                match numbers.get((numbers.len() / 2) - 1){Some(res) => res, None => panic!()}
            ) as f32 / 2.0,
        _ => match numbers.get(numbers.len() / 2){Some(res) => *res as f32, None => panic!()}
    };
    let dominant :i32 = {
        let mut all_number_count : HashMap<i32,i32> = HashMap::new();
        for num in &numbers{
            let count = all_number_count.entry(*num).or_insert(0);
            *count +=1;
        }
        let mut max_count : i32 = 0;
        let mut key : Option<i32> = None;
        for pair in all_number_count{
            if pair.1 > max_count{
                max_count = pair.1;
                key = Some(pair.0);
            }
        }
        match key {
            Some(res) => res,
            None => panic!(),
        }
    };
    println!("Sorted vecotr: {:?}",numbers);
    println!("Avarage = {average}, \nMedian = {median}, \nDominant = {dominant}")
}
