
pub fn set_cell(grid: &mut [bool], grid_w: usize, x: isize, y: isize, val: bool) {
    if x >= 0 && y >= 0 {
        let (x, y) = (x as usize, y as usize);
        if y * grid_w + x < grid.len() {
            grid[y * grid_w + x] = val;
        }
    }
}

pub fn glider(grid: &mut [bool], w: usize, x: isize, y: isize) {
    let pts = [(1,0),(2,1),(0,2),(1,2),(2,2)];
    for (dx,dy) in pts { set_cell(grid, w, x+dx, y+dy, true); }
}

pub fn blinker(grid: &mut [bool], w: usize, x: isize, y: isize) {
    let pts = [(0,1),(1,1),(2,1)];
    for (dx,dy) in pts { set_cell(grid, w, x+dx, y+dy, true); }
}

pub fn toad(grid: &mut [bool], w: usize, x: isize, y: isize) {
    let pts = [(1,0),(2,0),(3,0),(0,1),(1,1),(2,1)];
    for (dx,dy) in pts { set_cell(grid, w, x+dx, y+dy, true); }
}

pub fn beacon(grid: &mut [bool], w: usize, x: isize, y: isize) {
    let pts = [(0,0),(1,0),(0,1),(1,1),(2,2),(3,2),(2,3),(3,3)];
    for (dx,dy) in pts { set_cell(grid, w, x+dx, y+dy, true); }
}

pub fn pulsar(grid: &mut [bool], w: usize, x: isize, y: isize) {
    let pts = [
        (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
        (0,2),(5,2),(7,2),(12,2),
        (0,3),(5,3),(7,3),(12,3),
        (0,4),(5,4),(7,4),(12,4),
        (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),

        (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
        (0,8),(5,8),(7,8),(12,8),
        (0,9),(5,9),(7,9),(12,9),
        (0,10),(5,10),(7,10),(12,10),
        (2,12),(3,12),(4,12),(8,12),(9,12),(10,12),
    ];
    for (dx,dy) in pts { set_cell(grid, w, x+dx, y+dy, true); }
}

pub fn lwss(grid: &mut [bool], w: usize, x: isize, y: isize) {
    let pts = [
        (1,0),(4,0),
        (0,1),(4,1),
        (4,2),
        (0,3),(3,3),
        (1,4),(2,4),
    ];
    for (dx,dy) in pts { set_cell(grid, w, x+dx, y+dy, true); }
}

pub fn block(grid: &mut [bool], w: usize, x: isize, y: isize) {
    let pts = [(0,0),(1,0),(0,1),(1,1)];
    for (dx,dy) in pts { set_cell(grid, w, x+dx, y+dy, true); }
}

pub fn boat(grid: &mut [bool], w: usize, x: isize, y: isize) {
    let pts = [(0,0),(1,0),(0,1),(2,1),(1,2)];
    for (dx,dy) in pts { set_cell(grid, w, x+dx, y+dy, true); }
}

pub fn loaf(grid: &mut [bool], w: usize, x: isize, y: isize) {
    let pts = [(1,0),(2,0),(0,1),(3,1),(1,2),(3,2),(2,3)];
    for (dx,dy) in pts { set_cell(grid, w, x+dx, y+dy, true); }
}