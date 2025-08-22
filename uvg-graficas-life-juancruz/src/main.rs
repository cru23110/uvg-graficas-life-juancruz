mod framebuffer;
mod patterns;

use framebuffer::Framebuffer;
use patterns::*;
use rand::{rng, Rng};
use raylib::prelude::*;
use std::{thread, time::Duration};

const GRID_W: usize = 100;   
const GRID_H: usize = 100;
const WIN_W: i32 = 800;      
const WIN_H: i32 = 800;

fn wrap(v: isize, max: isize) -> isize {
    (v % max + max) % max
}

fn idx(x: isize, y: isize) -> usize {
    (y as usize) * GRID_W + (x as usize)
}

fn neighbors_toroidal(grid: &[bool], x: isize, y: isize) -> u8 {
    let mut c = 0u8;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = wrap(x + dx, GRID_W as isize);
            let ny = wrap(y + dy, GRID_H as isize);
            if grid[idx(nx, ny)] { c += 1; }
        }
    }
    c
}

fn step(next: &mut [bool], cur: &[bool]) {
    for y in 0..GRID_H as isize {
        for x in 0..GRID_W as isize {
            let alive = cur[idx(x,y)];
            let n = neighbors_toroidal(cur, x, y);
            let new_alive = match (alive, n) {
                (true, 2) | (true, 3) => true,   
                (true, _)             => false,  
                (false, 3)            => true,   
                (false, _)            => false,
            };
            next[idx(x,y)] = new_alive;
        }
    }
}

fn draw_grid(fb: &mut Framebuffer, grid: &[bool]) {
    for y in 0..GRID_H as isize {
        for x in 0..GRID_W as isize {
            if grid[idx(x,y)] {
                fb.set_current_color(Color::GOLD);
            } else {
                fb.set_current_color(fb.background_color);
            }
            fb.set_pixel(x as i32, y as i32);
        }
    }
}

fn seed_initial_pattern(grid: &mut [bool]) {
    grid.fill(false);

    glider(grid, GRID_W, 5, 5);
    glider(grid, GRID_W, 12, 8);
    blinker(grid, GRID_W, 20, 20);
    toad(grid, GRID_W, 30, 30);
    beacon(grid, GRID_W, 42, 42);
    pulsar(grid, GRID_W, 60, 10);
    lwss(grid, GRID_W, 70, 70);
    block(grid, GRID_W, 10, 60);
    boat(grid, GRID_W, 25, 75);
    loaf(grid, GRID_W, 80, 40);

    let mut rng = rng();
    for _ in 0..600 {   
        let x = rng.random_range(0..GRID_W) as isize;
        let y = rng.random_range(0..GRID_H) as isize;
        grid[idx(x,y)] = true;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_W, WIN_H)
        .title("UVG - Game of Life (Juan Cruz)")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut fb = Framebuffer::new(GRID_W as u32, GRID_H as u32);
    fb.set_background_color(Color::new(36, 18, 32, 255));

    let mut cur = vec![false; GRID_W * GRID_H];
    let mut next = vec![false; GRID_W * GRID_H];
    seed_initial_pattern(&mut cur);

    let mut paused = false;
    let mut frame_ms: u64 = 60; 

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            seed_initial_pattern(&mut cur);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            cur.fill(false);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            fb.save("screenshot.png");
            println!("Screenshot guardado en screenshot.png");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            frame_ms = frame_ms.saturating_sub(5).max(5);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            frame_ms = (frame_ms + 5).min(200);
        }

        if !paused {
            step(&mut next, &cur);
            std::mem::swap(&mut cur, &mut next);
        }

        draw_grid(&mut fb, &cur);

        fb.swap_buffers(&mut rl, &thread);

        thread::sleep(Duration::from_millis(frame_ms));
    }
}