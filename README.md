# Conway’s Game of Life en Rust

## Descripción

Este proyecto es una implementación simple del juego de la vida de Conway usando el lenguaje Rust. El juego simula células que viven, mueren o nacen en una cuadrícula basada en reglas específicas.

## Instalación y Ejecución

Para correr el proyecto, asegúrate de tener Rust instalado. Luego, clona el repositorio y ejecuta:

```bash
cargo run
```

Esto compilará y ejecutará el juego.

## Controles

-   **Espacio**: Pausar / reanudar la simulación.
-   **R**: Reiniciar con un patrón inicial (varios organismos).
-   **C**: Limpiar toda la cuadrícula.
-   **P**: Guardar una captura en `screenshot.png`.
-   **Flecha ↑ / ↓**: Subir / bajar la velocidad de la simulación.

## Detalles técnicos

-   Grid lógico: **100 × 100** celdas, con bordes **toroidales** (los bordes están conectados).
-   Ventana: **800 × 800** px. El grid se **escala** para ocupar toda la ventana.
-   Framebuffer propio dibujado con `raylib` (se copia a la ventana en cada frame).

## Nota sobre el GIF de demostración

El GIF incluido en la carpeta `demo` muestra una ejecución típica del juego para ilustrar su funcionamiento.
