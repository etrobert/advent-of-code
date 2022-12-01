use std::fs;

fn parse_elf(elf: &str) -> u64 {
    return elf
        .lines()
        .map(|snack| snack.parse::<u64>().expect("Not a number"))
        .sum();
}

fn main() {
    let file_text = fs::read_to_string("../input").expect("Unable to read file");
    let mut elfs: Vec<u64> = file_text
        .split("\n\n")
        .map(parse_elf)
        .collect::<Vec<u64>>();

    elfs.sort_by(|a, b| b.cmp(a));

    // Print the maximum elf
    println!("Biggest elf: {}", elfs.iter().max().expect("No elfs"));

    // Get the first 3 elements of elfs
    let first_three = elfs.iter().take(3);

    // Print the sum of the first 3 elements of elfs
    println!("Biggest three elves: {}", first_three.sum::<u64>());
}
