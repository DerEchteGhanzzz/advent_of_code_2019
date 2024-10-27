use std::{collections::HashMap, f32::consts::PI};

use itertools::Itertools;

pub fn solve_a(input: Vec<String>) -> usize {
    let map = to_asteroid_map(input);
    let vsbl_map = get_asteroid_visibility_map(&map);
    vsbl_map.iter().fold(0, |acc, ((_, _), count)| if count > &acc { *count } else { acc })
}

pub fn solve_b(input: Vec<String>) -> i32 {
    let map = to_asteroid_map(input);
    let (x, y) = get_asteroid_visibility_map(&map).iter().fold(((0 as usize, 0  as usize), 0 as usize), |acc, ((x, y), count)| if count > &acc.1 { ((*x, *y), *count) } else { acc }).0;
    let mut asteroid_map = ray_trace((x as i32, y as i32), &map);
    let mut sorted_map = asteroid_map.iter().map(|((i, j), _)| (*i, *j)).sorted_by(|(x1, y1), (x2, y2)| Ord::cmp(&((angle_with_y_axis((*x1, *y1)) * 100.0) as i32), &((angle_with_y_axis((*x2, *y2)) * 100.0) as i32))).collect::<Vec<_>>();
    let mut count = 0;
    let mut i = 0;
    loop {
        let idx = i % sorted_map.len();
        let (dx, dy) = sorted_map[idx];
        let mut scalar_vec = asteroid_map.remove(&(dx, dy)).unwrap();
        let scalar = scalar_vec.remove(0);
        if scalar_vec.len() > 0 {
            asteroid_map.insert((dx, dy), scalar_vec);
            i += 1;
        } else {
            sorted_map.remove(idx);
        }
        count += 1;
        if count == 200 {
            println!("({} * {} + {}) * 100, {} * {} + {}", dx, scalar, x, dy, scalar, y);
            return ((dx*scalar) + x as i32)*100 + ((dy*scalar) + y as i32)
        }
    }
}

fn to_asteroid_map(input: Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|s| s.chars().collect()).collect()
}

fn get_asteroid_visibility_map(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), usize> {
    let mut visbl_map: HashMap<(usize, usize), usize> = HashMap::new();
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] != '#' {
                continue;
            }
            let sights = ray_trace((x as i32, y as i32), &map).len();
            visbl_map.insert((x, y), sights);
        }
    }
    visbl_map
}

fn ray_trace((x, y): (i32, i32), map: &Vec<Vec<char>>) -> HashMap<(i32, i32), Vec<i32>> {
    let mut visibility_map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut current_radius: i32 = 1;
    loop {

        let top: Vec<(i32, i32)> = (-current_radius..=current_radius).map(|i| (i, current_radius)).collect();
        let right: Vec<(i32, i32)> = (-current_radius+1..current_radius).map(|j| (current_radius, j)).collect();
        let left: Vec<(i32, i32)> = (-current_radius+1..current_radius).map(|j| (-current_radius, j)).collect();
        let bot: Vec<(i32, i32)> = (-current_radius..=current_radius).map(|i| (i, -current_radius)).collect();

        for (i, j) in vec![top, left, right, bot].concat(){
            if x+i < 0 || (x+i) as usize >= map[0].len() || y+j < 0 || (y+j) as usize >= map.len() {
                continue;
            }

            if map[(y+j) as usize][(x+i) as usize] == '.' {
                continue;
            }
            let (a, b) = match (i, j) {
                (0, _) => (0, j.signum()),
                (_, 0) => (i.signum(), 0),
                _ => simplest_frac(i, j),
            };
            match visibility_map.get_mut(&(a, b)) {
                None => {visibility_map.insert((a, b), vec![1]);},
                Some(v) => v.push(gcd::euclid_u32(i.abs() as u32, j.abs() as u32) as i32),
            }
        }
        current_radius += 1;
        if current_radius as usize > map.len() && current_radius as usize > map[0].len() {
            break;
        }
    }

    visibility_map
}

fn simplest_frac(a: i32, b: i32) -> (i32, i32) {
    let div = gcd::euclid_u32(a.abs() as u32, b.abs() as u32) as i32;
    ((a / div) as i32, (b / div) as i32)
}

fn angle_with_y_axis((x, y): (i32, i32)) -> f32 {
    // θ = cos-1 [ (a . b) / (|a| |b|) ]
    // y-axis: (0, 1)

    if x == 0 {
        return if y <= 0 { return 0.0; } else { 180.0 }
    } 
    if y == 0 {
        return if x >= 0 { return 90.0; } else { 270.0 }
    }
    let angle = (x as f32).atan2(-y as f32)*180.0/PI;
    if angle >= 0.0 { angle } else { 360.0 + angle }
}