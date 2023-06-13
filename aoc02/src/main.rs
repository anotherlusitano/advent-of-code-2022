use std::fs;

fn main() {
    let file_contents =
        fs::read_to_string("strategy_guide.txt").expect("Should have been able to read the file");

    let mut points: i32 = 0;

    for x in file_contents.lines().into_iter() {
        let some = x.split_ascii_whitespace().collect::<Vec<_>>();
        match some[..] {
            ["A", "Y"] => points += 8, // WIN
            ["A", "X"] => points += 4, // DRAWN
            ["A", "Z"] => points += 3, // LOSE
            ["B", "Y"] => points += 5, // DRAWN
            ["B", "X"] => points += 1, // LOSE
            ["B", "Z"] => points += 9, // WIN
            ["C", "Y"] => points += 2, // LOSE
            ["C", "X"] => points += 7, // WIN
            ["C", "Z"] => points += 6, // DRAWN
            _ => println!("Error idk"),
        }
    }

    println!("Total points = {}", points);
}
