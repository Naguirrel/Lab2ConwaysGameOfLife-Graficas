// Importamos solo lo que este archivo necesita del modulo framebuffer.
// `crate::` significa "desde la raiz del proyecto".
use crate::framebuffer::{DEAD, Framebuffer, alive_color, is_alive};

// Cuenta las 8 celulas vecinas alrededor de (x, y).
// Recibe `&Framebuffer` porque solo necesita leer el estado actual.
fn count_neighbors(fb: &Framebuffer, x: usize, y: usize) -> u32 {
    // Convertimos a `i32` porque los desplazamientos de vecinos pueden ser -1.
    // `usize` no permite numeros negativos, asi que seria incomodo para revisar
    // posiciones como x - 1 o y - 1.
    let w = fb.width as i32;
    let h = fb.height as i32;
    let mut count = 0;

    // Estos rangos recorren -1, 0 y 1. Combinados generan las 9 posiciones del
    // cuadrado alrededor de la celula, incluyendo la celula central.
    for dy in -1..=1 {
        for dx in -1..=1 {
            // La posicion (0, 0) representa la celula actual, no un vecino.
            if dx == 0 && dy == 0 {
                continue;
            }

            // Hacemos que los bordes se conecten como si la grilla fuera un
            // toro: salir por la izquierda entra por la derecha, y salir por
            // arriba entra por abajo. El `+ w` y `+ h` evitan modulos negativos.
            let nx = ((x as i32 + dx + w) % w) as usize;
            let ny = ((y as i32 + dy + h) % h) as usize;

            // Como ahora hay varios colores vivos, no comparamos con un unico
            // color. `is_alive` decide si el pixel representa vida.
            if is_alive(fb.get_color(nx, ny)) {
                count += 1;
            }
        }
    }

    count
}

// Calcula un turno completo del Game of Life.
// `current` se lee y `next` se escribe; separarlos es importante para que todas
// las celulas se actualicen al mismo tiempo desde el punto de vista del juego.
pub fn step(current: &Framebuffer, next: &mut Framebuffer) {
    for y in 0..current.height {
        for x in 0..current.width {
            let alive = is_alive(current.get_color(x, y));
            let neighbors = count_neighbors(current, x, y);

            // `match` en Rust es parecido a un switch mas potente. Aqui se
            // evalua la pareja `(esta_viva, vecinos)` para escribir directamente
            // las reglas de Conway:
            // - viva con 2 o 3 vecinos sobrevive
            // - muerta con 3 vecinos nace
            // - cualquier otro caso muere o sigue muerta
            let will_live = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };

            // Si la celula vive, elegimos un color de la paleta. Si muere, se
            // pinta con el color de fondo.
            let color = if will_live {
                alive_color(x, y, neighbors)
            } else {
                DEAD
            };

            next.point(x, y, color);
        }
    }
}
