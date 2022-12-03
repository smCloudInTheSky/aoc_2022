use aoc::util;

fn read_game(path: &str) -> i32 {
    let mut final_score: i32 = 0 ;
    let lines = util::read_lines(path).unwrap() ;
    for (_index, line) in lines.enumerate() {
        let play: String = line.unwrap().parse().unwrap() ; //on récup le résultat du round
        let mut game_state = String::new() ;
        for play in play.split(' ') {
            if play == "A" || play == "X" {
                game_state.push('1') ; 
            } else if play == "B" || play == "Y" {
                game_state.push('2') ;
            } else if play == "C" || play == "Z" {
                game_state.push('3') ;
            }
        }
        match game_state.as_str() {
            "11" | "22" | "33" => final_score = final_score + 3 + game_state.pop().expect("Un int").to_string().parse::<i32>().unwrap(),
            "12" | "23" | "31" => final_score = final_score + 6 + game_state.pop().expect("Un int").to_string().parse::<i32>().unwrap(),
            "13" | "21" | "32" => final_score = final_score + game_state.pop().expect("Un int").to_string().parse::<i32>().unwrap(),
            _ => println!("empty"),

        }
    }
    final_score 
}

fn read_game_smart(path: &str) -> i32 {
    let mut final_score: i32 = 0 ;
    let lines = util::read_lines(path).unwrap() ;
    for (_index, line) in lines.enumerate() {
        let play: String = line.unwrap().parse().unwrap() ; //on récup le résultat du round
        let mut player_win = 0i32 ;
        let mut player_lose = 0i32 ;
        let mut player_draw = 0i32 ;
        for played in play.split(' ') {
            match played {
                "A" => { 
                    player_draw = 1 ;
                    player_win = 2 ;
                    player_lose = 3 ;
                },
                "B" => {
                    player_draw = 2 ;
                    player_win = 3 ;
                    player_lose = 1 ;
                },
                "C" => {
                    player_draw = 3 ;
                    player_win = 1 ;
                    player_lose = 2 ;
                },
                "X" => final_score = final_score + player_lose ,
                "Y" => final_score = final_score + 3 + player_draw ,
                "Z" => final_score = final_score + 6 + player_win ,
                _ => println!("empty ne doit pas arriver"),
            }
        }
    }
    final_score 
}

fn main () {
  let result_p1 = read_game("input/day2_p1.txt") ;
  let result_p2 = read_game_smart("input/day2_p1.txt") ;
  println!("result : {}",result_p1);
  println!("result : {}",result_p2);

}



