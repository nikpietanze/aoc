use std::fs;

fn main() {
    let a_opts = ["A", "B", "C"];
    let b_opts = ["X", "Y", "Z"];
    let mut part_one: Vec<usize> = vec![];
    let mut part_two: Vec<usize> = vec![];

    let input = fs::read_to_string("./src/input.txt").unwrap();
    let games = input.split("\n");
    for game in games {
        if !game.is_empty() {
            let pairs: Vec<&str> = game.split(" ").collect();
            let a = a_opts.iter().position(|&r| r == pairs[0]).unwrap();
            let b = b_opts.iter().position(|&r| r == pairs[1]).unwrap();

            if a == 0 && b == 1 || a == 1 && b == 2 || a == 2 && b == 0 {
                part_one.push(6 + b + 1);
            } else if a == b {
                part_one.push(3 + b + 1);
            } else {
                part_one.push(b + 1);
            }

            if b == 0 {
                if a == 0 {
                    part_two.push(3)
                } else {
                    part_two.push(a);
                }
            } else if b == 1 {
                part_two.push(3 + a + 1);
            } else if a == 2 {
                part_two.push(7)
            } else {
                part_two.push(6 + a + 2);
            }
        }
    }

    let part_one_answer: usize = part_one.iter().sum();
    let part_two_answer: usize = part_two.iter().sum();
    println!("Part 1: {}", part_one_answer);
    println!("Part 2: {}", part_two_answer);
}
