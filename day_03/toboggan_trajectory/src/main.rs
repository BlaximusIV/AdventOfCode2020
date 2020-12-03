use std::fs;
use std::error::Error;

// Note: The toboggan only travels in a single direction
// So x, y are both unsigned
struct Trajectory {
    vert_travel: usize,
    horiz_travel: usize
}

fn main() {
    let input_text = "input.txt";
    let toboggan_trajectory = Trajectory { vert_travel:1, horiz_travel:3 };

    let snow_field = survey_snow_field(input_text);
    let collision_count = plot_course_collisions(toboggan_trajectory, snow_field.unwrap());

    println!("{} tree collisions on the plotted course.", collision_count);
}

/// import the input text that represents the field to travel through
fn survey_snow_field(input_text: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut snow_field: Vec<Vec<char>> = Vec::new();
    
    let input = fs::read_to_string(input_text).unwrap();

    for line in input.lines() {
        let mut layer: Vec<char> = Vec::new();
        for c in line.chars(){
            layer.push(c);
        }
        snow_field.push(layer);
    }

    Ok(snow_field)
}

fn plot_course_collisions(trajectory: Trajectory, field: Vec<Vec<char>>) -> usize {
    let tree_character = '#';
    let field_width = field[0].len();

    let mut y_pos = 0;
    let mut x_pos = 0;

    let mut collision_count: usize = 0;
    loop {

        advance_course(&mut y_pos, &mut x_pos, &field_width, &trajectory);

        if y_pos == field.len(){
            break;
        }

        let pos_char: char = field[y_pos][x_pos];
        if pos_char == tree_character {
            collision_count += 1;
        }
    }

    collision_count
}

fn advance_course(
    y_pos: &mut usize, 
    x_pos: &mut usize,
    field_width: &usize,
    trajectory: &Trajectory) {
        *y_pos += trajectory.vert_travel;

        if *x_pos + trajectory.horiz_travel <= field_width - 1 {
            *x_pos += trajectory.horiz_travel;
        } else {
            // need to wrap around, the grid repeats horizontally
            *x_pos = (*x_pos + trajectory.horiz_travel) % field_width;
        }
}