fn main() {
    let (distance, l1, l2) = aoc::day1::a("./data/day1.txt");
    println!("{:?}", distance);
    let result = aoc::day1::b(&l1,&l2);
    println!("{:?}", result);
}