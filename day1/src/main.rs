use std::fs;

fn main() {
    let puzzle = fs::read_to_string("./src/puzzle_input.txt").expect("Error reading puzzle_input.txt");
    let largest = calories_list_group_get_largest_group_amount(puzzle.as_str());
    println!("Answer: {}", largest);
}

fn calories_list_group_get_largest_group_amount(calories_groups_and_blank_lines: &str) -> i32 {
    let mut largest_calories_group = 0;
    let mut pers_calories = 0;

    for lines in calories_groups_and_blank_lines.lines() {
        let parsed_line = lines.parse::<i32>();
        match parsed_line {
            Ok(calories) => {
                pers_calories += calories;

                if calories > largest_calories_group {
                    largest_calories_group = pers_calories;
                }
            }
            Err(_) => {
                // Reset calories
                pers_calories = 0;
            }
        }

        if pers_calories > largest_calories_group {
            largest_calories_group = pers_calories
        }
    }

    largest_calories_group
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calories_list_group_get_largest_group_amount() {
        let calories_group_per_elf = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let largest_calories_group_amount = calories_list_group_get_largest_group_amount(calories_group_per_elf);

        assert_eq!(largest_calories_group_amount, 24000);
    }
}