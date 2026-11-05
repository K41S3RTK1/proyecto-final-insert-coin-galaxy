# Recorrido para demostrar INSERT COIN: GALAXY

Para la evaluación escrita, comenzar con `Evaluar-CPU.command` y `output/Proyecto_Final_CPU.mp4`. Es un vuelo continuo de 30 s renderizado en CPU, incluida una vuelta completa. Los recorridos HD siguientes son adicionales.

La versión 0.7 incorpora un recorrido automático de 2:16, documentado en `VIDEO.md`, exportable con `tools/record-delivery.sh`. Este guion se conserva para una demostración manual ampliada de unos 3–5 minutos. Usa calidad equilibrada o rápida según el FPS observado con F1. Cierra aplicaciones pesadas; desactiva métricas cuando presentes la estética.

1. **Inicio y controles:** mostrar título, mando conectado y audio. Enter/X despega. Explicar en una frase: cinco mundos arcade unidos por una nave, con renderizado GPU HD y modo CPU alternativo.
2. **Vuelo continuo obligatorio:** acercarse al jardín, observar rotación; Tab/triángulo activa asistencia orbital, A/D rodea y W/S cambia radio; desactivar asistencia y alejarse para mostrar sol y planetas. No usar Home ni cortes en este tramo.
3. **Jardín:** volver al anillo verde, E/X a menos de 6 unidades. Mostrar transición y cuenta atrás. Moverse/disparar, identificar abanico, anillo y lluvia. La flor se vuelve violeta a media vida. Mostrar victoria e insignia si se logra, o reintento si se pierde.
4. **Segundo mundo:** regresar con Q/círculo o E/X tras victoria; N/R3 selecciona Neon Dojo. Seguir distancia, dirección y minimapa hasta el portal rosa. No se teletransporta al cambiar destino. Para ensayar rápidamente fuera de la grabación, Home recupera la nave frente al destino.
5. **Dojo:** mostrar bloqueo del puño (K/L2), esquiva tardía del barrido (L/R1) y contraataque durante recuperación (J/cuadrado). Explicar que la ventana del contraataque se abre al defender correctamente.
6. **Persistencia:** tras una victoria, volver al menú de vuelo y cerrar normalmente. Reabrir y mostrar que conserva la insignia. No editar el archivo para simular una victoria en la demostración.
7. **Explicación técnica breve:** F1 y comparación F2/F3 desde la misma cámara pausada. Mostrar `renderer.rs` y una ruta de vértice → clip → triángulo → píxel con profundidad. Explicar CPU para matrices, simulación y culling; GPU para rasterización, materiales y antialiasing. Mostrar `--software` si se quiere explicar el rasterizador CPU original.

## Ensayo de acciones nuevas

- N/R3 cambia destino solo en vuelo; la nave no salta de posición.
- E/X activa portal únicamente cerca y da tiempo para prepararse dentro.
- M/Share silencia y reactiva; el motor cambia con velocidad.
- F4/cruceta abajo cambia calidad sin perder partida.
- Jardín: reconocer los tres avisos y el carril libre de lluvia.
- Dojo: mantener bloqueo no evita un barrido; atacar solo no gana.
- Pausa congela cronómetro; desconexión del mando o pérdida de foco pausa.
- Ganar registra insignia/mejor tiempo; perder no lo borra; repetir más lento conserva récord.

Las partidas automáticas de pruebas validan las reglas, pero la comodidad del mando nuevo y la dificultad requieren ensayo humano. La versión anterior ya fue probada por el usuario con teclado y DualSense.


## Banana Rush y vibración — versión 0.4

N/R3 cicla Jardín → Dojo → Banana Rush. El tercer planeta conserva el Minion y el ferrocarril; en la versión 0.7 comparte la vegetación y los aros dorados del sistema Green Hill. Entra con E/X. Muestra un cambio de carril (A/D o stick), salto (espacio/X), deslizamiento (S/L2), bananas y trenes. La insignia se obtiene a 900 m con 20 bananas; J/R2 permite seguir en infinito después de la meta. Al terminar, reabrir conserva el mejor puntaje y las insignias anteriores.

F5/cruceta arriba conmuta la vibración; se detiene en pausa y pérdida de foco. F1 muestra pulsos aceptados por la API nativa. Verificar sosteniendo el DualSense la diferencia entre banana, salto, choque y portal. No afirmar sensación física basándose solo en el contador. Mantener un único mando conectado durante esta comprobación.

## Mushroom Kingdom y calidad HD — versión 0.5

1. Mostrar F1: renderizado GPU a 1920×1080; F4 cambia a 2560×1440 y 1280×720.
2. En vuelo, N selecciona Mushroom Kingdom. Para una demostración rápida, Home acerca al portal y E entra.
3. A/D mueve; espacio salta; Shift corre. En DualSense: stick izquierdo, X y R2.
4. Mostrar el primer bloque «?», el champiñón y el cambio a Super Mario. Romper un ladrillo y pisar un goomba.
5. Avanzar a la bandera: mostrar el desplazamiento lateral, paralaje, checkpoint y cuarta insignia.
6. Explicar AABB, separación horizontal/vertical, gravedad, tolerancia al borde y salto anticipado.
7. Mostrar una captura F12 con suavizado aplicado. Los modelos son recreaciones propias, no assets extraídos del juego.

## Sonic Grand Prix — versión 0.8

Selecciona SONIC GRAND PRIX con N/R3 en vuelo y entra al portal con E/X. Para ensayo rápido: N hasta ese destino, Home, E. En el selector: A/D o stick izquierdo, Enter/X confirma. Muestra los tres modelos OBJ. En carrera: A/D o stick dirige, Espacio/X salta, Shift/R2 acelera, mantener y soltar K/L2 hace Spin Dash y L/R1 activa la habilidad. Sonic ataca con dash, Tails vuela y Knuckles planea. Muestra el loop, anillos, un salto y la quinta insignia al llegar primero. P/Options pausa, Q/círculo regresa al sistema. `output/Sonic_Grand_Prix.mp4` es un clip separado del nuevo mundo; el video v0.7 conserva sus cuatro desafíos originales.
