use std::{error::Error, fs};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BoardingPass {
    ticket: String,
    row: usize,
    column: usize,
    seat_id: usize,
}

impl BoardingPass {
    fn new(ticket: String, row: usize, column: usize ) -> Self {
        Self{
            ticket,
            row,
            column,
            seat_id: (row * 8) + column,
        }
    }
}

/// Today's exercise is for finding seats given a ticket
fn main() {
    let tickets = import_tickets("input.txt");
    let (passes, max_id) = get_boarding_passes(tickets.unwrap());

    let missing_id = find_missing_seat(passes);
    
    println!("Missing ID: {}", missing_id);
    println!("Max ID: {}", max_id);
}

fn import_tickets(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let input = fs::read_to_string(input)?;

    let mut ticket_lines: Vec<String> = Vec::new();

    for line in input.lines() {
        ticket_lines.push(line.to_string());
    }

    Ok(ticket_lines)
}

fn get_boarding_passes(tickets: Vec<String>) -> (Vec<BoardingPass>, usize){
    let mut passes: Vec<BoardingPass> = Vec::new();
    let mut max_id: usize = 0;

    for ticket in tickets {
        let pass = BoardingPass::new(
        ticket.to_string(),
            find_position(ticket[..7].to_string(), 'F', 127),
            find_position(ticket[7..].to_string(),'L', 7)
        );

        if pass.seat_id > max_id {
            max_id = pass.seat_id
        }

        passes.push(pass);
    }
    
    (passes, max_id)
}

fn find_position(direction: String, lower_char: char, max_range: usize) -> usize {
    let translations = &direction[..direction.len() - 1];
    let final_char = &direction[direction.len() -1..];

    let mut range = (0, max_range);
    for c in translations.chars(){
        halve_range(&mut range, c == lower_char);
    }

    if final_char.contains(lower_char) {
        range.0
    } else {
        range.1
    }
}

fn halve_range(range: &mut (usize, usize), take_lower: bool) {
    let change = ((range.1 - range.0) / 2) + 1;

    if take_lower {
        range.1 -= change;
    } else {
        range.0 += change;
    }
}

fn find_missing_seat(mut passes: Vec<BoardingPass>) -> usize {
    &passes.sort_by(|a, b| a.seat_id.cmp(&b.seat_id));

    let mut missing_id = passes[0].seat_id;

    for pass in passes {
        if missing_id != pass.seat_id {
            return missing_id;
        }

        missing_id += 1;
    }

    panic!("No missing id found");
}

#[test]
fn get_correct_passes() {
    let passes = get_boarding_passes(vec![
        "BFFFBBFRRR".to_string(), 
        "FFFBBBFRRR".to_string(),
        "BBFFBBFRLL".to_string(),
        "FBFBBFFRLR".to_string(),
    ]);

    let passes = passes.0;

    assert_eq!(passes[0].row, 70);
    assert_eq!(passes[0].column, 7);
    assert_eq!(passes[0].seat_id, 567);

    assert_eq!(passes[1].row, 14);
    assert_eq!(passes[1].column, 7);
    assert_eq!(passes[1].seat_id, 119);

    assert_eq!(passes[2].row, 102);
    assert_eq!(passes[2].column, 4);
    assert_eq!(passes[2].seat_id, 820);

    assert_eq!(passes[3].row, 44);
    assert_eq!(passes[3].column, 5);
    assert_eq!(passes[3].seat_id, 357);
}