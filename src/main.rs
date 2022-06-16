use aalto2lib::{
    reset_grid,
    choose_collapsable,
    collapse,
    render,
    rules
};

fn main() {
    let h = 48;
    let w = ((h as f64)*1.75) as usize;
    let mut grid = reset_grid(w, h, rules::terrain());

    let mut tile = choose_collapsable(&grid);
    while tile.is_some() {
        let t = tile.unwrap();
        collapse(&mut grid, t.0, t.1, w*h);
        tile = choose_collapsable(&grid);
    }
    
    println!("{}", render(&grid));
}
