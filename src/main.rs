fn main() {
    let (v,counter) = aoc::day2::a("./data/day2.txt",false);
    println!("{:?}", counter);

}

fn _run_day_1() {
    let (distance, l1, l2) = aoc::day1::a("./data/day1.txt");
    println!("{:?}", distance);
    let result = aoc::day1::b(&l1,&l2);
    println!("{:?}", result);
}

fn _run_day_2() {
    let (v, counter) = aoc::day2::a("./data/day2.txt", false);
    println!("{:?}", counter);
    let (v, counter) = aoc::day2::a("./data/day2.txt", true);
    println!("{:?}", counter);
}