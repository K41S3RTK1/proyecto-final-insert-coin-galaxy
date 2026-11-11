# Entrega individual — INSERT COIN: GALAXY

**Integrante:** Daniel Sandoval. **Modalidad:** individual. **Fecha de revisión:** 2026-11-10.

Repositorio público: https://github.com/K41S3RTK1/proyecto-final-insert-coin-galaxy

## Contribuciones

Daniel Sandoval definió el concepto de mundos arcade conectados por una nave, eligió las referencias visuales, dirigió las iteraciones de diseño y reportó pruebas con teclado y mando PS5. El proyecto parte de trabajo previo del estudiante, incluido el cargador OBJ y operaciones vectoriales del Lab 4, y toma como referencia su nave del proyecto anterior.

La demostración individual debe explicar el pipeline y una contribución técnica concreta. Daniel debe comprender el código que entrega; las pruebas automatizadas no acreditan por sí solas esa comprensión.

Los modelos de Sonic, Tails, Knuckles, Mario y Minion son recreaciones del proyecto, no archivos oficiales descargados. Las referencias de videojuegos y la licencia de Nunito se detallan en README.md y assets/fonts/OFL.txt. No se atribuye propiedad de los personajes al estudiante.

## Qué ejecutar y presentar

1. Instalar Rust/Cargo, CMake y compilador C; en macOS, Xcode Command Line Tools.
2. Clonar el repositorio y entrar a la carpeta. `cargo run --release --locked -- --software --quality fast` ejecuta la modalidad académica; en macOS también `Evaluar-CPU.command`.
3. Enter despega, WASD mueve, flechas orientan, R/F cambian altura. Cerca del planeta, Tab activa asistencia orbital, A/D rodean y W/S cambian radio. Tab vuelve al vuelo libre. No usar Home en el tramo continuo de demostración.
4. Mostrar F1 (contadores), F2 (culling) y F3 (LOD) desde la misma cámara; explicar cómo cambia el trabajo del rasterizador.
5. Comparar normal mapping con F6 y gravedad en vuelo libre con G; explicar textura/TBN, aceleración y semilla Perlin según OPCIONALES.md.
6. Para la edición visual adicional, `Jugar.command` o `cargo run --release --locked`. Su modo GPU no es evidencia del requisito CPU.

## Archivos de entrega

- Código: src/, shaders/, platform/, Cargo.toml, Cargo.lock y build.rs.
- Recursos: assets/, incluidos todos los OBJ/MTL, audio y fuente tipográfica.
- Generadores reproducibles: tools/build_ship.py, build_planets.py, build_racers.py, asset_mesh.py, noise3d.py y build_normal_map.py.
- Video principal para la consigna: output/Proyecto_Final_CPU.mp4, 30 segundos, rasterización CPU, vuelo continuo con una vuelta completa. Controles automatizados identificados en pantalla; progreso aislado, sin editar partidas guardadas.
- Videos adicionales HD: output/Proyecto_Final_Green_Hill_Galaxy.mp4 y output/Sonic_Grand_Prix.mp4.
- Solicitud de evaluación y límites: RUBRICA.md. Controles: README.md y RACE.md.
- No se incluyen partidas personales, cachés de compilación, credenciales ni grabaciones intermedias.

## Preparación de la explicación individual

Seguir un vértice desde `Model::load` (obj.rs), transformación modelo/vista/proyección (math.rs y renderer.rs), clipping y baricéntricas hasta z-buffer y `fragment`. Explicar dt en `planet_at`/`spin_at`, cámara orbital y descarte/LOD. Ensayar cambiar en vivo la velocidad orbital de un planeta o la intensidad de un shader y explicar el resultado. La evaluación de esa explicación corresponde al docente.
