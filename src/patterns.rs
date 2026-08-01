// Este archivo contiene los organismos y semillas iniciales del Game of Life.
// Cada funcion dibuja una figura usando coordenadas relativas a un origen.
use crate::framebuffer::{Framebuffer, alive_color};

// `cells` es un slice: una vista prestada de una lista de coordenadas.
// Cada par `(cx, cy)` indica una celula viva relativa al origen `(ox, oy)`.
// Asi podemos definir una figura una sola vez y colocarla en cualquier parte.
fn place(fb: &mut Framebuffer, ox: usize, oy: usize, cells: &[(usize, usize)]) {
    // `for &(cx, cy)` copia los valores del par. El `&` aparece porque al
    // recorrer un slice Rust nos entrega referencias a sus elementos.
    for &(cx, cy) in cells {
        let x = ox + cx;
        let y = oy + cy;

        // `point` es la unica funcion que realmente pinta pixeles. Todo lo demas
        // en este archivo solo decide que coordenadas deben encenderse.
        fb.point(x, y, alive_color(x, y, (cx + cy) as u32));
    }
}

// Un glider es una nave pequena: se desplaza diagonalmente con el tiempo.
pub fn glider(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
}

// Oscilador simple: alterna entre horizontal y vertical cada turno.
pub fn blinker(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (2, 0)]);
}

// Oscilador de periodo 2. Cambia de forma y vuelve a su estado inicial.
pub fn toad(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)],
    );
}

// Oscilador de periodo 2 formado por dos bloques que se activan por turnos.
pub fn beacon(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (0, 0),
            (1, 0),
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 2),
            (2, 3),
            (3, 3),
        ],
    );
}

// Still lifes: estas figuras quedan estables si no reciben interferencia.
pub fn block(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (0, 1), (1, 1)]);
}

pub fn beehive(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)],
    );
}

pub fn loaf(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)],
    );
}

pub fn boat(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)]);
}

pub fn tub(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(1, 0), (0, 1), (2, 1), (1, 2)]);
}

// Semillas caoticas: empiezan pequenas, pero producen muchos cambios antes de
// estabilizarse. Sirven para que la animacion tenga movimiento interesante.
pub fn r_pentomino(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)]);
}

pub fn diehard(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(6, 0), (0, 1), (1, 1), (1, 2), (5, 2), (6, 2), (7, 2)],
    );
}

pub fn acorn(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)],
    );
}

// Spaceships: estas figuras se mueven por la pantalla despues de varios turnos.
pub fn lwss(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (0, 1),
            (4, 1),
            (4, 2),
            (0, 3),
            (3, 3),
        ],
    );
}

pub fn mwss(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (2, 0),
            (0, 1),
            (3, 1),
            (4, 2),
            (0, 3),
            (4, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
        ],
    );
}

pub fn hwss(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (2, 0),
            (3, 0),
            (0, 1),
            (4, 1),
            (5, 2),
            (0, 3),
            (5, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
        ],
    );
}

pub fn pentadecathlon(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (2, 0),
            (7, 0),
            (0, 1),
            (1, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (8, 1),
            (9, 1),
            (2, 2),
            (7, 2),
        ],
    );
}

// El pulsar es un oscilador mas grande de periodo 3.
// Se construye con simetria: primero dibujamos varias barras horizontales y
// luego repetimos la misma idea intercambiando filas y columnas.
pub fn pulsar(fb: &mut Framebuffer, ox: usize, oy: usize) {
    let rows = [0, 5, 7, 12];
    let cols = [2, 3, 4, 8, 9, 10];
    for &r in &rows {
        for &c in &cols {
            fb.point(ox + c, oy + r, alive_color(ox + c, oy + r, (c + r) as u32));
        }
    }
    for &c in &rows {
        for &r in &cols {
            fb.point(ox + c, oy + r, alive_color(ox + c, oy + r, (c + r) as u32));
        }
    }
}

// El Gosper glider gun es un patron clasico que genera gliders repetidamente.
// Es util para mostrar que el algoritmo no solo parpadea: puede producir
// estructuras que viajan por la pantalla.
pub fn gosper_glider_gun(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (24, 0),
            (22, 1),
            (24, 1),
            (12, 2),
            (13, 2),
            (20, 2),
            (21, 2),
            (34, 2),
            (35, 2),
            (11, 3),
            (15, 3),
            (20, 3),
            (21, 3),
            (34, 3),
            (35, 3),
            (0, 4),
            (1, 4),
            (10, 4),
            (16, 4),
            (20, 4),
            (21, 4),
            (0, 5),
            (1, 5),
            (10, 5),
            (14, 5),
            (16, 5),
            (17, 5),
            (22, 5),
            (24, 5),
            (10, 6),
            (16, 6),
            (24, 6),
            (11, 7),
            (15, 7),
            (12, 8),
            (13, 8),
        ],
    );
}

// Las siguientes tres figuras no son organismos clasicos estrictos; son semillas
// decorativas hechas a mano para que el inicio del GIF tenga siluetas distintas.
pub fn diamond_burst(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (4, 0),
            (3, 1),
            (5, 1),
            (2, 2),
            (6, 2),
            (1, 3),
            (3, 3),
            (5, 3),
            (7, 3),
            (0, 4),
            (2, 4),
            (6, 4),
            (8, 4),
            (1, 5),
            (3, 5),
            (5, 5),
            (7, 5),
            (2, 6),
            (6, 6),
            (3, 7),
            (5, 7),
            (4, 8),
        ],
    );
}

pub fn spiral_seed(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (2, 0),
            (3, 0),
            (4, 0),
            (4, 1),
            (4, 2),
            (3, 2),
            (2, 2),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
            (5, 5),
            (5, 6),
            (4, 6),
            (3, 6),
        ],
    );
}

pub fn comet_seed(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[
            (0, 2),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 0),
            (2, 2),
            (2, 4),
            (3, 2),
            (4, 1),
            (4, 2),
            (4, 3),
            (6, 0),
            (7, 1),
            (8, 2),
            (7, 3),
            (6, 4),
        ],
    );
}

// Esta funcion arma la pantalla inicial completa.
// La idea es mezclar organismos estables, osciladores, naves y semillas caoticas
// en posiciones diferentes para que el GIF no se vea como una tabla repetida.
pub fn load_initial_pattern(fb: &mut Framebuffer) {
    // Zona superior: un canon, un oscilador grande y figuras que se expanden.
    gosper_glider_gun(fb, 2, 3);
    pulsar(fb, 46, 5);
    diamond_burst(fb, 83, 5);
    hwss(fb, 105, 8);

    // Segunda franja: semillas caoticas que generan cambios durante muchos
    // frames y evitan que la simulacion se estabilice demasiado rapido.
    diehard(fb, 8, 23);
    acorn(fb, 23, 28);
    r_pentomino(fb, 39, 24);
    pentadecathlon(fb, 54, 25);
    spiral_seed(fb, 76, 23);
    comet_seed(fb, 99, 25);

    // Centro: mezcla de still lifes, osciladores y naves pequenas.
    beehive(fb, 4, 45);
    loaf(fb, 14, 48);
    beacon(fb, 26, 44);
    toad(fb, 39, 47);
    blinker(fb, 52, 45);
    glider(fb, 61, 49);
    pulsar(fb, 74, 43);
    mwss(fb, 101, 48);

    // Parte baja-media: otra fuente de gliders y patrones activos.
    diamond_burst(fb, 8, 67);
    gosper_glider_gun(fb, 30, 66);
    lwss(fb, 75, 69);
    diehard(fb, 91, 70);
    acorn(fb, 106, 75);

    // Base: figuras pequenas distribuidas de forma irregular para llenar la
    // pantalla sin repetir exactamente el mismo bloque de patrones.
    block(fb, 7, 93);
    boat(fb, 17, 96);
    tub(fb, 28, 94);
    spiral_seed(fb, 40, 91);
    comet_seed(fb, 58, 94);
    pentadecathlon(fb, 80, 94);
    glider(fb, 103, 93);
    lwss(fb, 111, 101);
}
