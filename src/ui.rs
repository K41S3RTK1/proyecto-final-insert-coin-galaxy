use crate::{
    game::{Game, Mode, WorldId},
    input::Controls,
    renderer::Renderer,
};
const CREAM: [u8; 3] = [255, 233, 118];
const MINT: [u8; 3] = [157, 246, 255];
const INK: [u8; 3] = [12, 43, 132];
const MUTED: [u8; 3] = [196, 225, 255];
const RED: [u8; 3] = [255, 116, 58];
fn glyph(c: char) -> [u8; 7] {
    match c {
        'A' => [14, 17, 17, 31, 17, 17, 17],
        'B' => [30, 17, 17, 30, 17, 17, 30],
        'C' => [14, 17, 16, 16, 16, 17, 14],
        'D' => [30, 17, 17, 17, 17, 17, 30],
        'E' => [31, 16, 16, 30, 16, 16, 31],
        'F' => [31, 16, 16, 30, 16, 16, 16],
        'G' => [14, 17, 16, 23, 17, 17, 15],
        'H' => [17, 17, 17, 31, 17, 17, 17],
        'I' => [31, 4, 4, 4, 4, 4, 31],
        'J' => [7, 2, 2, 2, 18, 18, 12],
        'K' => [17, 18, 20, 24, 20, 18, 17],
        'L' => [16, 16, 16, 16, 16, 16, 31],
        'M' => [17, 27, 21, 21, 17, 17, 17],
        'N' => [17, 25, 25, 21, 19, 19, 17],
        'O' => [14, 17, 17, 17, 17, 17, 14],
        'P' => [30, 17, 17, 30, 16, 16, 16],
        'Q' => [14, 17, 17, 17, 21, 18, 13],
        'R' => [30, 17, 17, 30, 20, 18, 17],
        'S' => [15, 16, 16, 14, 1, 1, 30],
        'T' => [31, 4, 4, 4, 4, 4, 4],
        'U' => [17, 17, 17, 17, 17, 17, 14],
        'V' => [17, 17, 17, 17, 17, 10, 4],
        'W' => [17, 17, 17, 21, 21, 27, 17],
        'X' => [17, 17, 10, 4, 10, 17, 17],
        'Y' => [17, 17, 10, 4, 4, 4, 4],
        'Z' => [31, 1, 2, 4, 8, 16, 31],
        '0' => [14, 17, 19, 21, 25, 17, 14],
        '1' => [4, 12, 4, 4, 4, 4, 14],
        '2' => [14, 17, 1, 2, 4, 8, 31],
        '3' => [30, 1, 1, 14, 1, 1, 30],
        '4' => [2, 6, 10, 18, 31, 2, 2],
        '5' => [31, 16, 16, 30, 1, 1, 30],
        '6' => [14, 16, 16, 30, 17, 17, 14],
        '7' => [31, 1, 2, 4, 8, 8, 8],
        '8' => [14, 17, 17, 14, 17, 17, 14],
        '9' => [14, 17, 17, 15, 1, 1, 14],
        '-' => [0, 0, 0, 31, 0, 0, 0],
        '/' => [1, 1, 2, 4, 8, 16, 16],
        ':' => [0, 4, 4, 0, 4, 4, 0],
        '.' => [0, 0, 0, 0, 0, 6, 6],
        '[' => [14, 8, 8, 8, 8, 8, 14],
        ']' => [14, 2, 2, 2, 2, 2, 14],
        '+' => [0, 4, 4, 31, 4, 4, 0],
        '!' => [4, 4, 4, 4, 4, 0, 4],
        '?' => [14, 17, 1, 2, 4, 0, 4],
        '<' => [2, 4, 8, 16, 8, 4, 2],
        '>' => [8, 4, 2, 1, 2, 4, 8],
        '=' => [0, 0, 31, 0, 31, 0, 0],
        _ => [0; 7],
    }
}
pub fn text(r: &mut Renderer, x: i32, y: i32, s: &str, size: i32, color: [u8; 3]) {
    if r.gpu {
        r.paint(crate::renderer::Paint::Text(
            x,
            y,
            s.to_string(),
            size,
            color,
        ));
        return;
    }
    for (i, c) in s.to_uppercase().chars().enumerate() {
        for (row, bits) in glyph(c).iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    r.rect(
                        x + i as i32 * 6 * size + col * size,
                        y + row as i32 * size,
                        size,
                        size,
                        color,
                    );
                }
            }
        }
    }
}
fn center(r: &mut Renderer, y: i32, s: &str, size: i32, color: [u8; 3]) {
    let x = (r.width as i32 - s.chars().count() as i32 * 6 * size) / 2;
    text(r, x, y, s, size, color);
}
fn panel(r: &mut Renderer, x: i32, y: i32, w: i32, h: i32) {
    r.rect(x, y, w, h, INK);
    r.rect(x, y, w, 1, CREAM);
    r.rect(x, y + h - 1, w, 1, CREAM);
    r.rect(x, y, 1, h, CREAM);
    r.rect(x + w - 1, y, 1, h, CREAM);
}
pub fn draw(r: &mut Renderer, g: &Game, controls: &Controls, debug: bool, fps: f32) {
    let w = r.width as i32;
    let h = r.height as i32;
    let pad = controls.using_pad;
    if !(g.inside() && matches!(g.active_world, WorldId::Mario | WorldId::Race)) {
        r.rect(0, 0, w, 45, INK);
        r.rect(0, 44, w, 1, [78, 77, 73]);
        r.rect(0, h - 42, w, 42, INK);
        r.rect(0, h - 42, w, 1, [78, 77, 73]);
        text(r, 14, 10, "INSERT COIN", 2, CREAM);
        text(r, 15, 29, "G A L A X Y   /   GREEN HILL", 1, MINT);
        text(
            r,
            w - 212,
            11,
            &format!(
                "0{} / {}",
                g.active_world.index() + 1,
                g.active_world.name()
            ),
            1,
            CREAM,
        );
        text(
            r,
            w - 212,
            28,
            if controls.gamepad.is_some() {
                "MANDO CONECTADO"
            } else {
                "TECLADO"
            },
            1,
            MUTED,
        );
        text(
            r,
            w - 100,
            28,
            if g.audio_muted {
                "AUDIO OFF"
            } else if g.audio_available {
                "AUDIO ON"
            } else {
                "SIN AUDIO"
            },
            1,
            MUTED,
        );
        text(
            r,
            287,
            28,
            if !g.vibration_enabled {
                "VIB OFF"
            } else if g.vibration_available {
                "VIB ON"
            } else {
                "VIB N/D"
            },
            1,
            MUTED,
        );
    }
    let collected = g.progress.medals.iter().filter(|v| **v).count();
    match g.mode {
        Mode::Title => {
            panel(r, 100, 78, 440, 216);
            center(r, 94, "UNA MONEDA. CINCO UNIVERSOS.", 1, MINT);
            center(r, 118, "INSERT COIN", 4, CREAM);
            center(r, 156, "G A L A X Y", 2, RED);
            center(r, 180, "01 INKWELL GARDEN   /   02 NEON DOJO", 1, CREAM);
            center(
                r,
                194,
                "03 BANANA RUSH   /   04 MUSHROOM KINGDOM",
                1,
                [255, 210, 66],
            );
            center(
                r,
                222,
                &format!("INSIGNIAS GUARDADAS {collected}/5"),
                1,
                MINT,
            );
            center(r, 207, "05 SONIC GRAND PRIX / TRES PERSONAJES", 1, MINT);
            r.rect(165, 237, 310, 31, [62, 92, 84]);
            center(
                r,
                248,
                if pad {
                    "[X] DESPEGAR"
                } else {
                    "[ENTER] DESPEGAR"
                },
                1,
                CREAM,
            );
            text(
                r,
                15,
                h - 28,
                "PILOTA / EXPLORA / DESAFIA / GUARDA TU PROGRESO",
                1,
                CREAM,
            );
            text(
                r,
                15,
                h - 14,
                if pad {
                    "R3: DESTINO   SHARE: AUDIO   CRUCETA ABAJO: CALIDAD"
                } else {
                    "N: DESTINO   M: AUDIO   F4: CALIDAD   H: CONTROLES"
                },
                1,
                MUTED,
            );
        }
        Mode::Flight | Mode::Entering => {
            text(
                r,
                15,
                58,
                if g.mode == Mode::Entering {
                    "ENTRANDO AL MUNDO"
                } else {
                    "VUELO LIBRE"
                },
                1,
                MINT,
            );
            text(
                r,
                15,
                74,
                &format!("PUERTA {:05.1} M", g.gate_distance()),
                1,
                CREAM,
            );
            text(
                r,
                15,
                90,
                &format!("VELOCIDAD {:04.1}", g.velocity.length()),
                1,
                MUTED,
            );
            text(
                r,
                15,
                106,
                if g.local_orbit {
                    "ORBITA ASISTIDA: ON"
                } else {
                    "CAMARA DE SEGUIMIENTO"
                },
                1,
                MUTED,
            );
            text(r, 15, 123, &format!("INSIGNIAS {collected}/5"), 1, MINT);
            text(
                r,
                15,
                139,
                if pad {
                    "R3: CAMBIAR DESTINO"
                } else {
                    "N: CAMBIAR DESTINO"
                },
                1,
                CREAM,
            );
            text(
                r,
                15,
                157,
                &format!(
                    "GRAVEDAD: {}  {:.2} M/S2",
                    if g.local_orbit {
                        "AUTO"
                    } else if g.gravity_enabled {
                        "ON"
                    } else {
                        "OFF"
                    },
                    g.gravity_acceleration.length()
                ),
                1,
                MUTED,
            );
            // Compact map in world XZ coordinates; selecting a destination never teleports.
            panel(r, w - 113, 58, 99, 88);
            let cx = w - 64;
            let cy = 100;
            r.rect(cx - 2, cy - 2, 5, 5, [245, 190, 94]);
            for id in WorldId::ALL {
                let p = g.planet_at(id);
                let x = cx + (p.x * 0.60) as i32;
                let y = cy + (p.z * 0.60) as i32;
                let c = id.color();
                r.rect(x - 2, y - 2, 5, 5, c);
                text(r, x + 4, y - 3, &format!("{}", id.index() + 1), 1, c);
            }
            let sx = cx + (g.ship.x * 0.60).clamp(-43., 43.) as i32;
            let sy = cy + (g.ship.z * 0.60).clamp(-36., 36.) as i32;
            r.rect(sx - 1, sy - 1, 3, 3, CREAM);
            if g.mode == Mode::Entering {
                panel(r, 184, h - 85, 272, 31);
                center(r, h - 74, "INSERT COIN / CRUZANDO PORTAL", 1, CREAM);
            } else if g.can_enter() {
                panel(r, 184, h - 85, 272, 31);
                center(
                    r,
                    h - 74,
                    if pad {
                        "[X] INSERT COIN"
                    } else {
                        "[E] INSERT COIN"
                    },
                    1,
                    CREAM,
                );
            } else {
                let bearing = g.destination_bearing();
                let guide = if bearing > 0.18 {
                    "DESTINO A LA DERECHA >"
                } else if bearing < -0.18 {
                    "< DESTINO A LA IZQUIERDA"
                } else {
                    "DESTINO AL FRENTE / ACERCATE"
                };
                center(r, h - 65, guide, 1, MINT);
            }
            text(
                r,
                15,
                h - 28,
                if pad {
                    "L: VOLAR  R: MIRAR  L1/R1: ALTURA  L2: FRENO  L3: TURBO"
                } else {
                    "WASD: VOLAR  FLECHAS: MIRAR  R/F: ALTURA  CTRL: FRENO"
                },
                1,
                CREAM,
            );
            text(
                r,
                15,
                h - 14,
                if pad {
                    "TRIANGULO: ORBITA  X: ENTRAR  R3: DESTINO  OPTIONS: PAUSA"
                } else {
                    "TAB: ORBITA  SHIFT: TURBO  E: ENTRAR  N: DESTINO  H: AYUDA"
                },
                1,
                MUTED,
            );
        }
        Mode::Battle | Mode::Victory | Mode::Defeat => {
            if g.active_world == WorldId::Race {
                crate::race_scene::hud(r, g, controls);
            } else if g.active_world == WorldId::Garden {
                panel(r, 9, 51, 193, 51);
                panel(r, w - 108, 51, 96, 20);
                text(
                    r,
                    15,
                    57,
                    if g.phase_two {
                        "GRINNING MOON / FASE 2"
                    } else {
                        "GRINNING MOON / FASE 1"
                    },
                    1,
                    CREAM,
                );
                r.rect(15, 71, 180, 7, [63, 43, 46]);
                r.rect(
                    15,
                    71,
                    180 * g.boss_hp.max(0) / crate::game::BOSS_MAX_HP,
                    7,
                    if g.phase_two { [186, 116, 186] } else { RED },
                );
                text(
                    r,
                    15,
                    87,
                    &format!("VIDAS {} / 3", g.health.max(0)),
                    1,
                    MINT,
                );
                text(
                    r,
                    w - 100,
                    58,
                    &format!("{:02.0} SEG", (g.time_limit() - g.battle_time).max(0.)),
                    1,
                    CREAM,
                );
                if g.warning && g.mode == Mode::Battle {
                    panel(r, 175, 106, 290, 22);
                    center(r, 113, g.pattern.name(), 1, [248, 205, 125]);
                }
                if g.phase_notice > 0. && g.mode == Mode::Battle {
                    panel(r, 200, 133, 240, 22);
                    center(r, 140, "FASE 2 / LA FLOR DESPIERTA", 1, CREAM);
                }
                text(
                    r,
                    15,
                    h - 28,
                    if pad {
                        "STICK L: ESQUIVAR   R2 / CUADRADO: DISPARAR"
                    } else {
                        "WASD: ESQUIVAR   ESPACIO / J: DISPARAR"
                    },
                    1,
                    CREAM,
                );
            } else if g.active_world == WorldId::Mario {
                let p = &g.platformer;
                for (x, label, value) in [
                    (28, "MARIO", format!("{:06}", p.score)),
                    (194, "MONEDAS", format!("x {:02}", p.coins)),
                    (359, "MUNDO", "1 - 1".to_string()),
                    (
                        518,
                        "TIEMPO",
                        format!("{:03.0}", (g.time_limit() - g.battle_time).max(0.)),
                    ),
                ] {
                    text(r, x + 1, 19, label, 2, [41, 113, 163]);
                    text(r, x, 18, label, 2, CREAM);
                    text(r, x + 1, 42, &value, 2, [41, 113, 163]);
                    text(r, x, 41, &value, 2, CREAM);
                }
                text(
                    r,
                    28,
                    65,
                    &format!(
                        "VIDAS {}{}",
                        p.lives.max(0),
                        if p.big { " / SUPER MARIO" } else { "" }
                    ),
                    1,
                    CREAM,
                );
                if p.checkpoint > 3. {
                    text(r, 359, 65, "CHECKPOINT", 1, CREAM);
                }
                r.rect(0, h - 42, w, 42, INK);
                text(
                    r,
                    15,
                    h - 28,
                    if pad {
                        "STICK L: MOVER   X: SALTAR   R2: CORRER"
                    } else {
                        "A/D O FLECHAS: MOVER   ESPACIO: SALTAR   SHIFT: CORRER"
                    },
                    1,
                    CREAM,
                );
            } else if g.active_world == WorldId::Runner {
                let run = &g.runner;
                panel(r, 10, 51, 205, 45);
                panel(r, 225, 51, 190, 45);
                panel(r, 425, 51, 205, 45);
                text(
                    r,
                    18,
                    59,
                    &format!("DISTANCIA {:04.0} M", run.distance),
                    1,
                    CREAM,
                );
                text(
                    r,
                    18,
                    77,
                    if run.endless {
                        "MODO INFINITO"
                    } else {
                        "META: 900 M + 20 BANANAS"
                    },
                    1,
                    MINT,
                );
                text(
                    r,
                    236,
                    59,
                    &format!("BANANAS {:03} / VIDAS {}", run.coins, run.health.max(0)),
                    1,
                    [255, 210, 66],
                );
                text(r, 236, 77, &format!("PUNTOS {}", run.score()), 1, CREAM);
                text(r, 436, 59, "SKIN: MINION", 1, CREAM);
                text(
                    r,
                    436,
                    77,
                    &format!("RECORD {}", g.progress.runner_score.max(run.score())),
                    1,
                    MINT,
                );
                if !run.endless {
                    r.rect(12, 101, 616, 4, [46, 74, 89]);
                    r.rect(
                        12,
                        101,
                        (616. * (run.distance / crate::runner::GOAL).min(1.)) as i32,
                        4,
                        [255, 210, 66],
                    );
                }
                if g.mode == Mode::Battle && g.arrival <= 0. && g.battle_time < 8. {
                    panel(r, 140, 115, 360, 22);
                    center(r, 122, "SALTA BARRERAS / DESLIZATE BAJO ARCOS", 1, CREAM);
                }
                text(
                    r,
                    15,
                    h - 28,
                    if pad {
                        "L: CARRIL   X: SALTAR   L2: DESLIZAR"
                    } else {
                        "A/D O FLECHAS: CARRIL   ESPACIO/W: SALTO   S: DESLIZAR"
                    },
                    1,
                    CREAM,
                );
            } else {
                panel(r, 10, 51, 194, 46);
                panel(r, w - 204, 51, 194, 46);
                panel(r, 280, 51, 80, 22);
                text(r, 18, 58, "RIN / TU LUCHADOR", 1, MINT);
                text(r, w - 196, 58, "KAI / GUARDIAN DEL DOJO", 1, CREAM);
                r.rect(18, 76, 176, 8, [39, 61, 69]);
                r.rect(18, 76, 176 * g.duel.player_hp.max(0) / 5, 8, MINT);
                r.rect(w - 196, 76, 176, 8, [63, 43, 46]);
                r.rect(w - 196, 76, 176 * g.duel.enemy_hp.max(0) / 6, 8, RED);
                center(
                    r,
                    58,
                    &format!("{:02.0} SEG", (g.time_limit() - g.battle_time).max(0.)),
                    1,
                    CREAM,
                );
                if g.mode == Mode::Battle && g.arrival <= 0. {
                    panel(r, 175, 104, 290, 24);
                    center(
                        r,
                        112,
                        g.duel.message,
                        1,
                        if g.duel.counter_open { MINT } else { CREAM },
                    );
                }
                text(
                    r,
                    15,
                    h - 28,
                    if pad {
                        "L: MOVER   L2: BLOQUEAR   R1: ESQUIVAR   R2: GOLPEAR"
                    } else {
                        "A/D: MOVER   K/CTRL: BLOQUEAR   L/SHIFT: ESQUIVAR   J: GOLPE"
                    },
                    1,
                    CREAM,
                );
            }
            text(
                r,
                15,
                h - 14,
                if pad {
                    "CIRCULO: VOLVER   OPTIONS: PAUSA   SHARE: AUDIO"
                } else {
                    "Q: VOLVER   P: PAUSA   M: AUDIO   F4: CALIDAD   H: AYUDA"
                },
                1,
                MUTED,
            );
            if g.mode != Mode::Battle {
                panel(r, 130, 131, 380, 135);
                center(
                    r,
                    147,
                    if g.mode == Mode::Victory {
                        if matches!(
                            g.active_world,
                            WorldId::Runner | WorldId::Mario | WorldId::Race
                        ) {
                            "META SUPERADA!"
                        } else {
                            "KNOCKOUT!"
                        }
                    } else {
                        "TRY AGAIN"
                    },
                    3,
                    if g.mode == Mode::Victory { MINT } else { RED },
                );
                center(
                    r,
                    183,
                    if g.mode == Mode::Victory {
                        if collected == 5 {
                            "GALAXY CLEARED / 5 INSIGNIAS"
                        } else {
                            "NUEVA INSIGNIA / MUNDO COMPLETADO"
                        }
                    } else {
                        "LEE LA SENAL ANTES DE ACTUAR"
                    },
                    1,
                    CREAM,
                );
                center(
                    r,
                    202,
                    if g.save_error.is_some() {
                        "GUARDADO NO DISPONIBLE / REVISA LA CONSOLA"
                    } else if g.mode == Mode::Victory && g.active_world == WorldId::Runner {
                        if pad {
                            "R2 / CUADRADO: SEGUIR EN MODO INFINITO"
                        } else {
                            "J: SEGUIR EN MODO INFINITO"
                        }
                    } else if g.mode == Mode::Victory {
                        "PROGRESO GUARDADO"
                    } else {
                        "PUEDES REINTENTAR ESTA RONDA"
                    },
                    1,
                    MUTED,
                );
                center(
                    r,
                    226,
                    if g.mode == Mode::Victory {
                        if pad {
                            "X: VOLVER / R3: DESTINO EN VUELO"
                        } else {
                            "E: VOLVER / N: DESTINO EN VUELO"
                        }
                    } else if pad {
                        "X: REINTENTAR / CIRCULO: SALIR"
                    } else {
                        "E: REINTENTAR / Q: SALIR"
                    },
                    1,
                    MINT,
                );
                if g.mode == Mode::Victory
                    && let Some(ms) = g.progress.best_ms[g.active_world.index()]
                {
                    center(
                        r,
                        246,
                        &format!("MEJOR TIEMPO {:.2} S", ms as f32 / 1000.),
                        1,
                        MUTED,
                    );
                }
            }
        }
    }
    if debug {
        let s = r.stats;
        panel(r, w - 220, 153, 205, 110);
        for (i, line) in [
            format!(
                "FPS {fps:.0}  {} {}X{}",
                if r.gpu { "GPU" } else { "CPU" },
                s.width,
                s.height
            ),
            format!("TRI ENVIADOS {}", s.submitted),
            format!("TRI RASTER {}", s.triangles),
            format!("OBJ FUERA {}/{}", s.culled, s.objects),
            format!("VIB: {} PULSOS ACEPTADOS", g.vibration_pulses),
            format!(
                "F6 NORMAL MAP: {}",
                if r.normal_mapping { "ON" } else { "OFF" }
            ),
            "F2: CULLING F3: LOD F4: RES".into(),
        ]
        .iter()
        .enumerate()
        {
            text(r, w - 211, 163 + i as i32 * 13, line, 1, MINT);
        }
    }
    if g.save_error.is_some() && (g.mode == Mode::Title || g.mode == Mode::Flight) {
        center(r, h - 52, "AVISO: GUARDADO NO DISPONIBLE", 1, RED);
    }
    if g.paused {
        panel(r, 150, 111, 340, 154);
        center(r, 130, "PAUSA", 3, CREAM);
        center(r, 172, "P / OPTIONS: CONTINUAR", 1, MINT);
        center(r, 194, "H: CONTROLES   M / SHARE: AUDIO", 1, CREAM);
        center(r, 217, "F4 / CRUCETA ABAJO: CALIDAD", 1, MUTED);
        center(r, 234, "F5 / CRUCETA ARRIBA: VIBRACION", 1, MUTED);
        center(r, 250, "F10: CERRAR EL JUEGO", 1, MUTED);
    }
}
pub fn help(r: &mut Renderer) {
    panel(r, 50, 34, 540, 300);
    center(r, 45, "CONTROLES", 2, CREAM);
    let lines = [
        "ACCION                  TECLADO         PS5",
        "VOLAR / MOVER           WASD            STICK L",
        "ORIENTAR / ALTURA       FLECHAS / R-F   R / R1-L1",
        "DESTINO / ORBITA        N / TAB         R3 / TRIANGULO",
        "ENTRAR / VOLVER         E / Q           X / CIRCULO",
        "DISPARAR / GOLPEAR      ESPACIO-J       R2-CUADRADO",
        "BLOQUEAR (DOJO)         K-CTRL          L2",
        "ESQUIVAR (DOJO)         L-SHIFT         R1",
        "SALTAR                 ESPACIO-W       X",
        "DESLIZAR (RUNNER)       S-FLECHA ABAJO  L2",
        "VIBRACION ON/OFF        F5              CRUCETA ARRIBA",
        "CORRER MARIO / TURBO    SHIFT           R2 / L3",
        "BOOST (SONIC GP)       SHIFT           R2",
        "CARGAR/SOLTAR SPIN     K-CTRL          L2",
        "HABILIDAD (SONIC GP)   L               R1",
        "PAUSA / AUDIO           P / M           OPTIONS / SHARE",
        "CALIDAD                 F4              CRUCETA ABAJO",
        "GRAVEDAD                G               CRUCETA IZQ",
        "NORMAL MAPPING          F6              CRUCETA DER",
        "F1: METRICAS  F2: CULLING  F3: LOD  F12: CAPTURA",
        "HOME: RESCATE AL DESTINO   H: CERRAR ESTA AYUDA",
    ];
    for (i, line) in lines.iter().enumerate() {
        text(
            r,
            65,
            75 + i as i32 * 12,
            line,
            1,
            if i == 0 { MINT } else { CREAM },
        );
    }
}
#[allow(clippy::too_many_arguments)]
pub fn compose(
    out: &mut Renderer,
    hud: &mut Renderer,
    g: &Game,
    controls: &Controls,
    debug: bool,
    show_help: bool,
    fps: f32,
) {
    hud.gpu = out.gpu;
    hud.background.clear();
    hud.overlay.clear();
    hud.pixels.fill(0);
    hud.stats = out.stats;
    hud.normal_mapping = out.normal_mapping;
    let fade = if g.paused {
        0.
    } else if g.mode == Mode::Entering {
        ((0.32 - g.portal_left) / 0.32).clamp(0., 1.)
    } else if g.inside() {
        ((g.arrival - 2.8) / 0.6).clamp(0., 1.)
    } else {
        (g.arrival / 0.7).clamp(0., 1.)
    };
    if fade > 0. && !out.gpu {
        for p in out.pixels.chunks_exact_mut(4) {
            for k in 0..3 {
                p[k] = (p[k] as f32 * (1. - fade) + [233., 220., 183.][k] * fade) as u8;
            }
        }
    }
    draw(hud, g, controls, debug, fps);
    if g.inside() && g.arrival > 0. && !g.paused {
        panel(hud, 135, 115, 370, 144);
        center(
            hud,
            130,
            &format!("WORLD 0{}", g.active_world.index() + 1),
            2,
            MINT,
        );
        center(hud, 159, g.active_world.name(), 2, CREAM);
        center(
            hud,
            190,
            if g.active_world == WorldId::Garden {
                "GRINNING MOON / LEE SUS ATAQUES"
            } else if g.active_world == WorldId::Dojo {
                "RIN VS KAI / DEFIENDE Y CONTRAATACA"
            } else if g.active_world == WorldId::Mario {
                "MUNDO 1-1 / SALTA Y ALCANZA LA BANDERA"
            } else {
                "900 METROS / 20 BANANAS / TRES VIDAS"
            },
            1,
            CREAM,
        );
        center(
            hud,
            214,
            &format!("{}", g.arrival.ceil().min(3.) as i32),
            4,
            RED,
        );
    }
    let hurt = (g.active_world == WorldId::Mario && g.platformer.flash > 0.)
        || (g.active_world == WorldId::Runner && g.runner.flash > 0.)
        || g.hit_flash > 0.
        || (g.active_world == WorldId::Dojo
            && g.duel.hit_flash > 0.
            && g.duel.message == "GOLPE RECIBIDO");
    if hurt {
        hud.rect(0, 45, 3, 273, RED);
        hud.rect(637, 45, 3, 273, RED);
    }
    if show_help {
        help(hud);
    }
    if out.gpu {
        return;
    }
    for y in 0..out.height {
        let source_y = y * hud.height / out.height;
        for x in 0..out.width {
            let from = (source_y * hud.width + x * hud.width / out.width) * 4;
            if hud.pixels[from + 3] > 0 {
                let dest = (y * out.width + x) * 4;
                out.pixels[dest..dest + 4].copy_from_slice(&hud.pixels[from..from + 4]);
            }
        }
    }
}
