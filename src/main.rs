mod framebuffer;
mod line;
mod patterns;

use framebuffer::Framebuffer;
use minifb::{Key, Scale, Window, WindowOptions};

const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 110;

fn render(current: &Framebuffer, next: &mut Framebuffer) {
    line::step(current, next);
}

#[cfg(windows)]
fn disable_dpi_scaling() {
    unsafe {
        winapi::um::winuser::SetProcessDPIAware();
    }
}

#[cfg(not(windows))]
fn disable_dpi_scaling() {}

fn main() {
    disable_dpi_scaling();

    let mut current = Framebuffer::new(GRID_WIDTH, GRID_HEIGHT);
    let mut next = Framebuffer::new(GRID_WIDTH, GRID_HEIGHT);

    patterns::load_initial_pattern(&mut current);

    let mut window = Window::new(
        "Conway's Game of Life",
        GRID_WIDTH,
        GRID_HEIGHT,
        WindowOptions {
            scale: Scale::X8,
            ..WindowOptions::default()
        },
    )
    .expect("No se pudo crear la ventana");

    window.set_target_fps(10);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(&current, &mut next);
        std::mem::swap(&mut current, &mut next);

        window
            .update_with_buffer(&current.buffer, GRID_WIDTH, GRID_HEIGHT)
            .unwrap();
    }
}
