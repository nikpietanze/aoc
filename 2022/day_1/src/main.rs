use std::fs;

fn get_input() -> String {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("error reading file");
    return contents;
}

fn main() {
    let input = get_input();
    let cals = input.split("\n\n");
    let mut totals: Vec<i32> = vec![];
    for elf in cals {
        let mut total = 0;
        let cal = elf.split("\n");
        for c in cal {
            if c != "" {
                total += c.parse::<i32>().unwrap();
            }
        }
        totals.push(total);
    }
    totals.sort_by(|a, b| b.cmp(a));
    let max = totals[0] + totals[1] + totals[2];
    println!("Part 1: {}", totals[0]);
    println!("Part 2: {}", max)
}
