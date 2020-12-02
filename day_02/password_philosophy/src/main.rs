use std::{error::Error, fs};

struct AuditItem {
    target_char: char,
    first_position: usize,
    second_position: usize,
    password: String,
}

impl AuditItem {
    fn is_valid(&self) -> bool {
        // This is optimistic, assuming password length aligns with policy indexes
        let char_at_first_index = self.password.chars()
            .nth(self.first_position - 1)
            .unwrap();

        let char_at_second_index = self.password.chars()
            .nth(self.second_position - 1)
            .unwrap();

        (char_at_first_index == self.target_char) ^ (char_at_second_index == self.target_char)
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
    let mut first_position: usize = 0;
    let mut second_position: usize = 0;
    let mut password: String = String::new();
    
    // we know the exact structure of the input document, so we can parse by index
    let props: Vec<&str> = audit_line.split(' ').collect();
    
    for prop in 0..props.iter().count(){
        match prop {
            0 => {
                let min_max: Vec<&str> = props[prop].split('-').collect();
                first_position = min_max[0].parse().unwrap();
                second_position = min_max[1].parse().unwrap();
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
        first_position,
        second_position,
        password,
    }
}

fn get_valid_count(items: Vec<AuditItem>) -> usize {
    items.iter()
        .filter(|item| item.is_valid())
        .count()
}