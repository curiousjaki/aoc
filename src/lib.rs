pub mod day1;
//#![feature(test)]
//extern crate test;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_day1_a() {
        let (distance, _l1, _l2) = day1::a("./data/day1.txt");
        assert!(distance == 2375403);
    }
    #[test]
    fn test_day1_b() {
        let (distance, l1, l2) = day1::a("./data/day1.txt");
        assert!(day1::b(&l1, &l2) == 23082277);
    }
    
    //#[bench]
    //fn bench_day1_B(b: &mut Bencher) {
    //    let (distance, l1, l2) = day_1_A("./data/day1.txt");
    //    b.iter(|| day_1_B(&l1,&l2));
    //    assert!(day_1_B(&l1, &l2) == 23082277);
    //}
}
