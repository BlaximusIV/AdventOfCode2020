use std::{ collections::HashMap, error::Error, fs };

// Find count of letters that all parties in a group chose
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
        let count = get_agreed_count(group);
        sum += count;
    }

    sum
}

fn get_agreed_count(group: &str) -> usize {
    let group = group.trim();
    let mut group_chars: HashMap<char, usize> = HashMap::new();

    for line in group.lines() {
        for c in line.trim().chars() {
            let count = group_chars.entry(c).or_insert(0);
            *count += 1;
        }
    }

    let mut unanimous_count = 0;
    for (_key, value) in &group_chars {
        if *value == group.lines().count() {
            unanimous_count += 1;
        }
    }

    unanimous_count
}

#[test]
fn group_sum() {
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
        6,
        get_groups_sum(groups)
    );
}

#[test]
fn agreed_count_single() {
    let group = "abc";

    assert_eq!(
        3,
        get_agreed_count(group)
    );
}

#[test]
fn agreed_count_unique(){
    let group = "a
    b
    c";

    assert_eq!(
        0,
        get_agreed_count(group)
    );
}

#[test]
fn agreed_count_mix(){
    let group = "ab
    ac";

    assert_eq!(
        1,
        get_agreed_count(group)
    )
}

#[test]
fn agreed_count_many() {
    let group = "a
    a
    a
    a";

    assert_eq!(
        1,
        get_agreed_count(group)
    );
}

#[test]
fn agreed_count_single_2() {
    let group = "b";

    assert_eq!(
        1,
        get_agreed_count(group)
    );
}