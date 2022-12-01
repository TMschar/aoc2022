use std::fs;

fn main() {
    let puzzle = fs::read_to_string("./src/puzzle_input.txt").expect("Error reading puzzle_input.txt");
    let largest_three_groups = get_top_three_groups_tot_kcal(puzzle.as_str());
    println!("Answer: {}", largest_three_groups);
}

fn get_top_three_groups_tot_kcal(calories_groups_and_blank_lines: &str) -> i32 {
    let mut elfs_calories: Vec<i32> = Vec::new();
    let mut pers_calories = 0;

    // Into vector
    for lines in calories_groups_and_blank_lines.lines() {
        let parsed_line = lines.parse::<i32>();
        match parsed_line {
            Ok(calories) => {
                pers_calories += calories;
            }
            Err(_) => {
                elfs_calories.push(pers_calories);
                // Reset calories
                pers_calories = 0;
            }
        }
    }

    elfs_calories.push(pers_calories);

    // Switch around order, min to max
    elfs_calories.sort_by(|a, b| b.cmp(a));

    elfs_calories[0] + elfs_calories[1] + elfs_calories[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_top_three_groups_tot_kcal() {
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

        let top_three_groups_tot_kcal = get_top_three_groups_tot_kcal(calories_group_per_elf);

        assert_eq!(top_three_groups_tot_kcal, 45000);
    }
}