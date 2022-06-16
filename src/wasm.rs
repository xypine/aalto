use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub fn reset_grid(width: usize, height: usize, default_possible: String) -> String {
    let default_possible_parsed = serde_json::from_str(&default_possible).unwrap();
    let result = crate::reset_grid(width, height, default_possible_parsed);
    serde_json::to_string(&result).unwrap()
}

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct WasmGrid {
    tiles: crate::Grid
}

pub fn propagate(grid: String, x: usize, y: usize, max_iterations: usize) -> String {
    let grid_parsed: WasmGrid = serde_json::from_str(&grid).unwrap();
    let mut mgrid = grid_parsed.tiles.clone();
    crate::propagate(&mut mgrid, x, y, max_iterations);
    return serde_json::to_string(&WasmGrid{
        tiles: mgrid
    }).unwrap();
}

pub fn collapse(grid: String, x: usize, y: usize, max_iterations: usize) -> String {
    let grid_parsed: WasmGrid = serde_json::from_str(&grid).unwrap();
    let mut mgrid = grid_parsed.tiles.clone();
    crate::collapse(&mut mgrid, x, y, max_iterations);
    return serde_json::to_string(&WasmGrid{
        tiles: mgrid
    }).unwrap();
}

pub fn choose_collapsable(grid: String) -> String {
    let grid_parsed: WasmGrid = serde_json::from_str(&grid).unwrap();
    let mgrid = grid_parsed.tiles.clone();
    let result = crate::choose_collapsable(&mgrid);
    return serde_json::to_string(&result).unwrap();
}

pub fn render(grid: String) -> String {
    let grid_parsed: WasmGrid = serde_json::from_str(&grid).unwrap();
    let mgrid = grid_parsed.tiles.clone();
    crate::render(&mgrid)
}