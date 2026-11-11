mod audio;
mod capture;
mod dojo_scene;
mod duel;
mod feedback;
mod game;
mod gpu;
mod gravity;
mod haptics;
mod input;
mod math;
mod normal_map;
mod obj;
mod platform_scene;
mod platformer;
mod progress;
mod race;
mod race_scene;
mod renderer;
mod runner;
mod runner_scene;
mod scene;
mod ui;
mod world;
use game::{Game, Mode, WorldId};
use input::{Controls, Input};
use raylib::prelude::*;
use renderer::Renderer;
use scene::Scene;
use std::{error::Error, path::PathBuf};
const SOFTWARE_RESOLUTIONS: [(usize, usize); 3] = [(800, 450), (960, 540), (640, 360)];
const RESOLUTIONS: [(usize, usize); 3] = [(1920, 1080), (2560, 1440), (1280, 720)];
fn assets() -> PathBuf {
    let bundled = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("../Resources/assets")));
    bundled
        .filter(|p| p.exists())
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets"))
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut export = None;
    let mut recording = None;
    let mut record_start = 0;
    let mut gpu_export = false;
    let mut software = false;
    let mut normal_mapping = true;
    let mut gravity_enabled = true;
    let mut view = "flight".to_string();
    let mut frames = None;
    let mut benchmark = false;
    let mut save_path = None;
    let mut quality = 0usize;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--record-start" => {
                record_start = args
                    .next()
                    .ok_or("Falta cuadro inicial")?
                    .parse::<usize>()?
            }
            "--record" => recording = Some(PathBuf::from(args.next().ok_or("Falta ruta MP4")?)),
            "--software" => software = true,
            "--no-normal-map" => normal_mapping = false,
            "--no-gravity" => gravity_enabled = false,
            "--gpu-render" => {
                gpu_export = true;
                export = Some(PathBuf::from(args.next().ok_or("Falta ruta PNG")?));
            }
            "--render" => export = Some(PathBuf::from(args.next().ok_or("Falta ruta PNG")?)),
            "--scene" => view = args.next().ok_or("Falta escena")?,
            "--frames" => frames = Some(args.next().ok_or("Falta numero")?.parse::<usize>()?),
            "--save" => {
                save_path = Some(PathBuf::from(args.next().ok_or("Falta ruta de guardado")?))
            }
            "--quality" => {
                quality = match args.next().as_deref() {
                    Some("balanced") => 0,
                    Some("high") => 1,
                    Some("fast") => 2,
                    _ => return Err("Calidad: balanced, high o fast".into()),
                }
            }
            "--benchmark" => benchmark = true,
            "--help" | "-h" => {
                println!(
                    "INSERT COIN: GALAXY / CINCO MUNDOS\ncargo run --release --locked --offline\n--render salida.png --scene flight|mario|mario-mid|mario-end|mario-planet|runner|runner-jump|runner-slide|runner-planet|battle|garden-phase2|dojo|dojo-counter|dojo-planet|portal|system|title|victory|close|orbit|ship\n--record salida.mp4: recorrido automatico 176 s, sin audio; --frames limita duracion\n--scene race|race-start|race-loop|race-tails|race-knuckles|sonic-model|tails-model|knuckles-model|race-planet\n--gpu-render salida.png: captura GPU HD\n--software: rasterizador CPU original; con --record exporta vuelo continuo de 30 s\n--quality balanced|high|fast\n--no-normal-map / --no-gravity: comparacion de opcionales\n--benchmark: 120 cuadros de vista fija sin ventana\n--frames N: prueba de ventana\n--save ruta: archivo de progreso alternativo\nG/cruceta izquierda: gravedad, F6/cruceta derecha: normal mapping\nH: controles, N/R3: destino, M/Share: audio, F4/cruceta abajo: calidad, F5/cruceta arriba: vibracion"
                );
                return Ok(());
            }
            _ => return Err(format!("Argumento desconocido: {a}").into()),
        }
    }
    if ![
        "race",
        "race-start",
        "race-loop",
        "race-tails",
        "race-knuckles",
        "sonic-model",
        "tails-model",
        "knuckles-model",
        "race-planet",
        "flight",
        "battle",
        "garden-phase2",
        "dojo",
        "dojo-counter",
        "dojo-planet",
        "portal",
        "system",
        "title",
        "victory",
        "close",
        "orbit",
        "ship",
        "garden-model",
        "dojo-model",
        "runner-model",
        "mario-model",
        "runner",
        "runner-jump",
        "runner-slide",
        "runner-planet",
        "minion",
        "mario",
        "mario-mid",
        "mario-end",
        "mario-planet",
    ]
    .contains(&view.as_str())
    {
        return Err("Escena desconocida".into());
    }
    if let Some(path) = recording {
        if software {
            return capture::record_software(
                &path,
                frames,
                quality,
                record_start,
                normal_mapping,
                gravity_enabled,
            );
        }
        return capture::record(
            &path,
            frames,
            quality,
            record_start,
            normal_mapping,
            gravity_enabled,
        );
    }
    let mut scene = Scene::new()?;
    let mut game = Game::new();
    game.gravity_enabled = gravity_enabled;
    let resolutions = if software || (export.is_some() && !gpu_export) || benchmark {
        SOFTWARE_RESOLUTIONS
    } else {
        RESOLUTIONS
    };
    let (width, height) = resolutions[quality];
    let mut renderer = Renderer::new(width, height);
    renderer.normal_mapping = normal_mapping;
    let mut hud = Renderer::new(640, 360);
    let mut controls = Controls::new();
    if export.is_some() || benchmark || frames.is_some() {
        if view != "title" {
            game.mode = Mode::Flight;
        }
        if ["battle", "garden-phase2", "victory", "dojo", "dojo-counter"].contains(&view.as_str()) {
            if view.starts_with("dojo") {
                game.active_world = WorldId::Dojo;
            }
            game.enter();
            game.arrival = 0.;
            for _ in 0..180 {
                game.tick(Input::default(), 1. / 120.);
            }
            game.invulnerable = 0.;
            if view == "garden-phase2" {
                game.boss_hp = game::BOSS_MAX_HP / 2;
                for _ in 0..80 {
                    game.tick(Input::default(), 1. / 120.);
                }
            }
            if view == "dojo-counter" {
                game.duel.stage = duel::Stage::Recovery;
                game.duel.counter_open = true;
                game.duel.message = "BIEN! AHORA CONTRAATACA";
            }
            if view == "victory" {
                game.mode = Mode::Victory;
                game.boss_hp = 0;
                game.progress.award(0, 24.5);
            }
        }
        if ["runner", "runner-jump", "runner-slide"].contains(&view.as_str()) {
            game.active_world = WorldId::Runner;
            game.enter();
            game.arrival = 0.;
            for _ in 0..120 {
                game.tick(Input::default(), 1. / 120.);
            }
            if view == "runner-jump" {
                game.runner.jump = 0.48;
            }
            if view == "runner-slide" {
                game.runner.slide = 0.65;
            }
        }
        if ["mario", "mario-mid", "mario-end"].contains(&view.as_str()) {
            game.active_world = WorldId::Mario;
            game.enter();
            game.arrival = 0.;
            game.platformer.x = match view.as_str() {
                "mario-mid" => 93. * platformer::TILE,
                "mario-end" => 190. * platformer::TILE,
                _ => 18.,
            };
            game.platformer.camera = game.platformer.x + 5.;
            game.platformer.time = 1.2;
            if view == "mario-end" {
                game.platformer.big = true;
            }
        }
        if [
            "race",
            "race-start",
            "race-loop",
            "race-tails",
            "race-knuckles",
        ]
        .contains(&view.as_str())
        {
            game.active_world = WorldId::Race;
            game.enter();
            if view == "race-tails" {
                game.race.hero = race::Hero::Tails;
            }
            if view == "race-knuckles" {
                game.race.hero = race::Hero::Knuckles;
            }
            if view != "race" {
                game.race.phase = race::Phase::Running;
                game.race.distance = if view == "race-loop" {
                    race::LOOP_START + 18.
                } else {
                    62.
                };
                game.race.speed = 32.;
                game.race.rings = 8;
                if view == "race-tails" || view == "race-knuckles" {
                    game.race.ability = 2.;
                    game.race.y = 3.;
                }
                let (eye, target) = race_scene::camera(&game.race);
                game.camera_eye = eye;
                game.camera_target = target;
            }
        }
        if view == "race-planet" {
            game.active_world = WorldId::Race;
            game.ship = game.world_at(WorldId::Race, math::Vec3::new(0., 2., 16.));
            game.camera_eye = game.ship + math::Vec3::new(0., 4., 9.);
            game.camera_target = game.planet_at(WorldId::Race);
        }
        if view == "mario-planet" {
            game.active_world = WorldId::Mario;
            game.ship = game.world_at(WorldId::Mario, math::Vec3::new(0., 2., 16.));
            game.camera_eye = game.ship + math::Vec3::new(0., 4., 9.);
            game.camera_target = game.planet_at(WorldId::Mario);
        }
        if view == "runner-planet" {
            game.active_world = WorldId::Runner;
            game.ship = game.world_at(WorldId::Runner, math::Vec3::new(0., 2., 16.));
            game.camera_eye = game.ship + math::Vec3::new(0., 4., 9.);
            game.camera_target = game.planet_at(WorldId::Runner);
        }
        if view == "close" {
            game.ship = game.world(math::Vec3::new(0., 1., 9.));
            game.camera_eye = game.world(math::Vec3::new(0., 2., 13.));
            game.camera_target = game.planet();
        }
        if view == "orbit" {
            game.ship = game.world(math::Vec3::new(14., 3., 0.));
            game.camera_eye = game.world(math::Vec3::new(20., 6., 3.));
            game.camera_target = game.planet();
        }
        if view == "dojo-planet" {
            game.active_world = WorldId::Dojo;
            game.ship = game.world_at(WorldId::Dojo, math::Vec3::new(0., 2., 16.));
            game.camera_eye = game.ship + math::Vec3::new(0., 4., 9.);
            game.camera_target = game.planet_at(WorldId::Dojo);
        }
        if view == "system" {
            game.camera_eye = math::Vec3::new(60., 43., 67.);
            game.camera_target = math::Vec3::new(0., 0., -7.);
        }
        if view == "portal" {
            game.tick(
                Input {
                    accept: true,
                    ..Input::default()
                },
                1. / 120.,
            );
            for _ in 0..95 {
                game.tick(Input::default(), 1. / 120.);
            }
        }
        if !gpu_export && (export.is_some() || benchmark) {
            let count = if benchmark { 120 } else { 1 };
            let mut timings = vec![];
            let started = std::time::Instant::now();
            for _ in 0..count {
                let t = std::time::Instant::now();
                if view == "ship" || view.ends_with("-model") {
                    scene.preview_asset(&mut renderer, &view);
                } else if view == "minion" {
                    renderer.begin(
                        math::Vec3::new(3.2, 2.5, 5.),
                        math::Vec3::new(0., 1.1, 0.),
                        0.,
                    );
                    runner_scene::minion(
                        &scene,
                        &mut renderer,
                        math::Vec3::default(),
                        1.,
                        0.,
                        0.,
                        false,
                    );
                } else {
                    scene.render(&mut renderer, &game);
                    if !(game.inside() && game.active_world == WorldId::Runner) {
                        renderer.engine_glow();
                    }
                    ui::compose(&mut renderer, &mut hud, &game, &controls, false, false, 60.);
                }
                timings.push(t.elapsed().as_secs_f64() * 1000.);
            }
            if benchmark {
                timings.sort_by(|a, b| a.total_cmp(b));
                println!(
                    "CPU {}x{} / {} / vista fija: {:.1} FPS medios; p95 {:.1} ms; {} triangulos enviados, {} rasterizados, {} objetos descartados",
                    width,
                    height,
                    view,
                    count as f64 / started.elapsed().as_secs_f64(),
                    timings[(count as f32 * 0.95) as usize],
                    renderer.stats.submitted,
                    renderer.stats.triangles,
                    renderer.stats.culled
                );
            }
            if let Some(path) = export {
                if let Some(p) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
                    std::fs::create_dir_all(p)?;
                }
                renderer.save(&path)?;
                println!("PNG: {}", path.display());
            }
            return Ok(());
        }
    }
    renderer.gpu = !software;
    let save_path = save_path
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("saves/progress.txt"));
    let writable = match if gpu_export {
        Ok(progress::Progress::default())
    } else {
        progress::load(&save_path)
    } {
        Ok(p) => {
            game.progress = p;
            game.medal = game.progress.medals[0];
            !gpu_export && frames.is_none()
        }
        Err(error) => {
            eprintln!("Guardado: {error}. Se conserva el archivo original.");
            game.save_error = Some(error);
            false
        }
    };
    let mut attempted = game.progress.clone();
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .resizable()
        .highdpi()
        .title("INSERT COIN: GALAXY | Green Hill / Sonic Grand Prix")
        .build();
    rl.set_target_fps(60);
    rl.set_exit_key(None);
    let device = if gpu_export {
        None
    } else {
        match RaylibAudio::init_audio_device() {
            Ok(a) => Some(a),
            Err(e) => {
                eprintln!("Audio no disponible: {e}");
                None
            }
        }
    };
    let sound = device
        .as_ref()
        .and_then(|d| match audio::Audio::new(d, &assets().join("audio")) {
            Ok(a) => Some(a),
            Err(e) => {
                eprintln!("Audio: {e}");
                None
            }
        });
    game.audio_available = sound.is_some();
    let mut haptics = haptics::Haptics::default();
    let mut texture = rl.load_texture_from_image(
        &thread,
        &Image::gen_image_color(width as i32, height as i32, Color::BLACK),
    )?;
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    let mut gpu = if software {
        None
    } else {
        Some(gpu::Gpu::new(&mut rl, &thread, width, height, &assets())?)
    };
    let mut debug = false;
    let mut help = false;
    let mut frame = 0;
    let mut frame_samples = Vec::new();
    let mut accumulator = 0.;
    let mut pending = Input::default();
    let mut pad_previous = false;
    let mut had_focus = true;
    while !rl.window_should_close() && !rl.is_key_pressed(KeyboardKey::KEY_F10) {
        let was_paused = game.paused;
        let focused = gpu_export || frames.is_some() || rl.is_window_focused();
        let mut command = controls.read(&rl);
        let mut redraw = !game.paused;
        if ((!focused && had_focus) || (pad_previous && controls.gamepad.is_none()))
            && game.mode != Mode::Title
        {
            game.paused = true;
        }
        had_focus = focused;
        pad_previous = controls.gamepad.is_some();
        if rl.is_key_pressed(KeyboardKey::KEY_H) {
            help = !help;
            if help && game.mode != Mode::Title {
                game.paused = true;
            }
            redraw = true;
        }
        if command.normal_mapping {
            renderer.normal_mapping = !renderer.normal_mapping;
            redraw = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F1) {
            debug = !debug;
            redraw = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F2) {
            renderer.culling = !renderer.culling;
            redraw = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F3) {
            scene.lod = !scene.lod;
            redraw = true;
        }
        if command.quality {
            quality = (quality + 1) % 3;
            let (w, h) = resolutions[quality];
            let culling = renderer.culling;
            let normal_mapping = renderer.normal_mapping;
            renderer = Renderer::new(w, h);
            renderer.culling = culling;
            renderer.normal_mapping = normal_mapping;
            renderer.gpu = !software;
            if let Some(gpu) = &mut gpu {
                gpu.resize(&mut rl, &thread, w, h)?;
            }
            texture = rl.load_texture_from_image(
                &thread,
                &Image::gen_image_color(w as i32, h as i32, Color::BLACK),
            )?;
            redraw = true;
        }
        redraw |= command.mute || command.vibration;
        if !focused {
            command = Input::default();
        }
        if help {
            command.pause = false;
            command.accept = false;
        }
        pending.merge_edges(command);
        command.merge_edges(pending);
        if !gpu_export {
            accumulator += rl.get_frame_time().min(0.25);
        }
        while accumulator >= 1. / 120. {
            game.tick(command, 1. / 120.);
            command.clear_edges();
            pending.clear_edges();
            accumulator -= 1. / 120.;
        }
        if game.progress != attempted {
            attempted = game.progress.clone();
            if writable {
                match progress::save(&save_path, &game.progress) {
                    Ok(()) => {
                        game.save_error = None;
                        println!("Progreso guardado: {}", save_path.display());
                    }
                    Err(e) => {
                        eprintln!("No se pudo guardar: {e}");
                        game.save_error = Some(e.to_string());
                    }
                }
            }
        }
        if let Some(sound) = &sound {
            sound.update(&game, &game.events);
        }
        let previous_haptics = game.vibration_available;
        haptics.update(
            controls.gamepad.is_some(),
            game.vibration_enabled,
            game.paused || !focused,
            rl.get_frame_time().min(0.25),
            &game.events,
        );
        game.vibration_available = haptics.available;
        game.vibration_pulses = haptics.delivered;
        if previous_haptics != game.vibration_available {
            println!(
                "Vibracion nativa: {}",
                if haptics.available {
                    "disponible"
                } else {
                    "no disponible"
                }
            );
        }
        redraw |= previous_haptics != game.vibration_available;
        game.events.clear();
        redraw |= !game.paused || was_paused != game.paused || frame == 0;
        let screenshot = rl.is_key_pressed(KeyboardKey::KEY_F12);
        redraw |= screenshot;
        if redraw {
            let asset_preview =
                (gpu_export || frames.is_some()) && (view == "ship" || view.ends_with("-model"));
            if asset_preview {
                scene.preview_asset(&mut renderer, &view);
            } else {
                scene.render(&mut renderer, &game);
            }
            if !(game.inside() && game.active_world == WorldId::Runner) {
                renderer.engine_glow();
            }
            if !asset_preview {
                ui::compose(
                    &mut renderer,
                    &mut hud,
                    &game,
                    &controls,
                    debug,
                    help,
                    rl.get_fps() as f32,
                );
            } else {
                hud.background.clear();
                hud.overlay.clear();
            }
            if let Some(gpu) = &mut gpu {
                gpu.render(&mut rl, &thread, &renderer, &hud, &game);
            } else {
                texture.update_texture(&renderer.pixels)?;
            }
        }
        if screenshot || gpu_export {
            let path = if gpu_export {
                export.clone().unwrap()
            } else {
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("output/captura.png")
            };
            if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
                std::fs::create_dir_all(parent)?;
            }
            if let Some(gpu) = &gpu {
                gpu.save(&path)?;
            } else {
                renderer.save(&path)?;
            }
            println!("PNG: {}", path.display());
        }
        let sw = rl.get_screen_width() as f32;
        let sh = rl.get_screen_height() as f32;
        let width = renderer.width as f32;
        let height = renderer.height as f32;
        let scale = (sw / width).min(sh / height);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let destination = Rectangle::new(
            (sw - width * scale) * 0.5,
            (sh - height * scale) * 0.5,
            width * scale,
            height * scale,
        );
        if let Some(gpu) = &mut gpu {
            gpu.present(&mut d, destination);
        } else {
            d.draw_texture_pro(
                &texture,
                Rectangle::new(0., 0., width, height),
                destination,
                Vector2::zero(),
                0.,
                Color::WHITE,
            );
        }
        drop(d);
        frame += 1;
        if frames.is_some() && frame > 5 {
            frame_samples.push(rl.get_frame_time() * 1000.);
        }
        if gpu_export || frames.is_some_and(|limit| frame >= limit) {
            break;
        }
    }
    haptics.stop();
    if !frame_samples.is_empty() {
        frame_samples.sort_by(|a, b| a.total_cmp(b));
        let mean = frame_samples.iter().sum::<f32>() / frame_samples.len() as f32;
        println!(
            "Medicion {}x{}: {:.1} FPS, p95 {:.1} ms (ventana limitada a 60 FPS)",
            renderer.width,
            renderer.height,
            1000. / mean,
            frame_samples[frame_samples.len() * 95 / 100]
        );
    }
    if game.inside() && game.active_world == WorldId::Runner {
        game.progress.runner_score = game.progress.runner_score.max(game.runner.score());
        if writable
            && game.progress != attempted
            && let Err(error) = progress::save(&save_path, &game.progress)
        {
            eprintln!("No se pudo guardar al salir: {error}");
        }
    }
    println!(
        "Salida normal. Mando: {}",
        if controls.name.is_empty() {
            "ninguno"
        } else {
            &controls.name
        }
    );
    Ok(())
}
