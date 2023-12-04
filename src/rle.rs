// https://conwaylife.com/wiki/Run_Length_Encoded
/*
b	dead cell
o	alive cell
$	end of line
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::coord::Coord;
use crate::state::State;

pub fn load_pattern(name: &str, state: &mut State, x_offset: i32, y_offset: i32) -> Result<(), String> {
  let file = File::open(name).expect("could not open file");
  let reader = BufReader::new(file);
  let mut x: i32 = x_offset;
  let mut y: i32 = y_offset;
  let mut run_count_str: String = "0".to_owned();
  
  println!("loading pattern: {}", name);
  for l in reader.lines() {
    let l = l.unwrap();
    for c in l.chars() {
      match c {
        '#' | 'x' => {
          break;
        },
        '0'..='9' => {
          run_count_str.push(c);
        },
        'o' => {
          let run_count = if run_count_str == "0" { 1 } else { run_count_str.parse::<i32>().unwrap() };
          for _ in 0..run_count {
            state.cell_coords.insert(Coord::new(x, y));
            x += 1;
          }
          run_count_str = "0".to_owned();
        },
        'b' => {
          let run_count = if run_count_str == "0" { 1 } else { run_count_str.parse::<i32>().unwrap() };
          x += run_count;
          run_count_str = "0".to_owned();
        },
        '$' => {
          let run_count = if run_count_str == "0" { 1 } else { run_count_str.parse::<i32>().unwrap() };
          y += run_count;
          x = x_offset;
          run_count_str = "0".to_owned();
        },
        '!' => {
          break;
        },
        _ => {}
      }
    }
  };

  Ok(())
}
