# Tres características opcionales — versión 0.9

Implementadas en ambos modos; para defender el pipeline propio, utilizar `Evaluar-CPU.command`. Permanecen los cinco mundos, sus modelos OBJ/MTL y los controles de teclado/PS5.

| Característica | Comparación | Evidencia |
|---|---|---|
| Distribución por ruido Perlin 3D | Cambiar `--seed` en el generador | Posiciones, valores de ruido y semilla en `assets/models/placements.json`; mismos sitios en ambos LOD |
| Normal mapping CPU | F6 / cruceta derecha, con F1 abierto | PNG de normales, muestreo bilineal, matriz TBN e iluminación por fragmento |
| Gravedad sobre la nave | G / cruceta izquierda durante vuelo libre | HUD muestra estado y aceleración; trayectoria se curva hacia los planetas |

## Distribución reproducible

`tools/noise3d.py` implementa ruido de gradiente Perlin 3D con interpolación quíntica y hash de retícula con semilla. Referencia del algoritmo: [Ken Perlin, Improved Noise (2002)](https://cs.nyu.edu/~perlin/noise/). El hash con semilla es una implementación del proyecto. Se combinan tres octavas; no se usa el ruido solo para pintar la superficie.

El generador evalúa 1024 candidatos sobre la esfera, los ordena por ruido y selecciona posiciones con una separación mínima de 0.38 sobre la esfera unitaria. Reserva zonas para monumentos y evita colocar decoración común a menos de 0.48 de sus centros o de las islas. Las restricciones son separaciones entre centros, no una garantía de ausencia de toda intersección entre hojas y piezas. Los detalles se orientan con un marco tangente y se hornean en OBJ. Cambiar la semilla cambia las posiciones. Los monumentos, portales y pistas principales conservan su composición manual. Ambos LOD usan la misma lista de posiciones.

Semilla entregada: **2026**. Para comparar sin sobrescribir los modelos del juego:

```sh
python3 tools/build_planets.py --seed 2026 --only garden --output-dir output/seed-2026
python3 tools/build_planets.py --seed 42 --only garden --output-dir output/seed-42
```

Importar los dos `garden.obj` en Blender o comparar sus `placements.json`. Para actualizar todos los modelos del juego: `python3 tools/build_planets.py --seed 2026`, seguido de `./tools/package-macos.sh` después de compilar. No se regeneran aleatoriamente en cada inicio.

**Alcance de la rúbrica:** falta confirmar si Perlin fue uno de los algoritmos vistos en clase. La implementación existe y es verificable; ese requisito específico debe contrastarse con el material del curso.

## Normal mapping real en software

`assets/textures/terrain-normal.png` guarda vectores en RGB en espacio tangente. Se genera con `python3 tools/build_normal_map.py`, a partir de derivadas de altura Perlin periódica. No contiene color de terreno.

`src/normal_map.rs` decodifica RGB a [-1, 1], repite la textura, filtra bilinealmente y transforma la normal muestreada mediante tangente, bitangente y normal geométrica (TBN). El rasterizador interpola posición local, normal y tangente con corrección de perspectiva antes de muestrear. Las coordenadas UV son esféricas y analíticas; no requieren ampliar el cargador OBJ con `vt`. La tangente gira con el modelo. La intensidad es 0.8 y se aplica a la superficie de los cinco planetas, no a los personajes.

La normal resultante modifica la iluminación; no cambia vértices, profundidad ni silueta. Una prueba compara ambos framebuffers y verifica exactamente el mismo z-buffer. GPU usa la misma textura y transformación en GLSL. No se afirma compatibilidad genérica con mapas definidos en MTL: es un material explícito del proyecto.

```sh
target/release/arcade-galaxy --render output/normal-map-cpu-on.png --scene garden-model --quality high
target/release/arcade-galaxy --render output/normal-map-cpu-off.png --scene garden-model --quality high --no-normal-map
```

Para HD, sustituir `--render` por `--gpu-render`. Durante el juego, acercarse al planeta, pausar con P y alternar F6 permite ver el cambio con la cámara fija. F1 muestra ON/OFF. Cambiar resolución con F4 conserva el estado.

## Gravedad planetaria

`src/gravity.rs` suma la atracción de los cinco centros planetarios actuales. Utiliza una fuerza inversamente proporcional al cuadrado de la distancia con suavizado de 3 m, parámetro gravitatorio de 180 y desvanecimiento continuo entre 30 y 40 m. El suavizado evita singularidades. Se integra la aceleración en la velocidad y después la velocidad en la posición usando dt.

La gravedad está activada por defecto en vuelo libre. Al soltar los controles queda inercia con amortiguación suave, por lo que la atracción se aprecia; Ctrl/K o L2 frena y Shift/L3 permite alejarse con turbo. G/cruceta izquierda vuelve al comportamiento anterior sin gravedad. La orientación sigue dependiendo del jugador y la cámara acompaña la nave. La asistencia orbital de Tab mantiene su radio impuesto y el HUD indica AUTO: no se presenta como una órbita física. La gravedad planetaria no altera las reglas de salto de los minijuegos.

Demostración: acercarse al jardín, salir de asistencia orbital si está activa, dar impulso lateral con A/D y soltar. Observar la curvatura, velocidad y aceleración. Repetir con G desactivado. Para repetir desde el arranque: ejecutar con y sin `--no-gravity`. Home puede ayudar a preparar la comparación, pero no debe usarse durante el recorrido continuo de la consigna.

## Comprobaciones

```sh
cargo test --release --locked --offline
cargo clippy --locked --offline --all-targets -- -D warnings
python3 tools/test_procedural.py
```

54 pruebas Rust y 3 pruebas Python. Se verifican continuidad, repetición y cambio de semilla; coincidencia del manifiesto con los OBJ de ambos LOD; textura periódica, TBN rotada y polos; iluminación diferente con idéntica geometría; dirección y alcance de la gravedad, pausa, activación y convergencia entre 30 y 120 pasos por segundo. Las pruebas anteriores de minijuegos y vuelo académico continuo permanecen activas.

Capturas de comparación: [CPU activado](output/normal-map-cpu-on.png), [CPU desactivado](output/normal-map-cpu-off.png), [GPU activado](output/normal-map-gpu-on.png), [GPU desactivado](output/normal-map-gpu-off.png).
