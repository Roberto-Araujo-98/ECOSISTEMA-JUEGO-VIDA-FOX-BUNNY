use macroquad::prelude::*;

use ::rand::{rng, RngExt};
use macroquad::audio::{load_sound, play_sound, stop_sound, PlaySoundParams, Sound};

// Tamaño de la cuadrícula
const FILAS: usize = 16;
const COLUMNAS: usize = 20;
const TAMANIO_CELDA: f32 = 25.0;

const ALTO_PANEL: f32 = 100.0;
const ANCHO_VENTANA: f32 = 500.0;
const ALTO_VENTANA: f32 = (FILAS as f32 * TAMANIO_CELDA) + ALTO_PANEL;

// --- PALETA DE COLORES (look "profesional") ---
const COLOR_FONDO_A: Color = Color::new(0.08, 0.09, 0.12, 1.0);
const COLOR_FONDO_B: Color = Color::new(0.13, 0.14, 0.18, 1.0);
const COLOR_PANEL: Color = Color::new(0.16, 0.17, 0.21, 1.0);
const COLOR_BORDE: Color = Color::new(1.0, 1.0, 1.0, 0.08);
const COLOR_ACENTO_AZUL: Color = Color::new(0.25, 0.55, 0.95, 1.0);
const COLOR_ACENTO_VERDE: Color = Color::new(0.30, 0.70, 0.45, 1.0);
const COLOR_ACENTO_ROJO: Color = Color::new(0.85, 0.30, 0.30, 1.0);
const COLOR_TEXTO_SUAVE: Color = Color::new(0.75, 0.76, 0.80, 1.0);

#[derive(PartialEq, Clone, Copy)]
enum Entidad {
    Vacio,
    Zorro,
    Conejo,
    Misterio,
}

#[derive(PartialEq)]
enum EstadoJuego {
    Menu,
    Simulando,
    VictoriaZorros,
    VictoriaConejos,
    ExtincionMutua,
}

fn ventana_conf() -> Conf {
    Conf {
        window_title: "Ecosistema Avanzado 2D - Desarrollado por Rok".to_string(),
        window_width: ANCHO_VENTANA as i32,
        window_height: ALTO_VENTANA as i32,
        fullscreen: false,
        ..Default::default()
    }
}

// --- UTILIDADES DE DIBUJO "PRO" ---

fn dibujar_fondo_degradado() {
    let pasos = 24;
    let alto_paso = ALTO_VENTANA / pasos as f32;
    for i in 0..pasos {
        let t = i as f32 / pasos as f32;
        let color = Color::new(
            COLOR_FONDO_A.r + (COLOR_FONDO_B.r - COLOR_FONDO_A.r) * t,
            COLOR_FONDO_A.g + (COLOR_FONDO_B.g - COLOR_FONDO_A.g) * t,
            COLOR_FONDO_A.b + (COLOR_FONDO_B.b - COLOR_FONDO_A.b) * t,
            1.0,
        );
        draw_rectangle(0.0, i as f32 * alto_paso, ANCHO_VENTANA, alto_paso + 1.0, color);
    }
}

fn dibujar_texto_sombra(texto: &str, x: f32, y: f32, tam: f32, color: Color) {
    draw_text(texto, x + 1.5, y + 1.5, tam, Color::new(0.0, 0.0, 0.0, 0.5));
    draw_text(texto, x, y, tam, color);
}

fn dibujar_panel(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, COLOR_PANEL);
    draw_rectangle_lines(x, y, w, h, 2.0, COLOR_BORDE);
}

struct Boton {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Boton {
    fn contiene(&self, mx: f32, my: f32) -> bool {
        mx >= self.x && mx <= self.x + self.w && my >= self.y && my <= self.y + self.h
    }

    fn dibujar(&self, texto: &str, color_fondo: Color, color_texto: Color, tam_fuente: f32, hover: bool) {
        let color = if hover {
            Color::new(
                (color_fondo.r + 0.08).min(1.0),
                (color_fondo.g + 0.08).min(1.0),
                (color_fondo.b + 0.08).min(1.0),
                1.0,
            )
        } else {
            color_fondo
        };
        draw_rectangle(self.x, self.y, self.w, self.h, color);
        draw_rectangle_lines(self.x, self.y, self.w, self.h, 2.0, Color::new(1.0, 1.0, 1.0, 0.18));
        let dims = measure_text(texto, None, tam_fuente as u16, 1.0);
        let tx = self.x + (self.w - dims.width) / 2.0;
        let ty = self.y + self.h / 2.0 + dims.height / 2.0;
        draw_text(texto, tx, ty, tam_fuente, color_texto);
    }
}

// --- FUNCIONES DE DIBUJO DE SPRITES (PIXEL ART DIRECTO POR CÓDIGO) ---

fn dibujar_zorro_pixel(x: f32, y: f32) {
    let px = TAMANIO_CELDA / 10.0;
    draw_rectangle(x + 2.0 * px, y + 2.0 * px, px, px, ORANGE);
    draw_rectangle(x + 7.0 * px, y + 2.0 * px, px, px, ORANGE);
    draw_rectangle(x + 2.0 * px, y + 3.0 * px, 6.0 * px, 4.0 * px, ORANGE);
    draw_rectangle(x + 1.0 * px, y + 5.0 * px, 8.0 * px, 2.0 * px, ORANGE);
    draw_rectangle(x + 3.0 * px, y + 6.0 * px, 4.0 * px, 2.0 * px, WHITE);
    draw_rectangle(x + 4.0 * px, y + 7.0 * px, 2.0 * px, px, BLACK);
    draw_rectangle(x + 3.0 * px, y + 4.0 * px, px, px, BLACK);
    draw_rectangle(x + 6.0 * px, y + 4.0 * px, px, px, BLACK);
}

fn dibujar_conejo_pixel(x: f32, y: f32) {
    let px = TAMANIO_CELDA / 10.0;
    draw_rectangle(x + 3.0 * px, y + 1.0 * px, px, 3.0 * px, WHITE);
    draw_rectangle(x + 6.0 * px, y + 1.0 * px, px, 3.0 * px, WHITE);
    draw_rectangle(x + 3.0 * px, y + 2.0 * px, px, px, PINK);
    draw_rectangle(x + 6.0 * px, y + 2.0 * px, px, px, PINK);
    draw_rectangle(x + 2.0 * px, y + 4.0 * px, 6.0 * px, 4.0 * px, WHITE);
    draw_rectangle(x + 1.0 * px, y + 5.0 * px, 8.0 * px, 3.0 * px, WHITE);
    draw_rectangle(x + 3.0 * px, y + 5.0 * px, px, px, BLACK);
    draw_rectangle(x + 6.0 * px, y + 5.0 * px, px, px, BLACK);
    draw_rectangle(x + 4.0 * px, y + 6.0 * px, 2.0 * px, px, PINK);
}

fn dibujar_misterio_pixel(x: f32, y: f32) {
    let px = TAMANIO_CELDA / 10.0;
    draw_rectangle(x + 2.0 * px, y + 2.0 * px, 2.0 * px, 2.0 * px, YELLOW);
    draw_rectangle(x + 6.0 * px, y + 2.0 * px, 2.0 * px, 2.0 * px, YELLOW);
    draw_rectangle(x + 4.0 * px, y + 4.0 * px, 2.0 * px, 2.0 * px, MAGENTA);
    draw_rectangle(x + 2.0 * px, y + 6.0 * px, 2.0 * px, 2.0 * px, YELLOW);
    draw_rectangle(x + 6.0 * px, y + 6.0 * px, 2.0 * px, 2.0 * px, YELLOW);
}

/// Reproduce la música de intro en bucle (deteniéndola antes si ya sonaba).
fn reiniciar_musica_intro(musica_intro: &Option<Sound>) {
    if let Some(m) = &musica_intro {
        stop_sound(m);
        play_sound(m, PlaySoundParams { looped: true, volume: 0.25 });
    }
}

#[macroquad::main(ventana_conf)]
async fn main() {
    println!("Buscando archivos de audio en assets/...");

    let mut musica_intro = None;
    match load_sound("assets/music_intro.ogg").await {
        Ok(m) => {
            println!("¡music_intro.ogg cargada con éxito!");
            musica_intro = Some(m);
        }
        Err(e) => {
            println!("Error grave al buscar o leer music_intro.ogg: {:?}", e);
        }
    }

    if let Some(musica) = &musica_intro {
        play_sound(musica, PlaySoundParams { looped: true, volume: 0.25 });
    }

    let sonido_victoria = load_sound("assets/music_victory.ogg").await.ok();
    if sonido_victoria.is_some() {
        println!("¡music_victory.ogg cargada con éxito!");
    } else {
        println!("Advertencia: No se pudo cargar music_victory.ogg");
    }

    let mut estado_actual = EstadoJuego::Menu;

    let mut prob_zorros: u32 = 15;
    let mut prob_conejos: u32 = 15;
    let mut prob_mutacion: u32 = 5;

    let mut generador = rng();

    let mut tablero = [[Entidad::Vacio; COLUMNAS]; FILAS];
    let mut ultimo_turno = get_time();

    let mut total_zorros = 0;
    let mut total_conejos = 0;
    let mut total_misterios = 0;

    // Botones de la pantalla final (mismos para las 3 variantes de fin de partida)
    let btn_seguir = Boton { x: 60.0, y: 340.0, w: 170.0, h: 55.0 };
    let btn_salir = Boton { x: 270.0, y: 340.0, w: 170.0, h: 55.0 };

    // Botón "Menú" durante la simulación: arriba a la derecha del panel inferior,
    // junto a la zona de INFO/Mutación, donde hay espacio libre.
    let y_panel_const = FILAS as f32 * TAMANIO_CELDA;
    let btn_menu_sim = Boton { x: 430.0, y: y_panel_const + 6.0, w: 60.0, h: 22.0 };

    loop {
        let (mx, my) = mouse_position();

        // --- DETECTOR DE CLICS (INTERFAZ) ---
        if is_mouse_button_pressed(MouseButton::Left) {
            if estado_actual == EstadoJuego::Menu {
                // CONTROLES DE LOS ZORROS (Y: 150 - 190)
                if my >= 150.0 && my <= 190.0 {
                    if mx >= 320.0 && mx <= 360.0 { prob_zorros = (prob_zorros + 5).min(50); }
                    if mx >= 380.0 && mx <= 420.0 { prob_zorros = prob_zorros.saturating_sub(5); }
                }

                // CONTROLES DE LOS CONEJOS (Y: 225 - 265)
                if my >= 225.0 && my <= 265.0 {
                    if mx >= 320.0 && mx <= 360.0 { prob_conejos = (prob_conejos + 5).min(50); }
                    if mx >= 380.0 && mx <= 420.0 { prob_conejos = prob_conejos.saturating_sub(5); }
                }

                // CONTROLES DE MUTACIÓN (Y: 290 - 330)
                if my >= 290.0 && my <= 330.0 {
                    if mx >= 320.0 && mx <= 360.0 { prob_mutacion = (prob_mutacion + 5).min(50); }
                    if mx >= 380.0 && mx <= 420.0 { prob_mutacion = prob_mutacion.saturating_sub(5); }
                }

                // Botón INICIAR JUEGO
                if mx >= 120.0 && mx <= 380.0 && my >= 355.0 && my <= 415.0 {
                    for fila in 0..FILAS {
                        for col in 0..COLUMNAS {
                            let valor = generador.random_range(0..100);
                            if valor < prob_zorros {
                                tablero[fila][col] = Entidad::Zorro;
                            } else if valor < prob_zorros + prob_conejos {
                                tablero[fila][col] = Entidad::Conejo;
                            } else {
                                tablero[fila][col] = Entidad::Vacio;
                            }
                        }
                    }
                    estado_actual = EstadoJuego::Simulando;
                }
            } else if estado_actual == EstadoJuego::VictoriaZorros
                || estado_actual == EstadoJuego::VictoriaConejos
                || estado_actual == EstadoJuego::ExtincionMutua
            {
                if btn_seguir.contiene(mx, my) {
                   
                    if let Some(v) = &sonido_victoria {
                        stop_sound(v);
                    }
                    reiniciar_musica_intro(&musica_intro);
                    estado_actual = EstadoJuego::Menu;
                } else if btn_salir.contiene(mx, my) {
                    // "Salir": cierra el juego por completo.
                    std::process::exit(0);
                }
            } else if estado_actual == EstadoJuego::Simulando {
                if btn_menu_sim.contiene(mx, my) {
                    // Vuelve a la pantalla de parámetros; detiene la música actual
                    // y reinicia la de intro desde el principio.
                    if let Some(v) = &sonido_victoria {
                        stop_sound(v);
                    }
                    reiniciar_musica_intro(&musica_intro);
                    estado_actual = EstadoJuego::Menu;
                }
            }
        }

        // --- LÓGICA DE SIMULACIÓN (TEMPORIZADA) ---
        if estado_actual == EstadoJuego::Simulando && get_time() - ultimo_turno > 0.4 {
            let mut siguiente_tablero = tablero;

            for f in 0..FILAS {
                for c in 0..COLUMNAS {
                    let (v_zorros, v_conejos) = contar_vecinos_especie(&tablero, f, c);

                    match tablero[f][c] {
                        Entidad::Zorro => {
                            if v_zorros == 0 && v_conejos == 0 {
                                siguiente_tablero[f][c] = Entidad::Vacio; // aislamiento total
                            } else if v_zorros >= 3 {
                                siguiente_tablero[f][c] = Entidad::Vacio; // sobrepoblación de zorros
                            } else if v_conejos == 0 {
                                // Sin comida cerca: hay riesgo de morir de hambre, pero no es seguro
                                if generador.random_range(0..100) < 50 {
                                    siguiente_tablero[f][c] = Entidad::Vacio;
                                } else {
                                    siguiente_tablero[f][c] = Entidad::Zorro;
                                }
                            } else {
                                siguiente_tablero[f][c] = Entidad::Zorro;
                            }
                        }
                        Entidad::Conejo => {
                            if v_zorros >= 1 {
                                siguiente_tablero[f][c] = Entidad::Vacio; // devorado
                            } else if (v_conejos == 2 || v_conejos == 3) && v_zorros == 0 {
                                siguiente_tablero[f][c] = Entidad::Conejo; // nacimiento más controlado
                            } else {
                                siguiente_tablero[f][c] = Entidad::Vacio; // aislamiento o sobrepoblación extrema
                            }
                        }
                        Entidad::Vacio => {
                            if v_zorros == 2 && v_conejos >= 1 {
                                siguiente_tablero[f][c] = Entidad::Zorro; // ¡una pareja procrea si hay suficiente comida!
                            } else if v_conejos >= 2 && v_zorros == 0 {
                                siguiente_tablero[f][c] = Entidad::Conejo; // nacimiento estándar
                            } else if generador.random_range(0..100) < prob_mutacion {
                                siguiente_tablero[f][c] = Entidad::Misterio;
                            }
                        }
                        Entidad::Misterio => {
                            let tirada = generador.random_range(0..10);
                            if tirada < 3 {
                                siguiente_tablero[f][c] = Entidad::Zorro;
                            } else if tirada < 6 {
                                siguiente_tablero[f][c] = Entidad::Conejo;
                            } else {
                                siguiente_tablero[f][c] = Entidad::Vacio;
                            }
                        }
                    }
                }
            }

            tablero = siguiente_tablero;

            total_zorros = 0;
            total_conejos = 0;
            total_misterios = 0;

            for f in 0..FILAS {
                for c in 0..COLUMNAS {
                    match tablero[f][c] {
                        Entidad::Zorro => total_zorros += 1,
                        Entidad::Conejo => total_conejos += 1,
                        Entidad::Misterio => total_misterios += 1,
                        _ => {}
                    }
                }
            }

            let mut cambiar_final = false;

            if total_zorros == 0 && total_conejos > 0 {
                estado_actual = EstadoJuego::VictoriaConejos;
                cambiar_final = true;
            } else if total_conejos == 0 && total_zorros > 0 {
                estado_actual = EstadoJuego::VictoriaZorros;
                cambiar_final = true;
            } else if total_zorros == 0 && total_conejos == 0 {
                estado_actual = EstadoJuego::ExtincionMutua;
                cambiar_final = true;
            }

            if cambiar_final {
                // La música de intro se detiene y solo suena la de victoria.
                if let Some(m) = &musica_intro {
                    stop_sound(m);
                }
                if let Some(v) = &sonido_victoria {
                    play_sound(v, PlaySoundParams { looped: false, volume: 0.45 });
                }
            }

            ultimo_turno = get_time();
        }

        // --- RENDERIZADO (DIBUJAR) ---
        dibujar_fondo_degradado();

        match estado_actual {
            EstadoJuego::Menu => {
                dibujar_panel(15.0, 15.0, ANCHO_VENTANA - 30.0, ALTO_VENTANA - 30.0);

                dibujar_texto_sombra("ECOSISTEMA: FOX & BUNNY", 35.0, 60.0, 24.0, WHITE);
                dibujar_zorro_pixel(410.0, 35.0);
                draw_line(35.0, 75.0, ANCHO_VENTANA - 35.0, 75.0, 1.5, COLOR_BORDE);

                draw_text("Ajusta la densidad inicial de cada especie:", 35.0, 100.0, 15.0, COLOR_TEXTO_SUAVE);

                dibujar_control_poblacion(
                    "Zorros", prob_zorros, 150.0, ORANGE, 50.0, "%",
                );
                dibujar_control_poblacion(
                    "Conejos", prob_conejos, 225.0, WHITE, 50.0, "%",
                );
                dibujar_control_poblacion(
                    "Mutación", prob_mutacion, 300.0, YELLOW, 50.0, "%",
                );

                let hover_iniciar = mx >= 120.0 && mx <= 380.0 && my >= 355.0 && my <= 415.0;
                let boton_iniciar = Boton { x: 120.0, y: 355.0, w: 260.0, h: 60.0 };
                boton_iniciar.dibujar("INICIAR SIMULACIÓN", COLOR_ACENTO_AZUL, WHITE, 20.0, hover_iniciar);
            }
            EstadoJuego::Simulando => {
                // 1) Tablero de celdas y criaturas (nada de botones aquí dentro)
                for f in 0..FILAS {
                    for c in 0..COLUMNAS {
                        let x = c as f32 * TAMANIO_CELDA;
                        let y = f as f32 * TAMANIO_CELDA;

                        draw_rectangle(x, y, TAMANIO_CELDA - 1.0, TAMANIO_CELDA - 1.0, Color::new(0.18, 0.18, 0.18, 1.0));

                        match tablero[f][c] {
                            Entidad::Zorro => dibujar_zorro_pixel(x, y),
                            Entidad::Conejo => dibujar_conejo_pixel(x, y),
                            Entidad::Misterio => dibujar_misterio_pixel(x, y),
                            Entidad::Vacio => {}
                        }
                    }
                }

                // 2) Panel inferior con estadísticas
                let y_panel = FILAS as f32 * TAMANIO_CELDA;
                dibujar_panel(0.0, y_panel, ANCHO_VENTANA, ALTO_PANEL);
                draw_line(0.0, y_panel, ANCHO_VENTANA, y_panel, 2.0, COLOR_ACENTO_AZUL);

                dibujar_chip_stat(20.0, y_panel + 18.0, "ZORROS", total_zorros, ORANGE);
                dibujar_chip_stat(190.0, y_panel + 18.0, "CONEJOS", total_conejos, WHITE);

                if total_misterios > 0 {
                    draw_text(
                        &format!("Mutaciones activas: {}", total_misterios),
                        20.0, y_panel + 82.0, 14.0, YELLOW,
                    );
                }

                let x_info = 355.0;
                draw_line(x_info - 15.0, y_panel + 12.0, x_info - 15.0, y_panel + ALTO_PANEL - 12.0, 1.5, COLOR_BORDE);
                draw_text("INFO", x_info, y_panel + 28.0, 14.0, GREEN);
                draw_text("Mutación", x_info, y_panel + 52.0, 14.0, COLOR_TEXTO_SUAVE);
                draw_text(&format!("{}%", prob_mutacion), x_info, y_panel + 74.0, 16.0, COLOR_ACENTO_VERDE);

                // 3) Botón "Menú": se dibuja al final, por encima de todo lo anterior,
                // así no lo tapa el panel ni queda escondido detrás de nada.
                let hover_menu_sim = btn_menu_sim.contiene(mx, my);
                btn_menu_sim.dibujar("Menú", COLOR_ACENTO_ROJO, WHITE, 14.0, hover_menu_sim);
            }
            EstadoJuego::VictoriaConejos => {
                dibujar_pantalla_final(
                    "VICTORIA DE LOS CONEJOS",
                    "Los zorros se extinguieron. La colonia de conejos domina el ecosistema.",
                    WHITE, &btn_seguir, &btn_salir, mx, my,
                );
            }
            EstadoJuego::VictoriaZorros => {
                dibujar_pantalla_final(
                    "VICTORIA DE LOS ZORROS",
                    "Los conejos se extinguieron. Los zorros dominan el ecosistema.",
                    ORANGE, &btn_seguir, &btn_salir, mx, my,
                );
            }
            EstadoJuego::ExtincionMutua => {
                dibujar_pantalla_final(
                    "ECO-EXTINCIÓN COMPLETA",
                    "Ambas especies desaparecieron. Solo mutaciones aleatorias podrían repoblar el tablero.",
                    COLOR_ACENTO_ROJO, &btn_seguir, &btn_salir, mx, my,
                );
            }
        }

        next_frame().await
    }
}

/// Dibuja el control (+/-) de un parámetro en el menú, con una barra de progreso visual.
fn dibujar_control_poblacion(nombre: &str, valor: u32, y: f32, color: Color, maximo: f32, sufijo: &str) {
    draw_text(&format!("{}: {}{}", nombre, valor, sufijo), 35.0, y - 8.0, 18.0, color);

    // Barra de progreso relativa al máximo del control
    let ancho_max = 220.0;
    let ancho_actual = (valor as f32 / maximo) * ancho_max;
    draw_rectangle(35.0, y, ancho_max, 10.0, Color::new(1.0, 1.0, 1.0, 0.08));
    draw_rectangle(35.0, y, ancho_actual, 10.0, color);

    draw_rectangle(320.0, y - 10.0, 40.0, 40.0, Color::new(0.16, 0.32, 0.19, 1.0));
    draw_rectangle_lines(320.0, y - 10.0, 40.0, 40.0, 1.5, COLOR_BORDE);
    draw_text("+", 333.0, y + 18.0, 26.0, WHITE);

    draw_rectangle(380.0, y - 10.0, 40.0, 40.0, Color::new(0.38, 0.16, 0.16, 1.0));
    draw_rectangle_lines(380.0, y - 10.0, 40.0, 40.0, 1.5, COLOR_BORDE);
    draw_text("-", 395.0, y + 16.0, 26.0, WHITE);
}

/// Dibuja una "tarjeta" con el conteo de una especie en el panel inferior.
fn dibujar_chip_stat(x: f32, y: f32, etiqueta: &str, total: i32, color: Color) {
    draw_rectangle(x, y, 150.0, 55.0, Color::new(1.0, 1.0, 1.0, 0.05));
    draw_rectangle_lines(x, y, 150.0, 55.0, 1.5, COLOR_BORDE);
    draw_text(etiqueta, x + 12.0, y + 20.0, 13.0, COLOR_TEXTO_SUAVE);
    draw_text(&format!("{}", total), x + 12.0, y + 44.0, 22.0, color);
}

/// Dibuja la pantalla de fin de partida con título, subtítulo y los dos botones.
fn dibujar_pantalla_final(
    titulo: &str,
    subtitulo: &str,
    color_titulo: Color,
    btn_seguir: &Boton,
    btn_salir: &Boton,
    mx: f32,
    my: f32,
) {
    // Overlay oscuro sobre el degradado de fondo
    draw_rectangle(0.0, 0.0, ANCHO_VENTANA, ALTO_VENTANA, Color::new(0.0, 0.0, 0.0, 0.35));
    dibujar_panel(30.0, 130.0, ANCHO_VENTANA - 60.0, 300.0);

    let dims_titulo = measure_text(titulo, None, 24, 1.0);
    dibujar_texto_sombra(titulo, (ANCHO_VENTANA - dims_titulo.width) / 2.0, 190.0, 24.0, color_titulo);

    draw_multiline_text(subtitulo, 55.0, 230.0, 15.0, Some(1.3), COLOR_TEXTO_SUAVE);

    let hover_seguir = btn_seguir.contiene(mx, my);
    let hover_salir = btn_salir.contiene(mx, my);

    btn_seguir.dibujar("Seguir simulando", COLOR_ACENTO_VERDE, WHITE, 16.0, hover_seguir);
    btn_salir.dibujar("Salir", COLOR_ACENTO_ROJO, WHITE, 16.0, hover_salir);
}

fn contar_vecinos_especie(tablero: &[[Entidad; COLUMNAS]; FILAS], fila: usize, col: usize) -> (i32, i32) {
    let mut zorros = 0;
    let mut conejos = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue; }
            let nueva_f = ((fila as i32 + i + FILAS as i32) % FILAS as i32) as usize;
            let nueva_c = ((col as i32 + j + COLUMNAS as i32) % COLUMNAS as i32) as usize;
            match tablero[nueva_f][nueva_c] {
                Entidad::Zorro => zorros += 1,
                Entidad::Conejo => conejos += 1,
                _ => {}
            }
        }
    }
    (zorros, conejos)
}