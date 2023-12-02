use day1::{part_one, part_two};

fn main() {
    let data = std::fs::read("input.txt").unwrap();
    println!("part one result: {}", part_one(&data));
    println!("part two result: {}", part_two(&data));
}
