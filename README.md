# 🦊🐰 Ecosistema Avanzado 2D: Fox & Bunny

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Macroquad](https://img.shields.io/badge/Macroquad-Game_Engine-blue?style=for-the-badge)

**Ecosistema Avanzado 2D** es una simulación interactiva basada en el paradigma de los autómatas celulares, inspirada en el "Juego de la Vida" de Conway. Desarrollado íntegramente en Rust, este proyecto modela la dinámica poblacional entre depredadores (Zorros) y presas (Conejos), introduciendo variables de caos controladas mediante mutaciones.

Este proyecto representa una inmersión técnica en el ecosistema de Rust, aplicando fundamentos de ingeniería de software para gestionar estados, renderizado en tiempo real y lógica matemática de cuadrículas.

---

## 🎯 Objetivo Académico

El propósito principal de este desarrollo fue comprender y aplicar los conceptos base del lenguaje **Rust** en un entorno de simulación gráfica. Al ser un primer acercamiento al lenguaje, el proyecto sirvió como campo de pruebas para evaluar la transición desde otros paradigmas de programación hacia el sistema de *Ownership* y tipado estricto de Rust.

---

## ⚙️ Arquitectura y Lógica de Simulación

El núcleo del sistema es un espacio bidimensional toroidal (los bordes se conectan) de 16x20 celdas, evaluado en intervalos de 0.4 segundos. Las transiciones de estado se calculan en una matriz paralela (`siguiente_tablero`) para evitar efectos de cascada en el mismo turno.

### Reglas del Autómata
* **🦊 Zorros (Depredadores):** * Mueren por sobrepoblación (>= 3 vecinos zorros) o aislamiento extremo.
    * Tienen un 50% de probabilidad de morir de hambre si no hay conejos adyacentes.
* **🐰 Conejos (Presas):**
    * Mueren inmediatamente si hay al menos 1 zorro en su vecindad.
    * Sobreviven y mantienen el equilibrio si están en grupos de 2 a 3 sin amenazas.
* **🧬 Mutación (Entidad Misterio):**
    * Celdas vacías tienen una probabilidad configurable de generar una mutación espontánea, inyectando nueva vida (zorros o conejos) al ecosistema para prevenir el colapso estático.

---

## 🚀 Requisitos e Instalación

Para compilar y ejecutar esta simulación, es necesario tener el *toolchain* de Rust instalado en el sistema.

### Prerrequisitos técnicos
* **Rust y Cargo:** Versión estable (1.70+ recomendado).
* **Dependencias del sistema:** Las requeridas por la librería gráfica `macroquad` (varían según el sistema operativo: ALSA/Wayland/X11 en Linux, frameworks nativos en Windows/macOS).

### Estructura de Assets
El juego requiere un directorio `assets/` en la raíz del proyecto para la gestión de audio:
```text
.
├── Cargo.toml
├── src/
│   └── main.rs
└── assets/
    ├── music_intro.ogg
    └── music_victory.ogg
Ejecución
Bash
cargo run --release
Nota: Se recomienda compilar con el flag --release para garantizar la máxima fluidez en el motor de renderizado.

🧠 Reflexiones Técnicas: La Experiencia con Rust
Al ser el primer desarrollo en Rust, el proceso evidenció contrastes marcados respecto a otros lenguajes de la ingeniería de software:

Ventajas Observadas
Pattern Matching (match): Resultó ser una herramienta excepcionalmente limpia y robusta para gestionar la lógica de las entidades y la máquina de estados del juego (EstadoJuego).

Rendimiento Predictible: Al no depender de un Garbage Collector, los tiempos de ejecución en el Game Loop a 60 FPS se mantuvieron completamente estables, algo vital en simulaciones gráficas.

Seguridad de Tipos: El compilador forzó a manejar explícitamente los casos nulos o de error (como el sistema de carga de audios con Result y Option).

Dificultades y Curva de Aprendizaje
Ownership y Matrices: Modificar una matriz bidimensional in situ violaba las reglas de borrowing en iteraciones solapadas. La solución arquitectónica óptima fue clonar el estado del tablero (let mut siguiente_tablero = tablero;) en cada tick, garantizando la seguridad de la memoria.

Aritmética de Tipos: Rust es estricto con las conversiones. Intercalar cálculos entre usize (para índices de matrices) y f32 (para coordenadas de dibujado en Macroquad) requirió una atención cuidadosa a los castings.

🔭 Visión a Futuro
El proyecto establece una base sólida que permite expansiones significativas. La hoja de ruta de desarrollo contempla:

Integración de Inteligencia Artificial: Evolucionar de un autómata celular rígido a un sistema de agentes autónomos, aplicando algoritmos de IA (como lógica difusa o redes neuronales simples) para que las entidades "tomen decisiones" de movimiento basadas en su entorno.

Optimización Estructural: Refactorizar el código orientándolo a componentes (ECS - Entity Component System) para soportar cuadrículas masivas (ej. 1000x1000) sin penalización de rendimiento.

Métricas Analíticas: Implementar gráficos en tiempo real sobre la evolución poblacional para un análisis estadístico más profundo del ecosistema.

Desarrollado por Roberto Araujo "Rok".