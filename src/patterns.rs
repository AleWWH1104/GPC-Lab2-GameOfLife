use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

pub fn add_glider(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [
        (1, 0), (2, 1), (0, 2), (1, 2), (2, 2)
    ];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_blinker(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(0, 1), (1, 1), (2, 1)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_block(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(0, 0), (1, 0), (0, 1), (1, 1)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_toad(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_beacon(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_beehive(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_loaf(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_tub(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(1, 0), (0, 1), (2, 1), (1, 2)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_lwss(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [
        (1, 0), (4, 0), (0, 1), (0, 2), (4, 2), (0, 3), (1, 3), (2, 3), (3, 3)
    ];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_pulsar(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [
        // Top part
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        // Bottom part
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_pentadecathlon(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [
        (1, 0), (1, 1), (0, 2), (2, 2), (1, 3), (1, 4), (1, 5), (1, 6),
        (0, 7), (2, 7), (1, 8), (1, 9)
    ];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_gosper_glider_gun(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [
        // Left square
        (0, 4), (0, 5), (1, 4), (1, 5),
        // Left group
        (10, 4), (10, 5), (10, 6), (11, 3), (11, 7), (12, 2), (12, 8),
        (13, 2), (13, 8), (14, 5), (15, 3), (15, 7), (16, 4), (16, 5), (16, 6), (17, 5),
        // Right group
        (20, 2), (20, 3), (20, 4), (21, 2), (21, 3), (21, 4), (22, 1), (22, 5),
        (24, 0), (24, 1), (24, 5), (24, 6),
        // Right square
        (34, 2), (34, 3), (35, 2), (35, 3)
    ];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_r_pentomino(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_diehard(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(6, 0), (0, 1), (1, 1), (1, 2), (5, 2), (6, 2), (7, 2)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}

pub fn add_acorn(fb: &mut Framebuffer, start_x: i32, start_y: i32) {
    let pattern = [(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)];
    
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in pattern {
        fb.set_pixel(start_x + dx, start_y + dy);
    }
}