use std::fs;

fn main() {
    let puzzle = fs::read_to_string("./src/puzzle_input.txt").expect("Error reading puzzle_input.txt");
    let tot_score = get_total_score(&puzzle);
    println!("tot score {}", tot_score);
}

fn get_round_score(opponent_round_enc: &str, me_round_enc: &str) -> u32 {
    let mut score_me = match me_round_enc {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => panic!("malformed data"),
    };

    let score_opponent = match opponent_round_enc {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        &_ => panic!("malformed data")
    };

    if score_me == 1 && score_opponent == 3 {
       score_me += 6
    } else if score_opponent == 1 && score_me == 3 {
        score_me += 0
    } else if score_me > score_opponent {
        // win
        score_me += 6;
    } else if score_me == score_opponent {
        // draw
        score_me += 3;
    }

    score_me
}

fn get_total_score(rounds: &str) -> u32 {
    let mut score_me_tot = 0;

    for round in rounds.lines() {
        let round_lr: Vec<&str> = round.split(char::is_whitespace).collect();

        let opponent_round_enc = round_lr[0];
        let me_round_enc = round_lr[1];

        let score_me = get_round_score(opponent_round_enc, me_round_enc);

        score_me_tot += score_me;

        println!("{}", score_me_tot);
    }

    score_me_tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_score() {
        let test_input = r#"A Y
B X
C Z"#;

        let total_score = get_total_score(test_input);

        assert_eq!(total_score, 15);
    }

    #[test]
    fn test_get_total_score_ext() {
        let inp = fs::read_to_string("./src/test2.txt").expect("error reading file");
        let ts = get_total_score(&inp);
        assert_eq!(ts, 58);
    }

    #[test]
    fn test_get_round_score() {
        let t = get_round_score("A", "X");
        assert_eq!(t, 4);

        let t1 = get_round_score("B", "X");
        assert_eq!(t1, 1);

        let t2 = get_round_score("C", "Y");
        assert_eq!(t2, 2);
    }
}