mod framebuffer;
mod patterns;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use patterns::*;

fn conway_step(fb: &mut Framebuffer) {
    // Crear una copia del framebuffer actual para calcular el siguiente estado
    let mut next_fb = Framebuffer::new(fb.width, fb.height, Color::BLACK);
    
    for x in 0..fb.width {
        for y in 0..fb.height {
            let mut live_neighbors = 0;
            
            // Contar vecinos vivos (8 direcciones)
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    
                    let nx = x + dx;
                    let ny = y + dy;
                    
                    // Verificar límites y contar si está vivo
                    if nx >= 0 && nx < fb.width && ny >= 0 && ny < fb.height {
                        let neighbor_color = fb.get_color(nx, ny);
                        if neighbor_color == Color::WHITE {
                            live_neighbors += 1;
                        }
                    }
                }
            }
            
            let current_color = fb.get_color(x, y);
            let is_alive = current_color == Color::WHITE;
            
            // Aplicar las reglas de Conway
            let next_alive = match (is_alive, live_neighbors) {
                (true, 2) | (true, 3) => true,  // Célula viva con 2 o 3 vecinos sobrevive
                (false, 3) => true,              // Célula muerta con exactamente 3 vecinos nace
                _ => false,                      // En todos los otros casos, muere o permanece muerta
            };
            
            if next_alive {
                next_fb.set_current_color(Color::WHITE);
                next_fb.set_pixel(x, y);
            }
        }
    }
    
    // Reemplazar el buffer actual con el nuevo estado
    fb.color_buffer = next_fb.color_buffer;
}

fn render(fb: &mut Framebuffer) {
    conway_step(fb);
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 120;
    let framebuffer_height = 90;
    let background_color = Color::BLACK;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .build();

    let mut fb = Framebuffer::new(framebuffer_width, framebuffer_height, background_color);
    
    // Crear patrón inicial creativo con múltiples organismos
    
    add_gosper_glider_gun(&mut fb, 5, 5);
    add_glider(&mut fb, 5, 5);
    add_glider(&mut fb, 50, 10);
    add_glider(&mut fb, 100, 5);

    add_block(&mut fb, 20, 15);
    add_beehive(&mut fb, 35, 15);
    add_loaf(&mut fb, 55, 15);
    add_tub(&mut fb, 75, 15);
    add_block(&mut fb, 95, 15);

    add_glider(&mut fb, 100, 5);          
    add_beehive(&mut fb, 95, 15);         
    add_block(&mut fb, 110, 10);          
    add_tub(&mut fb, 100, 20); 

    add_blinker(&mut fb, 95, 40);         // oscilador pequeño
    add_toad(&mut fb, 105, 45);           // oscilador mediano
    add_loaf(&mut fb, 100, 50);           // patrón quieto
    add_r_pentomino(&mut fb, 95, 55); 

    add_r_pentomino(&mut fb, 10, 25);
    add_diehard(&mut fb, 80, 25);

    add_blinker(&mut fb, 15, 35);
    add_toad(&mut fb, 30, 35);
    add_beacon(&mut fb, 45, 35);
    add_blinker(&mut fb, 65, 35);
    add_blinker(&mut fb, 85, 35);

    add_pulsar(&mut fb, 20, 45);

    add_acorn(&mut fb, 10, 50);
    add_pentadecathlon(&mut fb, 50, 65);
    add_lwss(&mut fb, 70, 70);

    add_beehive(&mut fb, 5, 75);
    add_block(&mut fb, 25, 75);
    add_glider(&mut fb, 45, 75);
    add_blinker(&mut fb, 10, 78);
    add_tub(&mut fb, 5, 85);

    add_glider(&mut fb, 100, 65);           
    add_blinker(&mut fb, 95, 75);
    add_beehive(&mut fb, 85, 78);
    add_loaf(&mut fb, 105, 75);
    add_lwss(&mut fb, 90, 60);
    add_toad(&mut fb, 90, 80);
    add_block(&mut fb, 110, 80);
    add_pentadecathlon(&mut fb, 80, 55);
    add_glider(&mut fb, 100, 70);

    // Configurar velocidad de animación
    rl.set_target_fps(8); 
    
    let mut generation = 0;
    
    while !rl.window_should_close() {
        if generation > 0 { // No hacer step en el primer frame para ver el patrón inicial
            render(&mut fb);
        }
        generation += 1;
        
        // Dibujar en pantalla
        fb.swap_buffers(&mut rl, &thread);
        
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}