pub mod day1;
pub mod day2; 
//#![feature(test)]
//extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;
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

    #[test]
    fn test_day2_a() {
        let (v, counter) =  day2::a("./data/day2.txt", false);
        assert_eq!(counter,572);
    }

    #[test]
    fn test_day2_b() {
        let (v, counter) =  day2::a("./data/day2.txt", true);
        assert_eq!(counter,612);
    }
    
    
    //#[bench]
    //fn bench_day1_B(b: &mut Bencher) {
    //    let (distance, l1, l2) = day_1_A("./data/day1.txt");
    //    b.iter(|| day_1_B(&l1,&l2));
    //    assert!(day_1_B(&l1, &l2) == 23082277);
    //}
}