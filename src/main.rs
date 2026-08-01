// Cada `mod` conecta este archivo con otro archivo dentro de `src`.
// Por ejemplo, `mod framebuffer;` hace disponible el codigo de
// `src/framebuffer.rs` como el modulo `framebuffer`.
mod framebuffer;
mod line;
mod patterns;

// `use` trae nombres de otros modulos al alcance actual para no escribir
// rutas largas como `framebuffer::Framebuffer` en cada lugar.
use framebuffer::Framebuffer;
use minifb::{Key, Scale, Window, WindowOptions};

// `usize` es el tipo entero que Rust usa normalmente para indices y tamanos.
// Aqui la simulacion corre en una grilla pequena, y minifb la escala visualmente.
const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 110;

// `&Framebuffer` significa "prestame una referencia de solo lectura".
// `&mut Framebuffer` significa "prestame una referencia modificable".
// Asi evitamos copiar todo el framebuffer en cada frame.
fn render(current: &Framebuffer, next: &mut Framebuffer) {
    line::step(current, next);
}

// En Windows, algunas pantallas con escalado de DPI pueden cambiar el tamano
// real de la ventana. Esta llamada intenta que cada pixel del framebuffer se
// mantenga consistente con la escala que le pedimos a minifb.
#[cfg(windows)]
fn disable_dpi_scaling() {
    // Esta funcion viene de la API de Windows y Rust no puede verificar su
    // seguridad automaticamente, por eso debe llamarse dentro de `unsafe`.
    unsafe {
        winapi::um::winuser::SetProcessDPIAware();
    }
}

// Si el programa se compila en otro sistema operativo, esta version vacia evita
// que tengamos que cambiar `main`: se llama igual, pero no hace nada.
#[cfg(not(windows))]
fn disable_dpi_scaling() {}

fn main() {
    disable_dpi_scaling();

    // Usamos dos framebuffers: uno tiene el estado actual y el otro se llena con
    // el siguiente turno. Esto evita que una celula ya actualizada afecte el
    // calculo de sus vecinos en el mismo frame.
    let mut current = Framebuffer::new(GRID_WIDTH, GRID_HEIGHT);
    let mut next = Framebuffer::new(GRID_WIDTH, GRID_HEIGHT);

    // El patron inicial se dibuja una sola vez. Despues, las reglas de Conway
    // se encargan de transformar la pantalla frame por frame.
    patterns::load_initial_pattern(&mut current);

    // minifb crea una ventana simple y recibe directamente nuestro arreglo de
    // colores. `Scale::X8` hace que cada celula se vea como un bloque de 8x8
    // pixeles en pantalla, aunque internamente siga siendo una grilla pequena.
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

    // Limitamos la velocidad para que la animacion sea visible y no cambie tan
    // rapido que parezca ruido.
    window.set_target_fps(10);

    // El ciclo principal corre mientras la ventana siga abierta y no se presione
    // Escape. Cada vuelta equivale a un turno del juego.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(&current, &mut next);

        // Intercambiamos los buffers en vez de copiar todos sus pixeles. Despues
        // del swap, `current` contiene el nuevo estado y `next` queda listo para
        // reutilizarse en el siguiente frame.
        std::mem::swap(&mut current, &mut next);

        // minifb espera un slice con width * height colores. Nuestro framebuffer
        // ya guarda los colores en ese formato lineal.
        window
            .update_with_buffer(&current.buffer, GRID_WIDTH, GRID_HEIGHT)
            .unwrap();
    }
}
