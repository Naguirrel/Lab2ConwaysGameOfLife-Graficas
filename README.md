# Lab2 - Conway's Game of Life

Este proyecto implementa el algoritmo de Conway's Game of Life en Rust usando un framebuffer propio y la libreria `minifb` para mostrar la simulacion en tiempo real.

La idea principal del laboratorio es renderizar una grilla de celulas usando la funcion `point`. Cada celula se representa como un pixel del framebuffer: si la celula esta viva se pinta con un color de la paleta, y si esta muerta se pinta con el color de fondo.

## Resultado visual

El proyecto incluye un GIF generado:

```text
Lab2.gif
```

Ese archivo sirve como evidencia visual del programa corriendo. La animacion usa una paleta de varios colores y un patron inicial con organismos diferentes para que la pantalla no se vea repetida.

## Requisitos

- Rust instalado.
- Cargo, que normalmente viene incluido con Rust.
- Windows, Linux o macOS. En Windows se usa `winapi` para ajustar el escalado DPI de la ventana.

## Como ejecutar

Desde la carpeta del proyecto:

```powershell
cargo run
```

Para revisar que el programa compila sin ejecutarlo:

```powershell
cargo check
```

Para formatear el codigo:

```powershell
cargo fmt
```

Durante la ejecucion se abre una ventana con la simulacion. Para salir, se puede cerrar la ventana o presionar `Escape`.

## Estructura del proyecto

```text
Lab2/
  .git/
  .gitignore
  Cargo.toml
  Cargo.lock
  Lab2.gif
  README.md
  src/
    main.rs
    framebuffer.rs
    line.rs
    patterns.rs
```

## Archivos principales

### `src/main.rs`

Es el punto de entrada del programa.

Responsabilidades:

- Declara los modulos del proyecto: `framebuffer`, `line` y `patterns`.
- Define el tamano de la grilla con `GRID_WIDTH` y `GRID_HEIGHT`.
- Crea dos framebuffers: `current` y `next`.
- Carga el patron inicial.
- Crea la ventana con `minifb`.
- Ejecuta el ciclo principal de la simulacion.

El programa usa dos framebuffers porque las reglas de Conway deben calcularse usando el estado anterior completo. Si se modificara el mismo framebuffer mientras se recorre, una celula actualizada podria afectar incorrectamente a sus vecinas en el mismo turno.

### `src/framebuffer.rs`

Define la representacion de la imagen en memoria.

Elementos importantes:

- `DEAD`: color de fondo para las celulas muertas.
- `ALIVE_COLORS`: paleta de colores para celulas vivas.
- `is_alive`: determina si un color representa una celula viva.
- `alive_color`: escoge un color vivo segun posicion y cantidad de vecinos.
- `Framebuffer`: estructura que guarda ancho, alto y buffer de pixeles.
- `point`: dibuja un pixel/celula en el framebuffer.
- `get_color`: obtiene el color de una celula.

El buffer se guarda como un `Vec<u32>`, es decir, una lista lineal de colores. Para convertir coordenadas 2D a una posicion del vector se usa:

```text
indice = y * width + x
```

### `src/line.rs`

Contiene las reglas del Game of Life.

Responsabilidades:

- Contar vecinos vivos alrededor de cada celula.
- Aplicar las reglas de Conway.
- Escribir el siguiente turno en el framebuffer `next`.

Las reglas implementadas son:

1. Una celula viva con menos de 2 vecinos vivos muere.
2. Una celula viva con 2 o 3 vecinos vivos sobrevive.
3. Una celula viva con mas de 3 vecinos vivos muere.
4. Una celula muerta con exactamente 3 vecinos vivos nace.

Los bordes se tratan como si la pantalla estuviera conectada consigo misma. Si una celula sale por la izquierda, aparece por la derecha; si sale por arriba, aparece por abajo. Esto se conoce como borde tipo toro.

### `src/patterns.rs`

Contiene los organismos y patrones iniciales.

Incluye patrones clasicos como:

- `glider`
- `blinker`
- `toad`
- `beacon`
- `block`
- `beehive`
- `loaf`
- `boat`
- `tub`
- `r_pentomino`
- `diehard`
- `acorn`
- `lwss`
- `mwss`
- `hwss`
- `pentadecathlon`
- `pulsar`
- `gosper_glider_gun`

Tambien incluye semillas decorativas para hacer el resultado visual mas diferente:

- `diamond_burst`
- `spiral_seed`
- `comet_seed`

Cada patron se define como una lista de coordenadas relativas. Por ejemplo, si una figura tiene una celula en `(1, 0)` y se coloca en el origen `(20, 10)`, esa celula se dibuja realmente en `(21, 10)`.

## Funcionamiento general

El flujo del programa es:

1. Se crea un framebuffer inicial.
2. Se dibujan patrones iniciales en posiciones distintas.
3. Se abre una ventana con `minifb`.
4. En cada frame se calcula el siguiente estado del juego.
5. Se intercambian los framebuffers `current` y `next`.
6. Se muestra el nuevo buffer en pantalla.
7. El proceso se repite hasta cerrar la ventana o presionar `Escape`.

## Apartado sobre `.git`

En esta carpeta existe un directorio oculto llamado:

```text
.git/
```

Ese directorio indica que el laboratorio esta dentro de un repositorio Git. Git lo usa para guardar el historial de cambios, ramas, configuracion interna y referencias del proyecto.

No se debe editar manualmente el contenido de `.git/`. Para trabajar con ese directorio se usan comandos de Git, por ejemplo:

```powershell
git status
git add .
git commit -m "Documentacion del laboratorio"
```

Para verificar que la carpeta `.git` esta presente en PowerShell se puede usar:

```powershell
Get-ChildItem -Force
```

La opcion `-Force` muestra archivos y carpetas ocultas, por eso permite ver `.git`.

## Dependencias

El archivo `Cargo.toml` declara las dependencias del proyecto:

```toml
[dependencies]
minifb = "0.27"

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser"] }
```

`minifb` se usa para abrir la ventana y dibujar el framebuffer. `winapi` se usa solo en Windows para configurar el proceso como DPI-aware.

## Notas de implementacion

- El programa usa `point` como funcion principal para pintar celulas.
- `get_color` permite leer el estado de una celula.
- Las celulas vivas pueden tener varios colores.
- Una celula se considera viva si su color es distinto de `DEAD`.
- El patron inicial fue distribuido de forma irregular para que el GIF tenga formas variadas.
- La simulacion corre a 10 FPS para que los cambios se puedan observar con claridad.
