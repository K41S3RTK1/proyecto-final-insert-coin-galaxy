# Sonic Grand Prix — quinto mundo, v0.8

Carrera de 1050 metros en un circuito Green Hill con anillos, palmeras, acantilados ajedrezados, resortes, aceleradores, huecos y un loop vertical de 360°. Elige Sonic, Tails o Knuckles; los otros dos compiten como rivales automáticos. La cámara sigue al corredor y se abre lateralmente en el loop.

## Entrar

1. Abre `Jugar.command` o `target/Insert Coin Galaxy.app`. Cierra primero cualquier versión anterior.
2. Enter/X despega. En vuelo, N/R3 cambia destino hasta **SONIC GRAND PRIX** (cuatro cambios desde el jardín).
3. Acércate al portal y pulsa E/X. Para probar rápidamente con teclado, Home coloca la nave frente al destino seleccionado.
4. Elige con A/D, flechas izquierda/derecha o stick izquierdo y confirma con Enter/E/X. La cuenta atrás dura tres segundos.

## Controles

| Acción | Teclado | DualSense |
|---|---|---|
| Dirigir lateralmente | A/D o flechas izquierda/derecha | Stick izquierdo |
| Saltar | Espacio, W o flecha arriba | X |
| Boost (consume energía) | Mantener Shift | Mantener R2 o L3 |
| Cargar Spin Dash | Mantener K o Ctrl izquierdo | Mantener L2 |
| Lanzar Spin Dash | Soltar K/Ctrl | Soltar L2 |
| Habilidad del personaje | L | R1 |
| Pausar | P/Escape | Options |
| Volver al sistema | Q | Círculo |

El avance es automático. W/S o stick vertical ajustan ligeramente la velocidad. Los anillos recargan energía y absorben un golpe; al recibirlo se pierden los anillos. Sin anillos, el golpe cuesta una vida. Caer cuesta una vida y vuelve al último checkpoint. Hay tres vidas, cuatro checkpoints y un límite de 120 segundos. Llegar primero otorga la quinta insignia; se conserva el mejor tiempo.

- **Sonic:** mayor velocidad base, dash enrollado y ataque dirigido a un robot cercano que esté por delante.
- **Tails:** vuelo temporal con sus dos colas girando, útil para atravesar huecos.
- **Knuckles:** planeo y golpe fuerte capaz de romper enemigos/cajas mientras la habilidad está activa.

Las habilidades tienen cinco segundos de recarga. Los tres pueden saltar, usar boost y Spin Dash. El loop mantiene automáticamente al corredor sobre la pista; dirige para recoger anillos. El control del mando usa las mismas acciones que el teclado y los eventos se integran con el audio/vibración existentes. La sensación física del DualSense requiere probarla sosteniendo el mando.

## Modelos y verificación

Mallas OBJ/MTL propias inspiradas en las referencias aportadas: púas azules, ojos verdes, guantes y zapatillas de Sonic; dos colas, orejas y pelaje de Tails; mechones rojos, ojos violetas, guantes con púas y calzado de Knuckles. Son recreaciones del proyecto, no modelos oficiales extraídos. Se animan por piezas con pivotes. Generador reproducible: `python3 tools/build_racers.py`; catálogo en `MODELOS.md`.

47 pruebas automatizadas: los tres corredores pueden ganar mediante entradas reales, se comprueban habilidades, loop, checkpoint, selección, pausa y guardado. El formato de progreso v4 conserva los cuatro mundos de v3. Se verificaron capturas de la selección, personajes y loop con GPU y una captura CPU. Muestra de 120 cuadros a 1080p: 54.1 FPS, p95 23.1 ms; no es una garantía de toda la partida.

Captura del nuevo mundo: `output/Sonic_Grand_Prix.mp4`. Es una demostración con controles automáticos del motor real y progreso aislado; no modifica tu partida. El video v0.7 de cuatro mundos sigue disponible por separado.
