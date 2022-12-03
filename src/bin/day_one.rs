use aoc::util;

fn read_cal(path: &str) -> i32 {
    let mut max_cal: i32 = 0 ;
    let mut cur_cal: i32 = 0 ;
    let lines = util::read_lines(path).unwrap() ;
    for (_index, line) in lines.enumerate() {
        let val: String = line.unwrap().parse().unwrap();
        if val.trim().is_empty() {
            if max_cal < cur_cal {
                max_cal = cur_cal ;
                cur_cal = 0 ;
            } else {
                cur_cal = 0 ;
            } 
        } else {
            let value: i32 = val.parse().unwrap() ;
            println!("val : {}",value) ;
            cur_cal = cur_cal + value ; 
        }
    }
    max_cal
}

fn read_cal_p2(path: &str) -> i32 {
    let mut first_cal: i32 = 0 ;
    let mut sec_cal: i32 = 0 ;
    let mut third_cal: i32 = 0 ;
    let mut cur_cal: i32 = 0 ;
    let lines = util::read_lines(path).unwrap() ;
    for (_index, line) in lines.enumerate() {
        let val: String = line.unwrap().parse().unwrap();
        if val.trim().is_empty() {
            if first_cal < cur_cal {
                third_cal = sec_cal ; 
                sec_cal = first_cal ; 
                first_cal = cur_cal ;
            } else {
                if sec_cal < cur_cal { 
                    third_cal = sec_cal ;
                    sec_cal = cur_cal ;
                } else { 
                    if third_cal < cur_cal {
                        third_cal = cur_cal ; 
                    }
                }
            } 
            cur_cal = 0 ;
        } else {
            let value: i32 = val.parse().unwrap() ;
            println!("val : {}",value) ;
            cur_cal = cur_cal + value ; 
        }
    }
    first_cal + sec_cal + third_cal
}
fn main () {
  let result_p1 = read_cal("input/day1_p1.txt") ;
  let p1 = read_cal_p2("input/day1_p1.txt") ; 
  println!("result : {}",result_p1);
  println!("result : {}",p1);

}



