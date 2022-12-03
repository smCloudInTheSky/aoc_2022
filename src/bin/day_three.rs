use aoc::util;

fn read_prio(path: &str) -> usize {
    let alphabet = (b'a'..=b'z')  
        .chain(b'A'..=b'Z') // Start as u8
        .filter_map(|c| {
            let c = c as char;                             // Convert to char
            if c.is_alphabetic() { Some(c) } else { None } // Filter only alphabetic chars
        })          
        .collect::<Vec<_>>();
    let mut prio: usize = 0 ;
    let lines = util::read_lines(path).unwrap() ;
    for (_index, line) in lines.enumerate() {
        let val: String = line.unwrap().parse().unwrap();
        let len = val.len() ;
        let ( first, second ) = val.split_at(len/2);
        for element in first.chars() {
            if second.contains(element) {
                prio = prio + alphabet.iter().position(|&r| r == element).unwrap() + 1 ;
                break ;
            }
        }
    }
    prio
}

fn read_group(path: &str) -> usize {
    let alphabet = (b'a'..=b'z')  
        .chain(b'A'..=b'Z') // Start as u8
        .filter_map(|c| {
            let c = c as char;                             // Convert to char
            if c.is_alphabetic() { Some(c) } else { None } // Filter only alphabetic chars
        })          
        .collect::<Vec<_>>();
    let mut prio: usize = 0 ;
    let lines = util::read_lines(path).unwrap() ;
    let mut elves: Vec<String> = vec!["".to_string() ; 3] ; // A vector containing the bags of a group of elves
    for (index, line) in lines.enumerate() {
        let modulo = index % 3 ;
        let val: String = line.unwrap().parse().unwrap();
        elves[modulo] = val.clone() ; 
        if modulo == 2 {
            for element in val.chars() {
                if elves[0].contains(element) && elves[1].contains(element) {
                    prio = prio + alphabet.iter().position(|&r| r == element).unwrap() + 1 ;
                    break ;
                }
            }
        }
    }
    prio
}

fn main () {
  let result_p1 = read_prio("2022/input/day3_p1.txt") ;
  let p2 = read_group("2022/input/day3_p1.txt") ; 
  println!("result : {}",result_p1);
  println!("result : {}",p2);

}
