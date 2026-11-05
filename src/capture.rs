//! Offline, deterministic recording of the real renderer and game simulation.
//! Inputs are scripted; cinematic setup shots are distinct from uninterrupted flight.
use crate::{
    duel::{Attack, Stage},
    game::{Game, Mode, WorldId},
    gpu::Gpu,
    input::{Controls, Input},
    math::Vec3,
    renderer::Renderer,
    scene::Scene,
    ui,
};
use raylib::prelude::*;
use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    process::{Child, Command, Stdio},
};
pub const FPS: usize = 30;
pub const SECONDS: usize = 176;
const CUTS: [usize; 12] = [0, 7, 31, 34, 37, 40, 43, 64, 84, 102, 124, 136];
struct Encoder(Option<Child>);
impl Drop for Encoder {
    fn drop(&mut self) {
        if let Some(child) = &mut self.0 {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
fn shot(frame: usize) -> usize {
    CUTS.iter().rposition(|s| frame >= s * FPS).unwrap()
}
fn setup(g: &mut Game, shot: usize) {
    *g = Game::new();
    g.mode = Mode::Flight;
    if (6..=9).contains(&shot) {
        g.active_world = WorldId::ALL[shot - 6];
        g.ship = g.gate();
        g.camera_eye = g.ship + Vec3::new(0., 3., 9.);
        g.camera_target = g.planet_at(g.active_world);
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1. / 120.,
        );
        // Preserve the game's actual entry countdown; no medals or health are fabricated.
    }
    if shot == 11 {
        g.active_world = WorldId::Race;
        g.enter();
    }
    if shot == 1 {
        g.ship = g.world(Vec3::new(0., 2., 18.));
        g.camera_eye = g.ship + Vec3::new(0., 4., 10.);
        g.camera_target = g.ship + Vec3::new(0., 0., -8.);
    }
}
fn demo_input(g: &Game, shot: usize, step: usize) -> Input {
    let t = step as f32 / 120.;
    match shot {
        1 => Input {
            forward: if t < 3. {
                0.22
            } else if t > 14. {
                -0.4
            } else {
                0.
            },
            strafe: if (3. ..14.).contains(&t) { 0.4 } else { 0. },
            orbit: step == 360 || step == 1680,
            ..Input::default()
        },
        6 => {
            let angle = g.battle_time * 2.4;
            let target = g.boss() + Vec3::new(angle.cos(), angle.sin(), 5.);
            let movement = (target - g.player) * 3.;
            Input {
                fire: true,
                strafe: movement.x.clamp(-1., 1.),
                forward: movement.y.clamp(-1., 1.),
                ..Input::default()
            }
        }
        7 => Input {
            brake: g.duel.attack != Attack::Sweep,
            dodge: g.duel.stage == Stage::Telegraph
                && g.duel.attack == Attack::Sweep
                && g.duel.timer < 0.20,
            fire: g.duel.stage == Stage::Recovery,
            ..Input::default()
        },
        8 => {
            let mut i = crate::runner::route_input(&g.runner, step.is_multiple_of(2));
            // Demonstrate jump/slide as genuine input, while steering around trains.
            i.jump = step % 600 == 480;
            i.slide = step % 600 == 240;
            i
        }
        9 => crate::platformer::route_input(&g.platformer),
        11 => {
            if t < 3. {
                Input::default()
            } else if g.race.phase == crate::race::Phase::Running && g.race.time < 1. {
                Input {
                    brake: true,
                    ..Input::default()
                }
            } else {
                crate::race::route_input(&g.race)
            }
        }
        _ => Input::default(),
    }
}
fn camera(g: &mut Game, shot: usize, t: f32) {
    if shot == 0 || shot == 10 {
        let a = 0.6 + t * 0.018;
        g.camera_eye = Vec3::new(a.sin() * 89., 52., a.cos() * 89.);
        g.camera_target = Vec3::new(0., 0., -5.);
    } else if (2..=5).contains(&shot) {
        let id = WorldId::ALL[shot - 2];
        g.active_world = id;
        let p = g.planet_at(id);
        let offset = Vec3::new(9. - t * 0.4, 7., 17.);
        g.camera_eye = p + offset;
        g.camera_target = p + Vec3::new(0., 1., 0.);
        // Ship is parked outside the frame during the labeled model showcase.
        g.ship = p + Vec3::new(0., 0., 32.);
    }
}
fn caption(hud: &mut Renderer, shot: usize, t: f32) {
    let (title, line) = match shot {
        0 => (
            "GREEN HILL GALAXY",
            "UN SISTEMA SOLAR DE CINCO MUNDOS ARCADE",
        ),
        1 => (
            "VUELO CONTINUO / SIN CORTES",
            "ACERCAMIENTO, ORBITA Y CAMARA DE SEGUIMIENTO",
        ),
        2 => ("01 / JARDIN TROPICAL", "PALMERAS, CASCADAS Y AROS DORADOS"),
        3 => ("02 / NEON DOJO", "ARQUITECTURA Y TERRENO AJEDREZADO"),
        4 => (
            "03 / BANANA RUSH",
            "FERROCARRIL Y MINIATURAS SOBRE LA ESFERA",
        ),
        5 => (
            "04 / MUSHROOM KINGDOM",
            "CASTILLO Y BLOQUES / MODELOS OBJ + MTL",
        ),
        6 => (
            "MINIJUEGO 01 / EL JARDIN",
            "MOVIMIENTO, DISPAROS Y PATRONES DEL JEFE",
        ),
        7 => (
            "MINIJUEGO 02 / EL DOJO",
            "BLOQUEAR, ESQUIVAR Y CONTRAATACAR",
        ),
        8 => (
            "MINIJUEGO 03 / LA CARRERA",
            "TRES CARRILES, SALTO Y DESLIZAMIENTO",
        ),
        9 => (
            "MINIJUEGO 04 / PLATAFORMAS",
            "DESPLAZAMIENTO LATERAL, FISICA Y COLISIONES",
        ),
        11 => (
            "MINIJUEGO 05 / SONIC GRAND PRIX",
            "TRES PERSONAJES / SPIN DASH / RIVALES / LOOP",
        ),
        _ if t < 6. => (
            "GRAFICAS POR COMPUTADORA",
            "OBJ + MTL / NORMALES / PROFUNDIDAD / LOD",
        ),
        _ => (
            "INSERT COIN / GALAXY",
            "TECLADO Y DUALSENSE / MODO GPU Y MODO CPU",
        ),
    };
    // Replace the bottom control strip with legible delivery captions.
    let top = if shot == 11 { 300 } else { 315 };
    hud.rect(0, top, 640, 360 - top, [9, 35, 105]);
    hud.rect(0, top, 640, 2, [255, 213, 51]);
    ui::text(hud, 14, top + 9, title, 1, [255, 233, 118]);
    ui::text(hud, 14, top + 27, line, 1, [222, 246, 255]);
    if shot == 11 {
        ui::text(
            hud,
            14,
            345,
            "MOTOR REAL / DEMOSTRACION CON CONTROLES AUTOMATICOS",
            1,
            [185, 225, 255],
        );
    }
    if shot != 9 && shot != 11 {
        hud.rect(0, 0, 640, 45, [9, 35, 105]);
        ui::text(
            hud,
            14,
            10,
            "INSERT COIN: GALAXY / GREEN HILL EDITION",
            1,
            [255, 233, 118],
        );
        ui::text(
            hud,
            14,
            28,
            "CAPTURA DEL JUEGO / RECORRIDO CON CONTROLES AUTOMATICOS",
            1,
            [185, 225, 255],
        );
    }
}
pub fn record(
    path: &Path,
    frame_limit: Option<usize>,
    quality: usize,
    start_frame: usize,
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    let (width, height) = crate::RESOLUTIONS[quality];
    let count = frame_limit.unwrap_or(SECONDS * FPS).min(SECONDS * FPS);
    if count == 0 || start_frame >= count {
        return Err("La grabacion requiere al menos un cuadro".into());
    }
    let child = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-f",
            "rawvideo",
            "-pixel_format",
            "rgba",
            "-video_size",
            &format!("{width}x{height}"),
            "-framerate",
            "30",
            "-i",
            "pipe:0",
            "-an",
            "-c:v",
            "libx264",
            "-preset",
            "fast",
            "-crf",
            "18",
            "-pix_fmt",
            "yuv420p",
            "-movflags",
            "+faststart",
        ])
        .arg(path)
        .stdin(Stdio::piped())
        .spawn()?;
    let mut encoder = Encoder(Some(child));
    let mut pipe = BufWriter::with_capacity(
        width * height * 4,
        encoder
            .0
            .as_mut()
            .unwrap()
            .stdin
            .take()
            .ok_or("Sin pipe de video")?,
    );
    let mut cues = BufWriter::new(File::create(path.with_extension("events.tsv"))?);
    let scene = Scene::new()?;
    let mut g = Game::new();
    let controls = Controls::new();
    let mut r = Renderer::new(width, height);
    r.gpu = true;
    let mut hud = Renderer::new(640, 360);
    hud.gpu = true;
    let (mut rl, thread) = raylib::init()
        .size(960, 540)
        .title("Grabando entrega / Green Hill Galaxy")
        .build();
    rl.set_exit_key(None);
    rl.set_target_fps(0);
    let mut gpu = Gpu::new(&mut rl, &thread, width, height, &crate::assets())?;
    let mut last = usize::MAX;
    for frame in 0..count {
        if rl.window_should_close() {
            return Err("Grabacion cancelada antes de completar el recorrido".into());
        }
        let segment = shot(frame);
        if segment != last {
            setup(&mut g, segment);
            last = segment;
            println!(
                "Video: escena {} / cuadro {} de {}",
                segment + 1,
                frame,
                count
            );
        }
        let local_frame = frame - CUTS[segment] * FPS;
        for sub in 0..4 {
            let i = demo_input(&g, segment, local_frame * 4 + sub);
            g.tick(i, 1. / 120.);
            if frame >= start_frame {
                for cue in &g.events {
                    writeln!(
                        cues,
                        "{:.6}\t{}",
                        ((frame - start_frame) * 4 + sub) as f32 / 120.,
                        cue.file()
                    )?;
                }
            }
            g.events.clear();
        }
        if frame < start_frame {
            continue;
        }
        camera(&mut g, segment, local_frame as f32 / FPS as f32);
        scene.render(&mut r, &g);
        let gameplay = segment == 1 || (6..=9).contains(&segment) || segment == 11;
        if gameplay {
            ui::compose(&mut r, &mut hud, &g, &controls, false, false, 30.);
        } else {
            hud.background.clear();
            hud.overlay.clear();
        }
        caption(&mut hud, segment, local_frame as f32 / FPS as f32);
        gpu.render(&mut rl, &thread, &r, &hud, &g);
        if local_frame == FPS || (gameplay && local_frame == FPS * 9) {
            let still = path.with_file_name(format!(
                "video-shot-{:02}-{:02}.png",
                segment,
                local_frame / FPS
            ));
            gpu.save(&still)?;
        }
        pipe.write_all(&gpu.rgba8()?)?;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        gpu.present(&mut d, Rectangle::new(0., 0., 960., 540.));
        if frame % 300 == 0 {
            println!("Video: {:.0}%", frame as f32 * 100. / count as f32);
        }
    }
    pipe.flush()?;
    drop(pipe);
    cues.flush()?;
    let status = encoder.0.as_mut().unwrap().wait()?;
    encoder.0 = None;
    if !status.success() {
        return Err("FFmpeg no pudo finalizar el video".into());
    }
    println!(
        "Video sin mezcla de audio: {} / {:.1} s",
        path.display(),
        (count - start_frame) as f32 / FPS as f32
    );
    Ok(())
}

const SOFTWARE_SECONDS: usize = 30;
fn software_setup() -> Game {
    let mut g = Game::new();
    g.mode = Mode::Flight;
    g.ship = g.planet_at(WorldId::Garden) + Vec3::new(0., 3., 36.);
    g.camera_eye = g.ship + Vec3::new(0., 4., 9.);
    g.camera_target = g.ship + Vec3::new(0., 0., -12.);
    g
}
fn software_input(step: usize) -> Input {
    let t = step as f32 / 120.;
    Input {
        forward: if (2. ..7.).contains(&t) {
            0.5
        } else if t >= 20. {
            -0.65
        } else {
            0.
        },
        strafe: if (7. ..20.).contains(&t) { 0.7 } else { 0. },
        orbit: step == 7 * 120 || step == 20 * 120,
        ..Input::default()
    }
}

/// Academic evidence: every scene pixel is rasterized by Renderer on the CPU.
/// This path does not initialize a raylib window or construct a Gpu at all.
pub fn record_software(
    path: &Path,
    frame_limit: Option<usize>,
    quality: usize,
    start_frame: usize,
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    let count = frame_limit
        .unwrap_or(SOFTWARE_SECONDS * FPS)
        .min(SOFTWARE_SECONDS * FPS);
    if count == 0 || start_frame >= count {
        return Err("La grabacion CPU requiere al menos un cuadro".into());
    }
    let (width, height) = crate::SOFTWARE_RESOLUTIONS[quality];
    let child = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-f",
            "rawvideo",
            "-pixel_format",
            "rgba",
            "-video_size",
            &format!("{width}x{height}"),
            "-framerate",
            "30",
            "-i",
            "pipe:0",
            "-an",
            "-c:v",
            "libx264",
            "-preset",
            "fast",
            "-crf",
            "18",
            "-pix_fmt",
            "yuv420p",
            "-movflags",
            "+faststart",
        ])
        .arg(path)
        .stdin(Stdio::piped())
        .spawn()?;
    let mut encoder = Encoder(Some(child));
    let mut pipe = BufWriter::with_capacity(
        width * height * 4,
        encoder
            .0
            .as_mut()
            .unwrap()
            .stdin
            .take()
            .ok_or("Sin pipe de video CPU")?,
    );
    let mut cues = BufWriter::new(File::create(path.with_extension("events.tsv"))?);
    let mut metrics = BufWriter::new(File::create(path.with_extension("metrics.tsv"))?);
    writeln!(metrics, "frame\trender_ms\tsubmitted\trasterized\tculled")?;
    let scene = Scene::new()?;
    let mut g = software_setup();
    let controls = Controls::new();
    let mut r = Renderer::new(width, height);
    let mut hud = Renderer::new(640, 360);
    for frame in 0..count {
        for sub in 0..4 {
            g.tick(software_input(frame * 4 + sub), 1. / 120.);
            if frame >= start_frame {
                for cue in &g.events {
                    writeln!(
                        cues,
                        "{:.6}\t{}",
                        ((frame - start_frame) * 4 + sub) as f32 / 120.,
                        cue.file()
                    )?;
                }
            }
            g.events.clear();
        }
        if frame < start_frame {
            continue;
        }
        let timer = std::time::Instant::now();
        scene.render(&mut r, &g);
        r.engine_glow();
        ui::compose(&mut r, &mut hud, &g, &controls, false, false, 30.);
        let stage = match frame / FPS {
            0..=1 => "VUELO / SISTEMA SOLAR",
            2..=6 => "ACERCAMIENTO CONTINUO",
            7..=19 => "VUELTA COMPLETA / CONTROL ORBITAL LOCAL",
            _ => "ALEJAMIENTO CONTINUO",
        };
        let top = height as i32 - 43;
        r.rect(0, top, width as i32, 43, [9, 35, 105]);
        ui::text(&mut r, 12, top + 6, stage, 1, [255, 233, 118]);
        ui::text(
            &mut r,
            12,
            top + 19,
            "RASTERIZACION CPU PROPIA / SIN CORTES / SIN SHADERS GPU",
            1,
            [222, 246, 255],
        );
        ui::text(
            &mut r,
            12,
            top + 31,
            "ENTRADAS AUTOMATICAS / 30 FPS DEL ARCHIVO, NO BENCHMARK",
            1,
            [185, 225, 255],
        );
        writeln!(
            metrics,
            "{frame}\t{:.3}\t{}\t{}\t{}",
            timer.elapsed().as_secs_f64() * 1000.,
            r.stats.submitted,
            r.stats.triangles,
            r.stats.culled
        )?;
        if [1, 6, 12, 19, 29].contains(&(frame / FPS)) && frame % FPS == 0 {
            r.save(&path.with_file_name(format!("cpu-flight-{:02}.png", frame / FPS)))?;
        }
        pipe.write_all(&r.pixels)?;
        if frame % (FPS * 5) == 0 {
            println!("Video CPU: {frame}/{count}");
        }
    }
    pipe.flush()?;
    drop(pipe);
    cues.flush()?;
    metrics.flush()?;
    let status = encoder.0.as_mut().unwrap().wait()?;
    encoder.0 = None;
    if !status.success() {
        return Err("FFmpeg no pudo finalizar el video CPU".into());
    }
    println!(
        "Video CPU: {} / {}x{} / {:.1} s",
        path.display(),
        width,
        height,
        (count - start_frame) as f32 / FPS as f32
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn academic_flight_approaches_orbits_fully_and_leaves_without_cuts() {
        let mut g = software_setup();
        let mut closest = f32::MAX;
        let mut turn = 0.;
        for step in 0..SOFTWARE_SECONDS * 120 {
            let before = g.ship;
            let rel = g.ship - g.planet_at(WorldId::Garden);
            g.tick(software_input(step), 1. / 120.);
            assert_eq!(g.mode, Mode::Flight);
            assert!((g.ship - before).length() < 0.2, "no teleportation");
            let after = g.ship - g.planet_at(WorldId::Garden);
            closest = closest.min(after.length());
            if g.local_orbit {
                let delta = after.x.atan2(after.z) - rel.x.atan2(rel.z);
                turn += (delta + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU)
                    - std::f32::consts::PI;
            }
            g.events.clear();
        }
        assert!(closest < 22.);
        assert!(turn > std::f32::consts::TAU);
        assert!(!g.local_orbit);
        assert!((g.ship - g.planet_at(WorldId::Garden)).length() > 45.);
        assert_eq!(g.progress, crate::progress::Progress::default());
    }
    #[test]
    fn recorded_gameplay_uses_real_inputs_and_stays_playable() {
        for segment in [1, 6, 7, 8, 9, 11] {
            let mut g = Game::new();
            setup(&mut g, segment);
            let seconds = if segment == 11 {
                SECONDS - CUTS[segment]
            } else {
                CUTS[segment + 1] - CUTS[segment]
            };
            let mut moving = false;
            for step in 0..seconds * 120 {
                let i = demo_input(&g, segment, step);
                g.tick(i, 1. / 120.);
                moving |= g.velocity.length() > 0.1;
                g.events.clear();
            }
            assert!(g.camera_eye.x.is_finite());
            assert_ne!(g.mode, Mode::Defeat, "demo segment {segment}");
            if segment == 1 {
                assert!(moving);
                assert_eq!(g.mode, Mode::Flight);
            }
            if segment == 7 {
                assert!(g.duel.enemy_hp < 6);
                assert_eq!(g.duel.player_hp, 5);
            }
            if segment == 8 {
                assert!(g.runner.distance > 100.);
                assert_eq!(g.runner.health, 3);
            }
            if segment == 9 {
                assert!(g.platformer.x > 50.);
            }
            if segment == 11 {
                assert_eq!(
                    g.mode,
                    Mode::Victory,
                    "race {} m / {} s",
                    g.race.distance,
                    g.race.time
                );
                assert!(g.progress.medals[WorldId::Race.index()]);
            }
        }
    }
}
