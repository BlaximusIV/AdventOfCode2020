use std::{error::Error, fs};

struct AuditItem {
    target_char: char,
    char_min: u8,
    char_max: u8,
    password: String,
}

impl AuditItem {
    fn is_valid(&self) -> bool {
        let mut target_count = 0;

        for character in self.password.chars(){
            if character == self.target_char {
                target_count += 1;
            }
        }

        target_count >= self.char_min && target_count <= self.char_max
    }
}

/// given a list of password policies and the passwords created under them
/// find how many passwords are correct in the given list (input.txt)
fn main() {
    // retrieve list
    let audit_items = get_audits("input.txt");

    // Find how many meet their audit criteria
    let valid_password_count = get_valid_count(audit_items.unwrap());
    print!("{} valid password items", valid_password_count);
}

fn get_audits(input_file: &str) -> Result<Vec<AuditItem>, Box<dyn Error>>{
    let input = fs::read_to_string(input_file)?;

    let items: Vec<AuditItem> = input
        .lines()
        .map(|l| { parse_audit(l) })
        .collect();

    Ok(items)
}

fn parse_audit(audit_line: &str) -> AuditItem {
    let mut target_char :char = 'a';
    let mut char_min: u8 = 0;
    let mut char_max: u8 = 0;
    let mut password: String = String::new();
    
    // we know the exact structure of the input document, so we can parse by index
    let props: Vec<&str> = audit_line.split(' ').collect();
    
    for prop in 0..props.iter().count(){
        match prop {
            0 => {
                let min_max: Vec<&str> = props[prop].split('-').collect();
                char_min = min_max[0].parse().unwrap();
                char_max = min_max[1].parse().unwrap();
            },
            1 => target_char = props[prop]
                    .chars()
                    .nth(0)
                    .unwrap(),
            2 => password = props[prop].to_string(),
            _ => println!("Unexpected index {}", prop)
        }
    }

    AuditItem {
        target_char,
        char_min,
        char_max,
        password,
    }
}

fn get_valid_count(items: Vec<AuditItem>) -> usize {
    items.iter()
        .filter(|item| item.is_valid())
        .count()
}