use std::{fs::{self, read}, path::Path};
fn read_reports(file_name : &str) -> Vec<Vec<i32>>{
    let data: String = fs::read_to_string(Path::new(file_name)).expect("Should habe been able to read the file.");
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for report in data.lines() {
        let string: Vec<&str> = report.split_whitespace().collect();
        let mut numbers: Vec<i32> = Vec::new();
        for s in string{
            numbers.push(s.parse::<i32>().unwrap());
        }
        reports.push(numbers);
    }
    return reports;
}

///removed one determines wether, the solution allows the recursive yes/ no of option b if true we use the version a
fn check_safe(report : &Vec<i32>, removed_one: bool) -> bool{
    let mut previous: i32 = report[0];
    let acending = report[report.len()-1] > report[0];
    for (index,r) in report.iter().enumerate(){
        if index > 0 {
            if ((r > &previous) != acending) || !((1 <= (r- &previous).abs()) && ((r-&previous).abs()<=3)) {
                //println!("{:?}", report);
                if !removed_one {
                    let mut opt_a = report.clone();
                    let mut opt_b = report.clone();
                    opt_a.remove(index-1);
                    opt_b.remove(index);

                    //println!("{:?}, {:?}, {:?}", &report, &opt_a, &opt_b);
                    let solution_a = check_safe(&opt_a, true);
                    let solution_b = check_safe(&opt_b,true);
                    if solution_a||solution_b{
                        return true;
                    }
                    //println!("{:?} not safe, opta {:?} optb{:?}", report, opt_a, opt_b);
                    return false; //check_safe(&opt_a, true) || check_safe(&opt_b, true)
                }
                return false;
            }
        }
        previous = *r;
    }
    return true;

}

pub fn a(file_name : &str, is_actually_b : bool) -> (Vec<usize>,u32){
    let reports = read_reports(file_name);

    let mut un: Vec<usize> = Vec::new();
    let mut counter : u32 = 0;
    for (index, report) in reports.iter().enumerate(){
        if check_safe(&report, !is_actually_b) { //a = 572
            counter += 1;
        }else{
            un.push(index);
            //println!("{:?} not safe due to  {:?}", index, report);
        }
    }
    //println!("{:?}",un);
    return (un,counter)
}