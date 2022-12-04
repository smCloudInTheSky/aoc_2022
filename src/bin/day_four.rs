use aoc::util;

fn read_assign(path: &str) -> usize {
    let mut assign: usize = 0 ;
    let lines = util::read_lines(path).unwrap() ;
    for (_index, line) in lines.enumerate() {
        let val: String = line.unwrap().parse().unwrap();
        let mut val_iter = val.split(',');
        let first = val_iter.next().unwrap() ;
        let second = val_iter.next().unwrap() ;
        let mut first_iter = first.split('-');
        let mut second_iter = second.split('-');
        let right_elf1: i32 = first_iter.next().unwrap().parse().unwrap();
        let left_elf1: i32 = first_iter.next().unwrap().parse().unwrap();
        let right_elf2: i32 = second_iter.next().unwrap().parse().unwrap();
        let left_elf2: i32 = second_iter.next().unwrap().parse().unwrap();
        if right_elf1 >= right_elf2 && left_elf1 <= left_elf2 {
            assign = assign + 1; 
        } else if right_elf2 >= right_elf1 && left_elf2 <= left_elf1 {
            assign = assign + 1; 
        }
    }
    assign
}

fn read_full_assign(path: &str) -> usize {
    let mut assign: usize = 0 ;
    let lines = util::read_lines(path).unwrap() ;
    for (_index, line) in lines.enumerate() {
        let val: String = line.unwrap().parse().unwrap();
        let mut val_iter = val.split(',');
        let first = val_iter.next().unwrap() ;
        let second = val_iter.next().unwrap() ;
        let mut first_iter = first.split('-');
        let mut second_iter = second.split('-');
        let right_elf1: i32 = first_iter.next().unwrap().parse().unwrap();
        let left_elf1: i32 = first_iter.next().unwrap().parse().unwrap();
        let right_elf2: i32 = second_iter.next().unwrap().parse().unwrap();
        let left_elf2: i32 = second_iter.next().unwrap().parse().unwrap();
        println!("first {}, second {}",first, second);
        if ( right_elf1 >= right_elf2 && right_elf1 <= left_elf2 ) || ( left_elf1 >= right_elf2 && left_elf1 <= left_elf2 ) {
            println!("ajout");
            assign = assign + 1; 
            
        } else if ( right_elf2 >= right_elf1 && right_elf2 <= left_elf1 ) || ( left_elf2 >= right_elf1 && left_elf2 <= left_elf1 ) {
            println!("ajout");
            assign = assign + 1; 
        }
    }
    assign
}

fn main () {
  let result_p1 = read_assign("input/day4_p1.txt") ;
  let p2 = read_full_assign("input/day4_p1.txt") ; 
  println!("result : {}",result_p1);
  println!("result : {}",p2);

}
