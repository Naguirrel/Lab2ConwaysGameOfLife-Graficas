use crate::framebuffer::{Framebuffer, alive_color};

fn place(fb: &mut Framebuffer, ox: usize, oy: usize, cells: &[(usize, usize)]) {
    for &(cx, cy) in cells {
        let x = ox + cx;
        let y = oy + cy;
        fb.point(x, y, alive_color(x, y, (cx + cy) as u32));
    }
}

pub fn glider(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
}

pub fn blinker(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (2, 0)]);
}

pub fn toad(fb: &mut Framebuffer, ox: usize, oy: usize) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)],
    );
}

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

pub fn load_initial_pattern(fb: &mut Framebuffer) {
    gosper_glider_gun(fb, 2, 3);
    pulsar(fb, 46, 5);
    diamond_burst(fb, 83, 5);
    hwss(fb, 105, 8);

    diehard(fb, 8, 23);
    acorn(fb, 23, 28);
    r_pentomino(fb, 39, 24);
    pentadecathlon(fb, 54, 25);
    spiral_seed(fb, 76, 23);
    comet_seed(fb, 99, 25);

    beehive(fb, 4, 45);
    loaf(fb, 14, 48);
    beacon(fb, 26, 44);
    toad(fb, 39, 47);
    blinker(fb, 52, 45);
    glider(fb, 61, 49);
    pulsar(fb, 74, 43);
    mwss(fb, 101, 48);

    diamond_burst(fb, 8, 67);
    gosper_glider_gun(fb, 30, 66);
    lwss(fb, 75, 69);
    diehard(fb, 91, 70);
    acorn(fb, 106, 75);

    block(fb, 7, 93);
    boat(fb, 17, 96);
    tub(fb, 28, 94);
    spiral_seed(fb, 40, 91);
    comet_seed(fb, 58, 94);
    pentadecathlon(fb, 80, 94);
    glider(fb, 103, 93);
    lwss(fb, 111, 101);
}
