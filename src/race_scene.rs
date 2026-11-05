//! OBJ characters animated around explicit joint pivots; the course uses shared CPU/GPU meshes.
use crate::{
    game::{Game, Mode},
    input::Controls,
    math::Vec3,
    obj::Model,
    race::{self, Hero, Kind, Phase, Race},
    renderer::{Material, Mesh, Renderer, Style},
    scene::Scene,
    ui,
};
use std::{error::Error, f32::consts::PI, path::Path};
struct Rig {
    core: Model,
    arms: [Model; 2],
    legs: [Model; 2],
    tails: Option<Model>,
    ball: Model,
}
pub struct Visuals {
    rigs: Vec<Rig>,
    road: Mesh,
    sides: Mesh,
    grass: Mesh,
    ring: Mesh,
}
fn blank() -> Mesh {
    Mesh {
        radius: 0.,
        vertices: vec![],
        normals: vec![],
        faces: vec![],
    }
}
fn quad(m: &mut Mesh, p: [Vec3; 4], n: [Vec3; 4]) {
    let k = m.vertices.len();
    m.vertices.extend(p);
    m.normals.extend(n);
    m.faces.extend([[k, k + 1, k + 2], [k, k + 2, k + 3]]);
    for v in p {
        m.radius = m.radius.max(v.length());
    }
}
impl Visuals {
    pub fn new(assets: &Path) -> Result<Self, Box<dyn Error>> {
        let mut rigs = vec![];
        for name in ["sonic", "tails", "knuckles"] {
            let part = |p: &str| Model::load(&assets.join(format!("models/{name}-{p}.obj")));
            rigs.push(Rig {
                core: part("core")?,
                arms: [part("arm-l")?, part("arm-r")?],
                legs: [part("leg-l")?, part("leg-r")?],
                tails: if name == "tails" {
                    Some(part("tails")?)
                } else {
                    None
                },
                ball: part("ball")?,
            });
        }
        let mut road = blank();
        let mut sides = blank();
        let mut grass = blank();
        for k in 0..(race::GOAL as usize / 2 + 20) {
            let a = k as f32 * 2. - 16.;
            let b = a + 2.;
            if race::gap((a + b) / 2.) {
                continue;
            }
            let fa = race::frame(a);
            let fb = race::frame(b);
            let p = |f: race::Frame, x: f32, h: f32| f.p + Vec3::new(x, 0., 0.) + f.up * h;
            quad(
                &mut road,
                [
                    p(fa, -5.8, 0.),
                    p(fa, 5.8, 0.),
                    p(fb, 5.8, 0.),
                    p(fb, -5.8, 0.),
                ],
                [fa.up, fa.up, fb.up, fb.up],
            );
            for side in [-1., 1.] {
                quad(
                    &mut grass,
                    [
                        p(fa, side * 5.8, 0.03),
                        p(fa, side * 7.7, 0.03),
                        p(fb, side * 7.7, 0.03),
                        p(fb, side * 5.8, 0.03),
                    ],
                    [fa.up, fa.up, fb.up, fb.up],
                );
                // The near wall is open through the loop, like a cutaway track,
                // so its six-metre foundation never hides the racer on entry/exit.
                if side < 0.
                    || !(race::LOOP_START - 2. ..race::LOOP_START + race::LOOP_LENGTH + 2.)
                        .contains(&a)
                {
                    quad(
                        &mut sides,
                        [
                            p(fa, side * 7.7, 0.),
                            p(fa, side * 7.7, -6.),
                            p(fb, side * 7.7, -6.),
                            p(fb, side * 7.7, 0.),
                        ],
                        [Vec3::new(side, 0., 0.); 4],
                    );
                }
            }
        }
        let mut ring = Mesh::portal();
        for (v, n) in ring.vertices.iter_mut().zip(&ring.normals) {
            *v = *v + *n * 0.17;
        }
        ring.radius = 2.56;
        Ok(Self {
            rigs,
            road,
            sides,
            grass,
            ring,
        })
    }
    #[allow(clippy::too_many_arguments)]
    fn hero(
        &self,
        r: &mut Renderer,
        id: Hero,
        root: Vec3,
        scale: f32,
        yaw: f32,
        pitch: f32,
        time: f32,
        speed: f32,
        ability: bool,
        rolling: bool,
    ) {
        let rig = &self.rigs[id.index()];
        let turn = |p: Vec3| root + (p * scale).rotate_x(pitch).rotate_y(yaw);
        if rolling {
            r.oriented_model(
                &rig.ball,
                turn(Vec3::new(0., 0.9, 0.)),
                scale,
                yaw,
                pitch + time * 23.,
                0.,
            );
            return;
        }
        let cycle = time * (speed * 0.34 + 2.);
        let swing = cycle.sin() * if speed > 1. { 0.65 } else { 0.03 };
        let bob = if speed > 1. {
            (cycle * 2.).sin().abs() * 0.07
        } else {
            (time * 2.).sin() * 0.025
        };
        let core = turn(Vec3::new(0., bob, 0.));
        r.oriented_model(&rig.core, core, scale, yaw, pitch, 0.);
        for (k, side) in [-1., 1.].iter().enumerate() {
            let wing = ability && id != Hero::Sonic;
            let arm_pitch = if wing { 0. } else { -swing * side };
            let arm_bank = if wing { side * 1.1 } else { side * 0.08 };
            r.oriented_model(
                &rig.arms[k],
                turn(Vec3::new(side * 0.38, 2.12 + bob, 0.)),
                scale,
                yaw,
                pitch + arm_pitch,
                arm_bank,
            );
            r.oriented_model(
                &rig.legs[k],
                turn(Vec3::new(side * 0.19, 1.38 + bob, 0.)),
                scale,
                yaw,
                pitch + swing * side,
                0.,
            );
        }
        if let Some(tails) = &rig.tails {
            r.oriented_model(
                tails,
                turn(Vec3::new(0., 1.55 + bob, -0.22)),
                scale,
                yaw,
                pitch,
                if ability {
                    time * 25.
                } else {
                    (time * 5.).sin() * 0.14
                },
            );
        }
    }
    fn ring(&self, r: &mut Renderer, p: Vec3, size: f32, yaw: f32) {
        r.mesh(
            &self.ring,
            p,
            Vec3::new(size, size, size),
            yaw,
            Material::new([255, 192, 24], Style::Metal),
        );
    }
}
pub fn camera(r: &Race) -> (Vec3, Vec3) {
    if r.phase == Phase::Choose {
        (Vec3::new(0., 3.6, 12.8), Vec3::new(0., 1.9, 0.))
    } else {
        let p = r.position();
        let follow = p + Vec3::new(-r.x * 0.45, 4.7, 11.8);
        let aim = p + Vec3::new(0., 1.3, -7.);
        // Open before the loop and return only after clearing it. Switching at
        // the exact seam made the smoothed camera cross through the track wall.
        let approach = (r.distance - race::LOOP_START + 45.) / 45.;
        let departure = (race::LOOP_START + race::LOOP_LENGTH + 45. - r.distance) / 45.;
        let w = approach.min(departure).clamp(0., 1.);
        let w = w * w * (3. - 2. * w);
        let center = race::frame(race::LOOP_START).p + Vec3::new(0., race::LOOP_RADIUS, 0.);
        let wide = center + Vec3::new(42., 5., 0.) + (p - center) * 0.12;
        (follow.lerp(wide, w), aim.lerp(p.lerp(center, 0.25), w))
    }
}
pub fn preview(s: &Scene, r: &mut Renderer, name: &str) {
    let hero = match name {
        "tails-model" => Hero::Tails,
        "knuckles-model" => Hero::Knuckles,
        _ => Hero::Sonic,
    };
    r.begin(Vec3::new(3.6, 2.7, 6.8), Vec3::new(0., 1.55, 0.), 0.);
    s.race.hero(
        r,
        hero,
        Vec3::default(),
        1.,
        -0.15,
        0.,
        0.,
        0.,
        false,
        false,
    );
}
fn sky(r: &mut Renderer) {
    for y in 0..r.height {
        let t = y as f32 / r.height as f32;
        r.rect(
            0,
            y as i32,
            r.width as i32,
            1,
            [
                (28. + t * 113.) as u8,
                (127. + t * 111.) as u8,
                (230. + t * 25.) as u8,
            ],
        );
    }
}
fn block(s: &Scene, r: &mut Renderer, p: Vec3, scale: Vec3, c: [u8; 3]) {
    s.world.block(r, p, scale, c);
}
pub fn render(s: &Scene, r: &mut Renderer, g: &Game) {
    let race = &g.race;
    r.begin(g.camera_eye, g.camera_target, g.time);
    sky(r);
    if race.phase == Phase::Choose {
        block(
            s,
            r,
            Vec3::new(0., -0.45, -1.),
            Vec3::new(18., 0.30, 12.),
            [51, 181, 42],
        );
        for (k, hero) in Hero::ALL.iter().enumerate() {
            let x = (k as f32 - 1.) * 4.5;
            r.mesh(
                &s.cylinder,
                Vec3::new(x, -0.11, 0.),
                Vec3::new(1.65, 0.18, 1.65),
                0.,
                Material::new(
                    if *hero == race.hero {
                        [255, 211, 43]
                    } else {
                        [226, 243, 251]
                    },
                    Style::Soft,
                ),
            );
            s.race.hero(
                r,
                *hero,
                Vec3::new(x, 0.09, 0.),
                1.30,
                if *hero == race.hero {
                    (g.time * 0.6).sin() * 0.14
                } else {
                    0.
                },
                0.,
                g.time,
                0.,
                false,
                false,
            );
            s.race.ring(r, Vec3::new(x, 3., -2.), 0.85, 0.);
        }
        for side in [-1., 1.] {
            s.palm(r, Vec3::new(side * 8., 0., -4.), 2.4);
            s.hill_loop(r, Vec3::new(side * 12., 0., -15.), 3.);
        }
        for k in -3..4 {
            s.ball(
                r,
                Vec3::new(k as f32 * 7., -2., -21.),
                Vec3::new(6., 9., 6.),
                0.,
                [64, 204, 32],
                Style::Soft,
            );
        }
        return;
    }
    let origin = Vec3::default();
    let unit = Vec3::new(1., 1., 1.);
    // Island under the circuit, with turquoise water below the cliff edges.
    block(
        s,
        r,
        Vec3::new(0., -8., -race.distance - 55.),
        Vec3::new(130., 0.12, 200.),
        [14, 184, 223],
    );
    r.mesh(
        &s.race.sides,
        origin,
        unit,
        0.,
        Material::new([194, 107, 43], Style::Checker),
    );
    r.mesh(
        &s.race.road,
        origin,
        unit,
        0.,
        Material::new([225, 189, 111], Style::Soft),
    );
    r.mesh(
        &s.race.grass,
        origin,
        unit,
        0.,
        Material::new([106, 232, 20], Style::Grass),
    );
    // Distinct stripes and shoulders make steering readable at high speed.
    for k in 0..(race::GOAL as usize / 8) {
        let at = k as f32 * 8.;
        if (at - race.distance).abs() > 140. || race::gap(at) {
            continue;
        }
        let f = race::frame(at);
        for side in [-1., 1.] {
            s.ball(
                r,
                f.p + Vec3::new(side * 5.9, 0., 0.) + f.up * 0.10,
                Vec3::new(0.10, 0.08, 0.85),
                0.,
                [255, 244, 175],
                Style::Glow,
            );
        }
    }
    // No trees or scenery intersect the loop's playable tube.
    for k in 0..55 {
        let at = k as f32 * 22.;
        if (at - race.distance).abs() > 170.
            || (race::LOOP_START - 22. ..race::LOOP_START + race::LOOP_LENGTH + 22.).contains(&at)
        {
            continue;
        }
        let f = race::frame(at);
        for side in [-1., 1.] {
            let p = f.p + Vec3::new(side * (9.5 + (k % 3) as f32), 0., 0.);
            if k % 2 == 0 {
                s.palm(r, p, 1.7 + (k % 3) as f32 * 0.2);
            }
            s.world
                .checker_block(r, p - Vec3::new(0., 3.2, 0.), Vec3::new(3., 3., 6.));
            block(
                s,
                r,
                p - Vec3::new(0., 0.13, 0.),
                Vec3::new(3.1, 0.12, 6.1),
                [94, 217, 24],
            );
            s.ball(
                r,
                p + Vec3::new(side * 14., -2., -9.),
                Vec3::new(10., 12. + (k % 4) as f32 * 3., 12.),
                0.,
                [54, 186, 37],
                Style::Soft,
            );
            if k % 5 == 0 {
                s.hill_loop(r, p + Vec3::new(side * 7., 0., -7.), 2.8);
            }
        }
    }
    for item in &race.items {
        if item.taken || item.s < race.distance - 16. || item.s > race.distance + 135. {
            continue;
        }
        let f = race::frame(item.s);
        let p = f.p + Vec3::new(item.x, 0., 0.);
        match item.kind {
            Kind::Ring => s.race.ring(r, p + f.up * 1.25, 0.20, g.time * 2.),
            Kind::Boost => {
                block(
                    s,
                    r,
                    p + Vec3::new(0., 0.07, 0.),
                    Vec3::new(1.1, 0.07, 1.1),
                    [22, 83, 226],
                );
                for j in 0..3 {
                    block(
                        s,
                        r,
                        p + Vec3::new(0., 0.16, j as f32 * 0.55 - 0.55),
                        Vec3::new(0.65, 0.02, 0.12),
                        [255, 213, 26],
                    );
                }
            }
            Kind::Spring => {
                r.mesh(
                    &s.cylinder,
                    p + Vec3::new(0., 0.30, 0.),
                    Vec3::new(0.52, 0.30, 0.52),
                    0.,
                    Material::new([205, 216, 223], Style::Metal),
                );
                r.mesh(
                    &s.cylinder,
                    p + Vec3::new(0., 0.66, 0.),
                    Vec3::new(0.75, 0.12, 0.75),
                    0.,
                    Material::new([235, 38, 48], Style::Soft),
                );
            }
            Kind::Crate => {
                s.world
                    .checker_block(r, p + Vec3::new(0., 0.62, 0.), Vec3::new(0.65, 0.62, 0.65));
                block(
                    s,
                    r,
                    p + Vec3::new(0., 0.65, 0.67),
                    Vec3::new(0.55, 0.055, 0.02),
                    [255, 213, 79],
                );
            }
            Kind::Spikes => {
                block(
                    s,
                    r,
                    p + Vec3::new(0., 0.10, 0.),
                    Vec3::new(1., 0.10, 0.65),
                    [88, 106, 125],
                );
                for side in [-0.6, 0., 0.6] {
                    s.ball(
                        r,
                        p + Vec3::new(side, 0.5, 0.),
                        Vec3::new(0.16, 0.50, 0.16),
                        0.,
                        [215, 230, 242],
                        Style::Metal,
                    );
                }
            }
            Kind::Robot => {
                s.ball(
                    r,
                    p + Vec3::new(0., 0.9, 0.),
                    Vec3::new(0.7, 0.5, 0.55),
                    0.,
                    [220, 34, 43],
                    Style::Soft,
                );
                block(
                    s,
                    r,
                    p + Vec3::new(0., 1., 0.48),
                    Vec3::new(0.45, 0.12, 0.08),
                    [32, 41, 62],
                );
                for side in [-1., 1.] {
                    s.ball(
                        r,
                        p + Vec3::new(side * 0.26, 1., 0.58),
                        Vec3::new(0.09, 0.09, 0.04),
                        0.,
                        [255, 234, 43],
                        Style::Glow,
                    );
                    s.ball(
                        r,
                        p + Vec3::new(side * 0.63, 0.30, 0.),
                        Vec3::new(0.26, 0.28, 0.34),
                        0.,
                        [66, 84, 110],
                        Style::Metal,
                    );
                }
            }
        }
    }
    // Finish arch and visible checkpoint gates.
    for at in [240., 480., 720., 900., race::GOAL] {
        if (at - race.distance).abs() > 150. {
            continue;
        }
        let p = race::frame(at).p;
        for side in [-1., 1.] {
            block(
                s,
                r,
                p + Vec3::new(side * 7., 3., 0.),
                Vec3::new(0.20, 3., 0.20),
                [239, 245, 250],
            );
        }
        for k in -7..7 {
            block(
                s,
                r,
                p + Vec3::new(k as f32 + 0.5, 6., 0.),
                Vec3::new(0.5, 0.35, 0.20),
                if k % 2 == 0 {
                    [245, 250, 253]
                } else {
                    [18, 48, 112]
                },
            );
        }
    }
    for (k, distance) in race.rivals.iter().enumerate() {
        if (*distance - race.distance).abs() > 55. {
            continue;
        }
        let hero = Hero::ALL[(race.hero.index() + k + 1) % 3];
        let f = race::frame(*distance);
        let jump = [(160., 175.), (540., 558.), (790., 807.)]
            .iter()
            .find_map(|&(a, b)| {
                if *distance > a - 8. && *distance < b + 8. {
                    Some(((*distance - a + 8.) / (b - a + 16.) * PI).sin() * 3.5)
                } else {
                    None
                }
            })
            .unwrap_or(0.);
        let p = f.p + Vec3::new(if k == 0 { -3.2 } else { 3.2 }, 0., 0.) + f.up * jump;
        s.race.hero(
            r,
            hero,
            p,
            0.83,
            PI,
            f.pitch,
            g.time + k as f32,
            23.,
            false,
            false,
        );
    }
    let pos = race.position();
    let f = race::frame(race.distance);
    // Flicker only during hit invulnerability, not after the race is over.
    if race.invulnerable <= 0. || (g.time * 14.) as i32 % 2 == 0 {
        s.race.hero(
            r,
            race.hero,
            pos,
            0.95,
            PI,
            f.pitch,
            g.time,
            if race.phase == Phase::Running {
                race.speed
            } else {
                0.
            },
            race.ability > 0.,
            race.rolling > 0. || race.charge > 0.15,
        );
    }
    if race.boosting || race.rolling > 0. {
        for j in 1..7 {
            let f = race::frame((race.distance - j as f32 * 1.1).max(0.));
            s.ball(
                r,
                f.p + Vec3::new(race.x, 1., 0.),
                Vec3::new(0.28, 0.25, 0.6),
                0.,
                race.hero.color(),
                Style::Glow,
            );
        }
    }
}
fn label(r: &mut Renderer, y: i32, s: &str, size: i32, c: [u8; 3]) {
    ui::text(r, (640 - s.len() as i32 * 6 * size) / 2, y, s, size, c);
}
fn panel(r: &mut Renderer, x: i32, y: i32, w: i32, h: i32) {
    r.rect(x, y, w, h, [10, 40, 114]);
    r.rect(x, y, w, 2, [255, 212, 42]);
}
pub fn hud(r: &mut Renderer, g: &Game, controls: &Controls) {
    let race = &g.race;
    let pad = controls.using_pad;
    if race.phase == Phase::Choose {
        panel(r, 0, 0, 640, 58);
        label(r, 10, "SONIC GRAND PRIX", 3, [255, 226, 75]);
        label(
            r,
            39,
            "ELIGE TU CORREDOR / GREEN HILL CIRCUIT",
            1,
            [214, 243, 255],
        );
        for (k, hero) in Hero::ALL.iter().enumerate() {
            let x = 36 + k as i32 * 194;
            let chosen = *hero == race.hero;
            panel(r, x, 232, 180, 55);
            if chosen {
                r.rect(x, 232, 180, 3, [255, 228, 67]);
            }
            ui::text(
                r,
                x + 12,
                241,
                hero.name(),
                2,
                if chosen {
                    [255, 228, 67]
                } else {
                    [227, 242, 255]
                },
            );
            let desc = hero.skill();
            ui::text(r, x + 12, 268, desc, 1, [185, 225, 249]);
        }
        panel(r, 0, 300, 640, 60);
        label(
            r,
            310,
            if pad {
                "STICK: ELEGIR   X: CORRER   CIRCULO: SALIR"
            } else {
                "A/D: ELEGIR   ENTER: CORRER   Q: SALIR"
            },
            1,
            [255, 234, 117],
        );
        label(
            r,
            328,
            "1050 M / DOS RIVALES / GANA LA QUINTA INSIGNIA",
            1,
            [216, 241, 255],
        );
        return;
    }
    panel(r, 0, 0, 640, 45);
    ui::text(r, 12, 9, race.hero.name(), 2, [255, 230, 90]);
    ui::text(
        r,
        180,
        10,
        &format!("ANILLOS {:03}  VIDAS {}", race.rings, race.lives.max(0)),
        1,
        [255, 229, 87],
    );
    ui::text(
        r,
        180,
        28,
        &format!(
            "TIEMPO {:05.1}   META {:04.0} M",
            race.time,
            (race::GOAL - race.distance).max(0.)
        ),
        1,
        [211, 242, 255],
    );
    ui::text(r, 477, 9, &format!("{} / 3", race.rank), 2, [255, 231, 105]);
    ui::text(
        r,
        477,
        31,
        &format!("{:03.0} KM/H", race.speed * 3.6),
        1,
        [218, 244, 255],
    );
    r.rect(12, 51, 616, 4, [16, 63, 122]);
    r.rect(
        12,
        51,
        (616. * race.distance / race::GOAL) as i32,
        4,
        [255, 220, 45],
    );
    panel(r, 12, 65, 135, 34);
    ui::text(r, 19, 73, "BOOST", 1, [214, 244, 255]);
    r.rect(58, 74, 80, 7, [25, 66, 117]);
    r.rect(58, 74, (race.energy * 0.8) as i32, 7, [69, 218, 255]);
    if race.cooldown > 0. {
        ui::text(
            r,
            19,
            88,
            &format!("HABILIDAD {:0.1} S", race.cooldown),
            1,
            [185, 213, 237],
        );
    } else {
        ui::text(
            r,
            19,
            88,
            if pad { "R1: HABILIDAD" } else { "L: HABILIDAD" },
            1,
            [255, 227, 87],
        );
    }
    if race.charge > 0.05 {
        label(
            r,
            251,
            &format!("SPIN DASH {:03.0}% / SUELTA EL FRENO", race.charge * 100.),
            1,
            [255, 225, 74],
        );
    }
    if race.notice_left > 0. {
        panel(r, 180, 66, 280, 22);
        label(r, 74, race.notice, 1, [255, 229, 92]);
    }
    if race.phase == Phase::Countdown {
        label(r, 125, "READY?", 3, [255, 234, 96]);
        label(
            r,
            165,
            &format!("{}", race.countdown.ceil() as i32),
            6,
            [255, 241, 175],
        );
    }
    panel(r, 0, 315, 640, 45);
    label(
        r,
        325,
        if pad {
            "L: DIRECCION  X: SALTO  R2: BOOST  L2: SPIN DASH  R1: PODER"
        } else {
            "A/D: GIRAR  ESPACIO: SALTO  SHIFT: BOOST  K: SPIN DASH  L: PODER"
        },
        1,
        [255, 232, 108],
    );
    if g.mode == Mode::Defeat {
        label(
            r,
            109,
            &format!("POSICION {} / 3 - REINTENTA PARA GANAR", race.rank),
            1,
            [255, 176, 112],
        );
    }
}
