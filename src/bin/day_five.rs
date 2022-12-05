use aoc::util;

fn read_assign(path: &str) -> String {
    let mut answer: String = String::new() ;
    let lines = util::read_lines(path).unwrap() ;
    let mut vec_lines = vec![vec!['H','C','R'],vec!['B','J','H','L','S','F'],vec!['R','M','D','H','J','T','Q'],vec!['S','G','R','H','Z','B','J'],vec!['R','P','F','Z','T','D','C','B'],vec!['T','H','C','G'],vec!['S','N','V','Z','B','P','W','L'],vec!['R','J','Q','G','C'],vec!['L','D','T','R','H','P','F','S']] ;
    let mut setup: Vec<Vec<String>> = Vec::new() ;
    for (index, line) in lines.enumerate() {
        let content: String = line.unwrap().parse().unwrap() ;
            let element = content.split(' ') ;
            let mut date = vec![0;3];
            let mut pos: usize = 0;
            for (index, word) in element.enumerate() {
                if index%2 == 1 {
                    date[pos] = word.parse::<i32>().unwrap() ;
                    pos = pos + 1 ;
                }
            }
            let mut tmp_vec: Vec<String> = Vec::new() ; 
            for i in 0..date[0] {
                tmp_vec.push(vec_lines[date[1] as usize - 1 ].pop().unwrap().to_string()) ;
            }
            for element in tmp_vec {
                let vec_char: Vec<char> = element.chars().collect() ; 
                vec_lines[date[2] as usize - 1 ].push(vec_char[0]);
            }
    }
    for mut word in vec_lines {
        answer = answer + &word.pop().unwrap().to_string() ;
    }
    answer
}


fn read_9001(path: &str) -> String {
    let mut answer: String = String::new() ;
    let lines = util::read_lines(path).unwrap() ;
    let mut vec_lines = vec![vec!['H','C','R'],vec!['B','J','H','L','S','F'],vec!['R','M','D','H','J','T','Q'],vec!['S','G','R','H','Z','B','J'],vec!['R','P','F','Z','T','D','C','B'],vec!['T','H','C','G'],vec!['S','N','V','Z','B','P','W','L'],vec!['R','J','Q','G','C'],vec!['L','D','T','R','H','P','F','S']] ;
    let mut setup: Vec<Vec<String>> = Vec::new() ;
    for (index, line) in lines.enumerate() {
        println!("mouvement : {}", index);
        let content: String = line.unwrap().parse().unwrap() ;
            let element = content.split(' ') ;
            let mut date = vec![0;3];
            let mut pos: usize = 0;
            for (index, word) in element.enumerate() {
                if index%2 == 1 {
                    date[pos] = word.parse::<i32>().unwrap() ;
                    pos = pos + 1 ;
                }
            }
            let mut tmp_vec: Vec<char> = Vec::new() ; 
            let split_index = vec_lines[date[1] as usize - 1 ].len() - date[0] as usize;
            println!("move {} from {} to {}", date[0],date[1],date[2]);
            println!("from {} : {}",date[1],split_index);
            println!("to {} : {}",date[2],vec_lines[date[2] as usize - 1 ].len());
            tmp_vec = vec_lines[date[1] as usize - 1 ].split_off( split_index );
            vec_lines[date[2] as usize - 1 ].append(&mut tmp_vec);
    }
    for mut word in vec_lines {
        answer = answer + &word.pop().unwrap().to_string() ;
    }
    answer
}

fn main () {
  let result_p1 = read_assign("input/day5_p1.txt") ;
  let p2 = read_9001("input/day5_p1.txt") ; 
  println!("result : {}",result_p1);
  println!("result : {}",p2);
}
