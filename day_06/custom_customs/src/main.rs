use std::{ error::Error, fs };

fn main() {
    let input = import_groups("input.txt").unwrap();
    let sum = get_groups_sum(&input);

    println!("Total count sum: {}", sum);
}

fn import_groups(input_text: &str) -> Result<String, Box<dyn Error>>{
    let input = fs::read_to_string(input_text)?;

    Ok(input)
}

fn get_groups_sum(group_string: &str) -> usize {
    let mut sum = 0;
    for group in group_string.split(|c: char| c == '-'){
        sum += get_unique_count(group);
    }

    sum
}

fn get_unique_count(group: &str) -> usize {
    let mut unique_chars: Vec<char> = Vec::new();

    for c in group.split(|c: char| c.is_whitespace()) {
        for ch in c.chars(){
            if !unique_chars.contains(&ch) {
                unique_chars.push(ch);
            }
        }
    }

    unique_chars.iter().count()
}

#[test]
fn group_sum(){
    let groups = "abc
    -
    a
    b
    c
    -
    ab
    ac
    -
    a
    a
    a
    a
    -
    b";

    assert_eq!(
        11,
        get_groups_sum(groups)
    );
}

#[test]
fn group_vertical(){
    let group = "a
    b
    c";

assert_eq!(
        3,
        get_unique_count(group)
    );
}

#[test]
fn group_horizontal(){
    let group = "ab";

    assert_eq!(
        2,
        get_unique_count(group)
    );
}

#[test]
fn group_repeating() {
    let group = "aaa";

    assert_eq!(
        1,
        get_unique_count(group)
    )
}