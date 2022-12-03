use std::fs;

fn find_common_element<'a>(a: &'a Vec<&'a str>, b: &'a Vec<&'a str>) -> &'a str {
    for c in a {
        println!("c: {}", c);
        if b.contains(c) {
            return c;
        }
    }
    panic!("No common element found");
}

fn compute_rucksack(rucksack: &Vec<&str>) {
    let size = rucksack.len();
    let (a, b) = rucksack.split_at(size / 2);
    let vec_a = a.to_vec();
    let vec_b = b.to_vec();
    println!("{}, {}", a.len(), b.len());
    println!("{}, {}", a.join(""), b.join(""));
    println!("Common element: {}", find_common_element(&vec_a, &vec_b));
    // println!("Size: {}", size);
}

fn main() {
    let file_text = fs::read_to_string("input").expect("Unable to read file");
    let rucksack = file_text.lines();
    let items: Vec<Vec<&str>> = rucksack.map(|line| line.split("").collect()).collect();
    // items[0].iter().for_each(|item| println!("{}", item));
    // Print the size of items[0]
    compute_rucksack(&items[1]);
    // items.iter().for_each(compute_rucksack);

    // println!("{}", file_text);
}
