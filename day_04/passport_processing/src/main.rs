use std::{error::Error, fs};
use regex::Regex;

/// Today's objective is to find the valid "passports" from the given input data
fn main() {
    // Get passports
    let input_content = get_input("input.txt").unwrap();

    // interperet passports
    let passports = parse_passports(&input_content);

    // count valid passports
    let valid_count = passports
        .iter()
        .filter(|p| p.is_valid())
        .count();

    // report valid passport count
    println!("Valid Passport Count: {}", valid_count);
}

struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<usize>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        is_valid_year(self.birth_year, 1920, 2002)
        && is_valid_year(self.issue_year, 2010, 2020)
        && is_valid_year(self.expiration_year, 2020, 2030)
        && is_valid_height(self.height.as_ref())
        && is_valid_hcl(self.hair_color.as_ref())
        && is_valid_ecl(self.eye_color.as_ref())
        && is_valid_pid(self.passport_id.as_ref())
    }
}

fn is_valid_year(year: Option<usize>, min: usize, max: usize) -> bool {
    year.is_some() 
    && year.unwrap() >= min 
    && year.unwrap() <= max
}

fn is_valid_height(height: Option<&String>)-> bool {
    if !height.is_some() {
        return false;
    }

    let height = height.as_ref().unwrap();

    let is_metric = height.contains("cm");
    let units: usize = height
        .to_string()
        .trim_end_matches(|s:char| s.is_alphabetic())
        .parse()
        .unwrap();

    if is_metric {
        units >=150 && units <= 193
    } else{
        units >= 59 && units <= 76
    }
}

fn is_valid_hcl(color: Option<&String>) -> bool {
    if !color.is_some() {
        return false;
    }

    let regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    regex.is_match(color.unwrap())
}

fn is_valid_ecl(color: Option<&String>) -> bool {
    if !color.is_some() {
        return false;
    }

    let valid_colors: Vec<String> = vec![
        "amb".to_string(),
        "blu".to_string(),
        "brn".to_string(),
        "gry".to_string(),
        "grn".to_string(),
        "hzl".to_string(),
        "oth".to_string()];

    valid_colors.contains(color.as_ref().unwrap())
}

fn is_valid_pid(passport_id: Option<&String>) -> bool {
    if !passport_id.is_some() {
        return false;
    }

    let regex = Regex::new(r"^[0-9]{9}$").unwrap();
    regex.is_match(passport_id.as_ref().unwrap())
}

fn get_input(input_file: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(input_file)?)
}

fn parse_passports(input: &str) -> Vec<Passport>{
    let mut passports: Vec<Passport> = Vec::new();

    let passport_strings: Vec<&str> = input.split('-').collect();

    for passport in passport_strings.iter(){
        passports.push(parse_passport(passport));
    }

    passports
}

fn parse_passport(pass_string: &str) -> Passport {
    let fields: Vec<&str> = pass_string.trim()
                                        .split(|c:char | c.is_whitespace())
                                        .collect();
    
    let mut passport = Passport { 
        birth_year: Option::None, 
        issue_year: Option::None,
        expiration_year: Option::None,
        height: Option::None,
        hair_color: Option::None,
        eye_color: Option::None,
        passport_id: Option::None,
        country_id: Option::None,
    }                            ;            

    for field in fields.iter().filter(|s| !s.is_empty()){
        let field_value: Vec<&str> = field.trim().split(':').collect();

        match field_value[0] {
            "byr" => passport.birth_year = Option::Some(field_value[1].parse().unwrap()),
            "iyr" => passport.issue_year = Option::Some(field_value[1].parse().unwrap()),
            "eyr" => passport.expiration_year = Option::Some(field_value[1].parse().unwrap()),
            "hgt" => passport.height = Option::Some(field_value[1].to_string()),
            "hcl" => passport.hair_color = Option::Some(field_value[1].to_string()),
            "ecl" => passport.eye_color = Option::Some(field_value[1].to_string()),
            "pid" => passport.passport_id = Option::Some(field_value[1].to_string()),
            "cid" => passport.country_id = Option::Some(field_value[1].parse().unwrap()),
            _ => {
                println!("Problem field: {}", field_value[0]);
                panic!("Field does not match a known pattern");
            }
        }
    }

    passport
}
