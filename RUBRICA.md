# Verificación contra la consigna — 2026-11-10

Se revisaron las cinco páginas de **PROYECTO_FINAL GRAFICAS 2026.pdf**, aportado por el estudiante, y el código de la entrega v0.9. Esta auditoría distingue requisitos obligatorios, características demostrables y evaluación del docente. No representa una nota otorgada.

## Requisitos obligatorios

La consigna indica que faltar a cualquiera inhabilita la evaluación. **Se debe presentar el modo CPU (`Evaluar-CPU.command` / `--software`)**. El arranque HD usa GPU y no cumple literalmente el primer requisito; se conserva como ampliación solicitada por el estudiante. No se presupone una autorización docente para sustituir el renderer CPU por GPU.

| Requisito del PDF | Evidencia verificada | Alcance |
|---|---|---|
| 1. Pipeline propio por software; raylib solo ventana, entradas y presentación | `Renderer::begin`, `model`, `clip_triangle`, `triangle`, `fragment` en src/renderer.rs; matrices propias en src/math.rs; rama CPU de src/main.rs sube los píxeles del framebuffer y los muestra como textura 2D | Cumple en `--software`. No se instancia `Gpu` ni se aplican shaders GPU a la escena en esa ruta. Sin motor de juego/física externo |
| 2. Sol, nave propia controlable, planeta con órbita y rotación por tiempo | src/scene.rs, assets/arcade-ship.obj/.mtl, tools/build_ship.py; cinco planetas en `planet_at`/`spin_at` de src/game.rs | Pruebas de dt, órbita, giro y controles; modelos propios del proyecto |
| 3. Acercarse, rodear y alejarse sin cambiar escena; cámara controlable | `Mode::Flight`, control libre y asistencia orbital. Prueba `academic_flight_approaches_orbits_fully_and_leaves_without_cuts` | 30 s de vuelo continuo: se comprueba aproximación, giro superior a 360°, alejamiento, continuidad y permanencia en Flight. Los portales a minijuegos son opcionales |
| 4. Cargar modelos, transformaciones MVP, profundidad y shader propio | src/obj.rs, src/math.rs, z-buffer y `fragment` en src/renderer.rs | Tests de índices/normales, orden de profundidad, clipping y carga/rasterización de los cinco planetas; captura CPU real |
| 5. Fuentes, recursos, instrucciones y video del recorrido | Repositorio, README.md, ENTREGA.md, assets/, Cargo.lock y output/Proyecto_Final_CPU.mp4 | Video CPU de 30 s sin cortes y con controles automáticos identificados. Los videos HD se entregan como evidencia adicional |

## Características que se solicitan evaluar

Puntos **postulables**, sujetos a funcionar en la demostración, identidad visual y explicación del estudiante. No se cuentan los minijuegos como planetas adicionales ni variantes de color como shaders nuevos.

| Característica del menú | Máximo solicitado | Evidencia y demostración |
|---|---:|---|
| Cinco planetas distinguibles | 50 | Jardín/árbol, ciudad/pagoda, ferrocarril/Minion, castillo/tuberías y circuito/meta. Modelos en assets/models; N elige destino, F1 permite observar rendimiento |
| Cuatro tipos de modelos de superficie | 20 | Árbol/palmera, edificio/pagoda, tren y castillo/monumento. Generadores propios en tools/build_planets.py y asset_mesh.py; no contar tamaños del mismo árbol como tipos |
| Orientación correcta sobre superficie curva | 5 | `Mesh.surface` crea marcos tangentes; la matriz del planeta transforma sus detalles al rotar |
| Cuatro shaders propios diferentes | 24 | `Style::Sun` (bandas), City (ventanas), Checker (damero) y Water (ondas); implementaciones distintas en `fragment` CPU. Mostrar sol, ciudad, acantilados y agua |
| Shader animado | 8 | Sol, agua o ventanas usan tiempo en el shader CPU; pausar/reanudar |
| Deformación geométrica | 10 | `radial` de build_planets.py altera el radio de la esfera con funciones trigonométricas. Deformación horneada en OBJ, no animación ni ruido del curso |
| Generación procedural de geometría | 10 | `Mesh::sphere`, `Mesh::portal` y `Mesh::cylinder` generan vértices/índices en CPU antes de rasterizar; no geometry shader GPU |
| Modo local para rodear planeta | 8 | Tab cerca del destino; A/D rodean y W/S modifican el radio, sin cortar el vuelo |
| Aceleración, frenado y giro progresivos | 8 | Integración de movimiento y suavizado con dt en game.rs; WASD, Ctrl/K y flechas |
| Colisión básica con planetas | 4 | `resolve_collisions` impide penetración y elimina velocidad hacia el interior; test de colisión |
| Descarte de objetos fuera de vista | 7 | Frustum culling anterior a transformación/rasterización. F1 muestra descartes; F2 compara desde cámara pausada |
| Nivel de detalle por distancia | 8 | OBJ cercanos/lejanos y esferas con distinta resolución; umbral de 42 unidades para planetas. F1/F3 permiten comparar triángulos |
| Distribución de instancias por ruido | 10, condicionados | Perlin 3D con semilla en tools/noise3d.py; posiciones horneadas en OBJ, manifiesto y LOD coincidentes. Falta confirmar que Perlin coincida con lo visto en clase; ver OPCIONALES.md |
| Normal mapping en software | 8 | PNG RGB de normales, filtro bilineal y TBN por fragmento en src/normal_map.rs y renderer.rs; F6 compara sin cambiar geometría |
| Atracción gravitatoria planetaria | 10 | src/gravity.rs y vuelo libre en game.rs; G compara, HUD muestra aceleración. No confundir con asistencia orbital |
| Coherencia visual y composición | 0–5 | Dirección Green Hill y modelos específicos; valoración docente |
| Claridad de controles y recorrido | 0–5 | Menús, HUD, ayuda H, documentación y video; valoración docente |

Subtotal técnico postulable: **190 puntos**, de los cuales **10 dependen de confirmar que Perlin se vio en clase**; sin ese criterio, **180**. Presentación: hasta **10** adicionales. La consigna limita la nota a 100. Esto indica margen técnico para aspirar a 100, **no garantiza esa nota**: cada criterio depende de la demostración y explicación, y el docente puede asignar puntos parciales.

Los tres opcionales antes pendientes ya tienen implementación. [OPCIONALES.md](OPCIONALES.md) explica cómo compararlos y sus límites.

## Validación de entrega

- v0.9: 54 pruebas Rust en release, 3 pruebas Python de ruido/distribución y Clippy sin advertencias. Capturas CPU/GPU con normal mapping activado/desactivado verificadas. Aplicación macOS empaquetada con textura y manifiesto.
- Video CPU verificado con FFprobe: H.264, 800×450, 900 cuadros a 30 fps, 30 s y AAC estéreo; decodificación completa sin errores y revisión de acercamiento, órbita y alejamiento.
- Validación histórica v0.8: copia limpia clonada desde Git, con directorio de compilación nuevo: `cargo test --release --locked --offline` aprobó 48/48; build release y exportación de la vista general CPU completados usando solo código y assets versionados. Las dependencias se tomaron de la caché Cargo local.
- Ruta CPU sin ventana para capturas y video: no inicializa un contexto GPU.
- v0.9: CPU 640×360, 120 cuadros de cámara fija: jardín cercano **29.5 FPS**, p95 **34.2 ms**; sistema **67.7 FPS**, p95 **14.9 ms**. GPU 1920×1080, 120 cuadros de jardín cercano: **53.3 FPS**, p95 **24.9 ms** (límite 60 FPS). Muestras de esta Mac con normal mapping activado; no garantía de toda la partida.
- Mediciones históricas v0.8, CPU a 640×360, 120 cuadros de cámara fija: jardín cercano **34.5 FPS**, p95 **29.2 ms**; vista general **72.4 FPS**, p95 **14.0 ms**. Son mediciones puntuales en esta Mac, no garantía de otra máquina o todo el juego.
- Otras vistas históricas v0.8, CPU a 640×360: Dojo **81.8 FPS**, Runner **48.0 FPS**, Mario **82.1 FPS**, Race **75.7 FPS**. Contadores y condiciones en output/validacion-entrega.txt.
- Datos de autoría: proyecto individual de **Daniel Sandoval**, contribuciones detalladas en ENTREGA.md.
- Regeneración del video CPU: tools/record-academic.sh; el FPS del archivo es fijo y no es un benchmark.
- El mando fue probado por el usuario en versiones anteriores. La sensación física de las acciones nuevas/vibración requiere ensayo humano; no se acredita mediante un contador de API.

## Pendiente de la entrega y defensa

La entrega institucional debe acompañarse de las declaraciones adicionales que solicita la página 4 de la consigna; no se certifica aquí su presentación al docente.

Daniel debe practicar y explicar el pipeline y una contribución técnica concreta, y poder modificar un parámetro en vivo. Esa comprobación individual y la calificación final corresponden al docente; no pueden certificarse mediante herramientas automáticas.
