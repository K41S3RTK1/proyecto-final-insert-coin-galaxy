# Videos de entrega — modo CPU y ampliación HD

## Evidencia principal de la consigna: CPU

`output/Proyecto_Final_CPU.mp4`: **30 segundos**, 800×450 nativos, 30 fps, H.264 y audio AAC. Se rasteriza cada píxel de la escena en el renderer CPU propio; esta ruta de grabación no abre ventana ni crea un contexto GPU. Se usa el mismo `Scene::render` que en el modo interactivo `--software`. La codificación y mezcla de audio usan FFmpeg.

| Tiempo | Recorrido continuo |
|---|---|
| 00:00–00:02 | Vuelo y entorno inicial |
| 00:02–00:07 | Acercamiento al planeta |
| 00:07–00:20 | Activación del control orbital y más de una vuelta completa |
| 00:20–00:30 | Desactivación del control orbital y alejamiento |

No hay cortes, cambios de escena ni teletransporte durante el recorrido. Las entradas son automáticas y se indica en pantalla; se validan en una prueba de simulación. El archivo tiene FPS fijos para facilitar la presentación, no mide el rendimiento interactivo. No se carga ni modifica progreso del usuario.

Regenerar con `./tools/record-academic.sh`. También:

```sh
cargo build --release --locked --offline
target/release/arcade-galaxy --software --record output/cpu-flight-silent.mp4 --quality balanced
python3 tools/mix_delivery_audio.py output/cpu-flight-silent.mp4 output/Proyecto_Final_CPU.mp4
```

El registro intermedio `cpu-flight-silent.metrics.tsv` incluye tiempos de render y contadores por cuadro; no se versionan archivos intermedios. Verificación: FFprobe confirmó 900 cuadros, 800×450, 30 fps, 30 s, H.264 y AAC estéreo; se decodificó completo sin errores y se revisaron fotogramas de aproximación, órbita y alejamiento.

## Videos adicionales en GPU

Estos videos muestran la edición HD y los minijuegos; **no son evidencia de rasterización CPU**.


Archivo final: `output/Proyecto_Final_Green_Hill_Galaxy.mp4`.

MP4 con video H.264, 1920×1080, 30 fps constantes y audio AAC estéreo. Duración: **2 minutos 16 segundos**. No contiene voz; lleva rótulos en español, efectos del juego sincronizados con sus eventos y una melodía arcade original sintetizada para este video. No utiliza música extraída de Sonic.

## Contenido

| Tiempo | Demostración |
|---|---|
| 00:00–00:07 | Vista general del sistema solar |
| 00:07–00:31 | Vuelo continuo: acercamiento, asistencia orbital y alejamiento, sin cortes ni teletransporte durante el tramo |
| 00:31–00:43 | Vistas de los cuatro planetas y sus modelos OBJ |
| 00:43–01:04 | Entrada al jardín, cuenta atrás, movimiento y disparos |
| 01:04–01:24 | Entrada al dojo, defensa y contraataque |
| 01:24–01:42 | Carrera, cambio de carril, salto y deslizamiento |
| 01:42–02:04 | Plataformas con desplazamiento lateral y colisiones |
| 02:04–02:16 | Vista final y características del proyecto |

Es una **captura del motor real con controles automáticos**: `src/capture.rs` ejecuta la simulación a 120 Hz, captura una imagen cada cuatro pasos y entrega los píxeles al codificador. La duración del archivo no depende de cuánto tarde en procesar cada cuadro. Los 30 fps del archivo no son un benchmark del juego.

El montaje contiene cortes entre secciones y cámaras de presentación para los modelos. El vuelo continuo usa la cámara habitual y entradas de movimiento; no modifica la posición durante ese tramo. Los minijuegos usan las reglas reales y su transición de portal; no se inventan vidas, victorias ni insignias. La grabación tiene progreso aislado en memoria y no altera `saves/progress.txt`.

El video demuestra el modo GPU. El rasterizador CPU continúa disponible con `--software`, con una captura independiente en `output/greenhill-cpu.png`. El rótulo de compatibilidad con DualSense describe la funcionalidad del proyecto; esta grabación no demuestra físicamente la vibración ni una sesión jugada a mano con mando. Los controles automáticos están identificados en el video.

## Regenerar el recorrido completo con cinco mundos (v0.8)

La versión 0.8 del generador produce **176 segundos**: conserva los tramos anteriores y añade selección y carrera Sonic Grand Prix de 02:16 a 02:56. El archivo completo ya existente sigue siendo la entrega v0.7 de 136 segundos hasta ejecutar este comando. El nuevo mundo se entrega también por separado.

Con FFmpeg y ffprobe disponibles en PATH:

```sh
./tools/record-delivery.sh
```

Pasos por separado:

```sh
cargo build --release --offline --locked
target/release/arcade-galaxy --record output/green-hill-silent.mp4
python3 tools/mix_delivery_audio.py output/green-hill-silent.mp4 output/Proyecto_Final_Green_Hill_Galaxy.mp4
```

La primera etapa genera el MP4 sin audio, el registro `.events.tsv` y capturas de revisión por escena. La segunda mezcla los WAV del juego con la melodía original y crea el archivo final. `--record ... --frames 60 --quality fast` permite ensayar dos segundos a 720p. La aplicación gráfica debe poder abrir una ventana OpenGL. Cerrar la ventana cancela la grabación.

No se han añadido nombre, carné o integrantes ficticios. Si se necesitan en pantalla, hay que incorporar los datos reales antes de la entrega institucional.

Para reexportar un intervalo, `--record-start N --frames M` simula el recorrido desde el comienzo y captura solo los cuadros `[N, M)` (30 cuadros por segundo). Los eventos del archivo auxiliar se ajustan al inicio del intervalo.

## Verificación del archivo entregado

FFprobe confirmó H.264 a 1920×1080, 30 fps, 4080 cuadros, 136 segundos y audio AAC estéreo. Tamaño aproximado: 41 MiB. Se decodificó el archivo completo sin errores y se revisaron capturas de cada sección, incluido el marcador de plataformas sin superposición. El pico de audio medido es −7.4 dBFS.

## Sonic Grand Prix — ampliación v0.8

Archivo separado: `output/Sonic_Grand_Prix.mp4`, **40 segundos**, 1920×1080, 30 fps, H.264 y AAC estéreo. Incluye selector de Sonic/Tails/Knuckles, cuenta atrás y una carrera completa de Sonic con carga de Spin Dash, anillos, saltos, loop y llegada. Se captura el motor real con controles automáticos, sin modificar el progreso del jugador.

```sh
target/release/arcade-galaxy --record output/sonic-grand-prix-silent.mp4 --record-start 4080 --frames 5280
python3 tools/mix_delivery_audio.py output/sonic-grand-prix-silent.mp4 output/Sonic_Grand_Prix.mp4
```

El video anterior continúa disponible; este clip añade el quinto mundo. No demuestra una sesión física con DualSense.

Verificación del clip v0.8: FFprobe confirmó 1200 cuadros H.264, 1920×1080, 30 fps y 40 segundos con AAC estéreo. Se decodificó completo sin errores; tamaño aproximado 16 MiB.
