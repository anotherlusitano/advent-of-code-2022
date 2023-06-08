use std::fs;
use std::str;

fn main() {
    let file_contents =
        fs::read_to_string("calories_list.txt").expect("Should have been able to read the file");

    let contents: Vec<&str> = file_contents.lines().collect();

    let mut vec: Vec<i32> = Vec::new();
    let mut vec2: Vec<Vec<i32>> = Vec::new();

    for x in contents.clone() {
        match x {
            "" => {
                vec2.push(vec.clone());
                vec.clear();
            }
            _ => vec.push(x.parse().unwrap()),
        }
    }

    let mut final_result: i32 = 0;

    for x in vec2.iter() {
        if final_result < x.iter().sum() {
            final_result = x.iter().sum();
        }
        vec.push(x.iter().sum());
    }

    vec.sort();

    println!("1º = {:?}", vec[vec.len() - 1]);
    println!("2º = {:?}", vec[vec.len() - 2]);
    println!("3º = {:?}", vec[vec.len() - 3]);

    let sum_of_top_3: i32 = vec[vec.len() - 1] + vec[vec.len() - 2] + vec[vec.len() - 3];
    println!("{}", sum_of_top_3);
}
