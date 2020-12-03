use std::fs;
use std::error::Error;

// Note: The toboggan only travels in a single direction
// So x, y are both unsigned
struct Trajectory {
    vert_travel: usize,
    horiz_travel: usize
}

// This exercise is to count how many times a toboggan would hit trees with the given trajectories and terrain
fn main() {
    let input_text = "input.txt";
    let snow_field = survey_snow_field(input_text);
    let snow_field = snow_field.unwrap();
    
    let toboggan_trajectories: Vec<Trajectory> = vec![
        Trajectory { horiz_travel: 1, vert_travel: 1 },
        Trajectory { horiz_travel: 3, vert_travel: 1 },
        Trajectory { horiz_travel: 5, vert_travel: 1 },
        Trajectory { horiz_travel: 7, vert_travel: 1 },
        Trajectory { horiz_travel: 1, vert_travel: 2 },
    ];


    let mut collision_product = 1;
    for trajectory in toboggan_trajectories {
        collision_product *= plot_course_collisions(trajectory, &snow_field);
    }

    println!("The product of all collision counts is {}", collision_product);
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

fn plot_course_collisions(trajectory: Trajectory, field: &Vec<Vec<char>>) -> usize {
    let tree_character = '#';
    let field_width = field[0].len();

    let mut y_pos = 0;
    let mut x_pos = 0;

    let mut collision_count: usize = 0;
    loop {

        advance_course(&mut y_pos, &mut x_pos, &field_width, &trajectory);

        if y_pos >= field.len(){
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