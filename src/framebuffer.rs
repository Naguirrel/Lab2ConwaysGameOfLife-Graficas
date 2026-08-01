// Color del fondo. El formato usado por minifb es 0x00RRGGBB:
// dos digitos hexadecimales para rojo, verde y azul.
pub const DEAD: u32 = 0x00070A18;

// Paleta para las celulas vivas. Aunque el juego de Conway solo necesita saber
// si una celula esta viva o muerta, usar varios colores hace que el resultado
// visual sea mas interesante.
const ALIVE_COLORS: [u32; 7] = [
    0x00F7D046, // yellow
    0x00FF4D8D, // pink
    0x0000D9FF, // cyan
    0x007CF56B, // green
    0x00B86BFF, // violet
    0x00FF8A3D, // orange
    0x00FFFFFF, // white
];

// En esta version, una celula esta viva si su color no es el color de fondo.
// Esto permite que haya muchas celulas vivas con colores distintos sin cambiar
// las reglas del juego.
pub fn is_alive(color: u32) -> bool {
    color != DEAD
}

// Escoge un color vivo de forma deterministica usando la posicion y la cantidad
// de vecinos. No es aleatorio: la misma celula con los mismos datos siempre
// recibe el mismo color, lo cual hace que la animacion sea estable.
pub fn alive_color(x: usize, y: usize, neighbors: u32) -> u32 {
    // `as usize` convierte `neighbors` al mismo tipo que usamos para indices.
    // El modulo `%` mantiene el indice dentro del tamano de la paleta.
    let index = (x * 3 + y * 5 + neighbors as usize) % ALIVE_COLORS.len();
    ALIVE_COLORS[index]
}

// Un framebuffer es una imagen en memoria. En vez de guardar filas separadas,
// usamos un solo `Vec<u32>` con todos los colores seguidos.
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    // `Self` significa "el tipo que estamos implementando", en este caso
    // `Framebuffer`. Esta funcion construye una imagen nueva llena del color
    // de fondo.
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![DEAD; width * height],
        }
    }

    // Dibuja un punto en coordenadas 2D. El `if` evita escribir fuera del vector
    // si accidentalmente se intenta dibujar fuera de la pantalla.
    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            // Conversion de 2D a 1D:
            // fila y * ancho + columna x.
            self.buffer[y * self.width + x] = color;
        }
    }

    // Lee el color de una celula. Si la coordenada no existe, regresamos fondo.
    // Eso evita errores de indices cuando otra parte del codigo pregunta por
    // una posicion fuera de la grilla.
    pub fn get_color(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x]
        } else {
            DEAD
        }
    }
}
