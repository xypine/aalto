use rand_chacha::ChaCha8Rng;
use rand::{seq::SliceRandom, SeedableRng};

pub mod value;
pub mod tile;
pub mod rules;
pub mod features;
#[cfg(feature = "wasm")]
pub mod wasm;

use tile::Tile;

pub type Grid = Vec<Vec<Tile>>;

pub fn reset_grid(width: usize, height: usize, default_possible: Vec<value::Value>) -> Grid {
    let mut grid: Grid = vec![];
    for _y in 0..height {
        let mut col = vec![];
        for _x in 0..width {
            let tile = Tile {
                possible: default_possible.clone()
            };
            col.push(tile);
        }
        grid.push(col);
    }
    grid
}

pub fn propagate(grid: &mut Grid, x: usize, y: usize, max_recursions: usize) {
    if max_recursions < 1 {
        return;
    }
    let tile = grid[y][x].clone();
    let mut neighbours: Vec<((usize, usize), usize, usize)> = vec![];
    
    if y > 0 {
        neighbours.push(
            ((x, y-1), 0, 2)
        );
    }
    if x > 0 {
        neighbours.push(
            ((x-1, y), 1, 3)
        );
    }
    if y < grid.len() - 1 {
        neighbours.push(
            ((x, y+1), 2, 0)
        );
    }
    if x < grid[y].len() - 1 {
        neighbours.push(
            ((x+1, y), 3, 1)
        );
    }

    for (neighbour_xy, neighbour_direction, neighbour_reverse_direction) in neighbours {
        let mut neighbour = &mut grid[neighbour_xy.1][neighbour_xy.0];

        if neighbour.possible.len() > 1 {
            let old_possible = neighbour.possible.clone();
            let mut new_possible = vec![];
            for possible in &tile.possible {
                let connector = &possible.connectors[neighbour_direction];
                let disallowed: Option<Vec<String>>;
                if possible.disallow.is_some() {
                    let tmp = possible.disallow.clone().unwrap()[neighbour_direction].clone();
                    disallowed = Some(tmp);
                }
                else {
                    disallowed = None;
                }
                for neighbour_possible in &neighbour.possible {
                    let matching_connector = &neighbour_possible.connectors[neighbour_reverse_direction];
                    if connector.iter().find(|c| matching_connector.contains(*c)).is_some() { // Check that atleast any connectors match
                        let allowed: bool;
                        if disallowed.is_some() {
                            allowed = disallowed.clone().unwrap().iter().find(|c| matching_connector.contains(*c)).is_none();
                        }
                        else {
                            allowed = true;
                        }
                        if allowed {
                            if !new_possible.contains(neighbour_possible) {
                                new_possible.push( neighbour_possible.clone() );
                            }
                        }
                    }
                }
            }

            // Only propagate if possible states have changed, don't use != as it takes order into account
            // Also don't propagate if this tile has no possible values
            if !old_possible.iter().all( |p| new_possible.contains(p) ) && new_possible.len() > 0 {
                neighbour.possible = new_possible;
                propagate(grid, neighbour_xy.0, neighbour_xy.1, max_recursions-1);
            }
        }
    }
}

/// If rng_seed is none, rng thread is used instead
pub fn collapse(grid: &mut Grid, x: usize, y: usize, rng_seed: Option<u64>, max_recursions: usize) {

    let mut current_tile = &mut grid[y][x];
    if current_tile.possible.len() > 1 {
        current_tile.possible = vec![
            match rng_seed {
                Some( seed ) => current_tile.possible.choose(&mut ChaCha8Rng::seed_from_u64(seed)).unwrap().clone(),
                None => current_tile.possible.choose(&mut rand::thread_rng()).unwrap().clone(),
            }
        ];
        propagate(grid, x, y, max_recursions);
    }
}

/// If rng_seed is none, rng thread is used instead
pub fn choose_collapsable(grid: &Grid, rng_seed: Option<u64>) -> Option<(usize, usize)> {
    let mut matching_tiles: Vec<(usize, usize)> = vec![];
    let mut min_entropy = usize::MAX;

    let mut y: usize = 0;
    for col in grid {
        let mut x: usize = 0;
        for t in col {
            let entropy = t.possible.len();
            if entropy < min_entropy && entropy > 1 {
                min_entropy = entropy;
                matching_tiles = vec![ (x, y) ];
            }
            else if entropy == min_entropy {
                matching_tiles.push((x, y));
            }
            x += 1;
        }
        y += 1;
    }

    if matching_tiles.len() > 0 {
        let chosen = match rng_seed {
            Some( seed ) => matching_tiles.choose(&mut ChaCha8Rng::seed_from_u64(seed)).unwrap().clone(),
            None => matching_tiles.choose(&mut rand::thread_rng()).unwrap().clone(),
        };
        return Some(chosen);
    }

    None
}

/// If rng_seed is none, rng thread is used instead
pub fn collapse_all(grid: &mut Grid, rng_seed: Option<u64>, max_recursions: usize) {
    let mut tile = choose_collapsable(&grid, rng_seed);
    while tile.is_some() {
        let t = tile.unwrap();
        collapse(grid, t.0, t.1, rng_seed, max_recursions);
        tile = choose_collapsable(&grid, rng_seed);
    }
}

pub fn is_healthy(grid: &Grid) -> bool {
    for col in grid {
        for t in col {
            if t.possible.len() < 1 {
                return false;
            }
        }
    }

    true
}

pub fn render(grid: &Grid) -> String {
    let mut out = String::from("");
    for col in grid {
        for t in col {
            if t.possible.len() == 1 {
                out = format!("{}{}", out, t.possible[0].value);
            }
            else {
                out = format!("{}{}", out, t.possible.len());
            }
        }
        out = format!("{}\n", out);
    }
    out
}
