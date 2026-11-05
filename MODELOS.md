# Modelos OBJ — versión 0.8

La nave y los cinco planetas exteriores se cargan desde archivos Wavefront OBJ con materiales MTL. Se pueden importar en Blender; conserva cada .obj junto a su .mtl. Los modelos contienen grupos de piezas y normales de vértice (`vn`, caras `v//vn`). Los objetos se triangulan antes de cargar. Los interiores conservan su geometría de juego y añaden palmeras y loops OBJ como decoración.

- **Nave:** casco de disco con paneles radiales, mandíbulas, cabina lateral, ventiladores, tornillería, antena, acoples y aro segmentado de propulsión. La misma malla se utiliza en el vuelo y el jardín.
- **Garden:** relieve radial, zonas de agua y vegetación, gran árbol, hongos y monumento floral.
- **Dojo:** ciudad con ventanas animadas, torres, pagoda, puerta y aro de neón.
- **Banana Rush:** ferrocarril ecuatorial, edificios, tren y Minion monumental.
- **Sonic Grand Prix:** circuito azul ecuatorial, arco de meta ajedrezado, aros dorados y tres marcadores de corredores; incluye LOD.
- **Mushroom Kingdom:** castillo con torres, tuberías, hongos y bloques de interrogación.

La vegetación y edificios se modelan en marcos tangentes a la esfera y se exportan como parte del planeta. Al aplicar la matriz del planeta, todos sus detalles orbitan y rotan juntos. La deformación de la superficie se calcula al generar el archivo; no es animación de deformación durante la partida.

## Estética Green Hill

Todos los planetas incorporan terrazas de césped, acantilados ajedrezados, palmeras, aros y cascadas. `hill-palm.obj` y `hill-loop.obj` se reutilizan dentro de los cuatro escenarios. `checker_cliff` y `water_blue` seleccionan los nuevos shaders; la superficie planetaria utiliza coordenadas esféricas para su damero. La pintura de la nave es azul/blanca con detalles dorados.

## Geometría entregada

| Archivo | Vértices | Triángulos |
|---|---:|---:|
| `arcade-ship.obj` | 12,485 | 9,720 |
| `dojo-low.obj` | 17,368 | 15,816 |
| `dojo.obj` | 33,960 | 41,048 |
| `garden-low.obj` | 20,496 | 23,076 |
| `garden.obj` | 51,664 | 66,708 |
| `hill-loop.obj` | 800 | 1,600 |
| `hill-palm.obj` | 2,184 | 2,108 |
| `mario-low.obj` | 13,239 | 14,901 |
| `mario.obj` | 33,091 | 43,917 |
| `runner-low.obj` | 13,344 | 14,976 |
| `runner.obj` | 33,836 | 46,840 |

| `race-low.obj` | 11,332 | 13,952 |
| `race.obj` | 30,800 | 44,712 |
| `sonic.obj` | 18,244 | 31,852 |
| `tails.obj` | 19,892 | 34,972 |
| `knuckles.obj` | 29,460 | 51,808 |

Las versiones `-low.obj` se usan cuando la distancia al planeta supera 42 unidades. F3 permite desactivar LOD y F1 muestra los triángulos enviados. Se aplica descarte por volumen antes de transformar y descarte de caras posteriores antes de rasterizar. Ambos renderizadores usan las mismas posiciones, normales y materiales.

## Regenerar

```sh
python3 tools/build_ship.py
python3 tools/build_planets.py
python3 tools/build_racers.py
cargo build --release --offline --locked
./tools/package-macos.sh
```

El generador usa únicamente Python estándar. `tools/asset_mesh.py` implementa superficies, cilindros, cajas biseladas, toros, marcos tangentes y exportación. Las normales suaves se usan en superficies curvas y se separan en aristas de piezas rígidas. No se descargaron modelos de terceros.

Los materiales `hero_` usan iluminación suave y brillo de borde tenue para conservar los colores de los corredores. Los materiales `paint_` emplean iluminación suave; `surface_` añade transiciones continuas de vegetación/agua; `city_` ventanas; `terrain_` terreno; `glass` cristal; `glow_` y `engine` superficies emisivas. El cargador lee Kd del MTL y usa estas convenciones para elegir los shaders propios. Ke/Ks/Ns se exportan para edición externa, pero este renderer no interpreta sus valores: no afirmar compatibilidad completa con todos los materiales de Blender. No hay mapas de textura ni normal mapping.

## Capturas reproducibles

```sh
target/release/arcade-galaxy --gpu-render output/ship-v6.png --scene ship
target/release/arcade-galaxy --gpu-render output/garden-model-v6.png --scene garden-model
target/release/arcade-galaxy --gpu-render output/dojo-model-v6.png --scene dojo-model
target/release/arcade-galaxy --gpu-render output/runner-model-v6.png --scene runner-model
target/release/arcade-galaxy --gpu-render output/mario-model-v6.png --scene mario-model
```

Para comprobar el pipeline CPU, sustituye `--gpu-render` por `--render`.

## Corredores articulados de Sonic Grand Prix

`sonic.obj`, `tails.obj` y `knuckles.obj` contienen cada figura completa para inspección/importación. Durante el juego se cargan las piezas `*-core.obj`, `*-arm-l.obj`, `*-arm-r.obj`, `*-leg-l.obj` y `*-leg-r.obj` alrededor de pivotes. Tails añade `tails-tails.obj` para animar sus dos colas; `*-ball.obj` representa la forma enrollada de cada corredor. Cada OBJ tiene su propio MTL y normales suaves.

Las animaciones son transformaciones de piezas rígidas y no skinning esquelético. El generador `tools/build_racers.py` construye hocico, ojos/iris, púas o mechones, dedos, guantes y calzado. Modelos propios basados en las referencias de Sonic, Tails y Knuckles del estudiante, sin descargar modelos oficiales.

```sh
target/release/arcade-galaxy --gpu-render output/race-v8.png --scene race
target/release/arcade-galaxy --gpu-render output/sonic-model-v8.png --scene sonic-model
target/release/arcade-galaxy --gpu-render output/race-loop-v8.png --scene race-loop
```
