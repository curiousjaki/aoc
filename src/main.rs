fn main() {
    let (distance, l1, l2) = aoc::day_1_A("./data/day1.txt");
    println!("{:?}", distance);
    let result = aoc::day_1_B(&l1,&l2);
    println!("{:?}", result);
}