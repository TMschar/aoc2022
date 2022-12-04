use std::fs;

fn main() {
    let input = fs::read_to_string("./src/prod_input.txt")
        .expect("Error reading prod_input.txt");
    let answer = get_group_item_types(&input);
    println!("{}", answer);
}

fn get_priority(char: char) -> usize {
    let start: usize;
    if char.is_lowercase() {
        start = 97
    } else if char.is_uppercase() {
        start = 65
    } else {
        panic!("malformed data");
    }

    let ascii_decimal = char as usize;
    let mut priority = ascii_decimal - start + 1;

    if char.is_uppercase() {
        priority += 26;
    }

    priority
}

fn find_item_in_both_comp1(comp1: &str, comp2: &str, comp3: &str) -> Vec<char> {
    let mut items_in_both: Vec<char> = vec![];

    for item in comp1.chars() {
        if comp2.find(item).is_some() && comp3.find(item).is_some() {
            items_in_both.push(item);
            return items_in_both
        }
    }

    items_in_both
}

fn get_group_item_types(input: &str) -> usize {
    let mut i = 0;
    let mut group_lines = vec![];
    let mut total_prio = 0;

    for line in input.lines() {
        i += 1;

        group_lines.push(line);

        if i == 3 {
            let char = find_item_in_both_comp1(group_lines[0], group_lines[1], group_lines[2])[0];
            let priority = get_priority(char);

            // Increase
            total_prio += priority;

            // Reset
            i = 0;
            group_lines = vec![];
        }
    }

    total_prio
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('p'), 16);
        assert_eq!(get_priority('L'), 38);
        assert_eq!(get_priority('v'), 22);
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("./src/test_input.txt")
            .expect("Error reading test_input.txt");
        let answer = get_group_item_types(&input);
        assert_eq!(answer, 70);
    }
}
