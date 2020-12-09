use regex::Regex;
use std:: {collections::HashMap, error::Error, fs};

fn main() {
    // import bag_rules
    let bag_rules = import_bag_rules("input.txt");

    // get target_bag_count
    let count = count_containing_bags(bag_rules.unwrap(), "shiny gold");

    println!("Bag Ancestor Count: {}", count);
}

fn import_bag_rules(input_text: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(input_text)?;
    Ok(input)
}

fn count_containing_bags(bag_rules: String, target_bag: &str) -> usize {
    // compile rules into searcheable form
    let rules = parse_bag_rules(bag_rules);

    // search rules
    let mut containing_bags: Vec<String> = Vec::new();

    for line in rules.iter().clone(){
        if !containing_bags.contains(line.0){
            search_bag_line(target_bag, &rules, line.0, &mut containing_bags);
        }
    }

    containing_bags.iter().count()
}

fn search_bag_line(
    target_bag: &str, 
    rules: &HashMap<String, Option<Vec<(String, usize)>>>,
    rule: &String, 
    containing_bags: &mut Vec<String>){
    
        

}

// revisit data structure. Pointer to both parents and children?
fn parse_bag_rules(bag_rules: String) -> HashMap<String, Option<Vec<(String, usize)>>> {
    let mut map: HashMap<String, Option<Vec<(String, usize)>>> = HashMap::new();

    // pass in regex to avoid creating in loop
    let children_regex = Regex::new(r"(\d) ([a-z ]* )").unwrap();
    for line in bag_rules.lines() {
        // Split line into children, parent
        let parent_children: Vec<&str> = line.split(" bags contain ").collect();
        let children = get_children(&children_regex, parent_children[1]);

        map.insert(parent_children[0].to_string(), children);
    }

    map
}

fn get_children<'a>(regex: &'a Regex, children_str: &'a str) -> Option<Vec<(String, usize)>> {
    let mut children: Vec<(String, usize)> = Vec::new();

    for cap in regex.captures_iter(children_str) {
        let count: usize = cap[1].parse().unwrap();
        children.push((cap[2].trim().to_string(), count));
    }

    if children.is_empty() {
        Option::None
    } else {
        Some(children)
    }
}

#[test]
fn finds_gold_bag(){
    let bag_rules = "light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.".to_string();

    let target_bag = "shiny gold";

    assert_eq!(4, count_containing_bags(bag_rules, target_bag));
}

#[test]
fn gets_children_some() {
}

#[test]
fn gets_children_none() {

}