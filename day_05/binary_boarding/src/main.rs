use std::{error::Error, fs};

struct BoardingPass {
    row: usize,
    column: usize,
    seat_id: usize,
}

impl BoardingPass {
    fn new(row: usize, column: usize ) -> Self {
        Self{
            row,
            column,
            seat_id: (row * 8) + column,
        }
    }
}

/// Today's exercise is for finding seats given a ticket
fn main() {
    let tickets = import_tickets("input.txt");

    let (_passes, max_id) = get_boarding_passes(tickets.unwrap());

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
            get_position(&ticket[..7],0, 127, 'F'),
            get_position(&ticket[7..], 0, 7, 'L')
        );

        if pass.seat_id > max_id {
            max_id = pass.seat_id
        }

        println!("row: {}; column: {}; sid: {}", pass.row, pass.column, pass.seat_id);
        passes.push(pass);
    }
    
    (passes, max_id)
}

fn get_position(directions: &str, range_min: usize, range_max: usize, direction_char: char) -> usize {
    let mut row_range: (usize,usize) = (range_min, range_max);

    for i in 0..directions.chars().count(){
        let c = directions.chars().nth(i).unwrap();

        if i == directions.len() - 1{
            if c == direction_char {
                return row_range.0;
            } else {
                return row_range.1;
            };
        } else {
            halve_range(&mut row_range, c == direction_char);
        }
    }
     0
}

fn halve_range(range: &mut (usize, usize), take_lower: bool) {
    if range.0 == range.1 {
        return;
    }

    let mid = range.0 + (range.1 - range.0) / 2;

    if take_lower {
        range.1 = mid - 1;
    } else {
        range.0 = mid + 1;
    }
}
