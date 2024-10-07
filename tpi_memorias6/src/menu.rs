use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::generador::{generar_archivo_procesos};
use crate::simulacion::ejecutar_simulacion;
use crate::confsim::{ConfiguracionSimulacion, Estrategia};
use crate::io_utils::{leer_archivo_procesos, limpiar_pantalla};

// Estructura que encapsula el estado de la simulación
struct EstadoSimulacion {
    archivo_procesos: Option<String>,
}

impl EstadoSimulacion {
    fn nuevo() -> Self {
        EstadoSimulacion {
            archivo_procesos: None,
        }
    }

    fn set_archivo(&mut self, archivo: String) {
        self.archivo_procesos = Some(archivo);
    }

    fn archivo_cargado(&self) -> bool {
        self.archivo_procesos.is_some()
    }
}

/// Muestra el menú principal y maneja las opciones seleccionadas por el usuario.
pub fn mostrar_menu() {
    let mut estado_simulacion = EstadoSimulacion::nuevo();

    loop {
        limpiar_pantalla();

        println!("=================================================");
        println!("       MemSim3000 - Sistema de Simulación de Memoria");
        println!("=================================================");
        println!("Bienvenido a MemSim3000, un simulador de gestión de memoria.");

        println!("\nSeleccione una de las siguientes opciones:\n");

        println!("1. Generar un nuevo archivo de procesos.");
        println!("2. Seleccionar un archivo de procesos.");
        if estado_simulacion.archivo_cargado() {
            println!("3. Configurar y comenzar una simulación.");
        } else {
            println!("3. (Deshabilitado) Cargue un archivo de procesos antes de simular.");
        }
        println!("4. Ver simulaciones anteriores.");
        println!("5. Salir\n");

        print!("Seleccione una opción (1-5): ");
        io::stdout().flush().unwrap();

        let mut seleccion = String::new();
        io::stdin().read_line(&mut seleccion).expect("Error al leer la opción.");

        match seleccion.trim() {
            "1" => generar_archivo(),
            "2" => seleccionar_archivo(&mut estado_simulacion),
            "3" => {
                if estado_simulacion.archivo_cargado() {
                    configurar_y_simular(&estado_simulacion);
                } else {
                    println!("Debe cargar un archivo de procesos antes de configurar la simulación.");
                }
            },
            "4" => ver_simulaciones_anteriores(),
            "5" => break,
            _ => println!("Opción no válida. Por favor, intente nuevamente."),
        }

        esperar_enter();
    }
}

/// Función para generar un nuevo archivo de procesos
fn generar_archivo() {
    println!("Generando un nuevo archivo de procesos...");
    generar_archivo_procesos();
    println!("Archivo generado con éxito.");
}

/// Función para seleccionar un archivo de procesos
fn seleccionar_archivo(estado: &mut EstadoSimulacion) {
    let carpeta = "file/procesos";
    let archivos = listar_archivos(carpeta);

    // Si no hay archivos en la carpeta, mostrar mensaje
    if archivos.is_empty() {
        println!("No hay archivos disponibles en la carpeta '{}'.", carpeta);
        return;
    }

    // Listar archivos disponibles
    println!("Archivos disponibles:");
    for (indice, archivo) in archivos.iter().enumerate() {
        println!("{:02}. {}", indice + 1, archivo);
    }

    // Instrucción para el usuario sobre cómo salir
    println!("Ingrese '00' para salir y volver al menú principal.");

    // Pedir al usuario que seleccione un archivo
    print!("Seleccione el número del archivo o '00' para salir: ");
    io::stdout().flush().unwrap();
    let mut seleccion = String::new();
    io::stdin().read_line(&mut seleccion).expect("Error al leer la selección.");
    let seleccion = seleccion.trim();

    if seleccion == "00" {
        println!("Regresando al menú principal...");
        return;
    }

    // Verificar que la selección sea un número válido
    let seleccion_num = seleccion.parse::<usize>();

    match seleccion_num {
        Ok(num) if num > 0 && num <= archivos.len() => {
            let archivo_seleccionado = &archivos[num - 1];
            estado.set_archivo(archivo_seleccionado.clone());
            println!("Archivo '{}' cargado correctamente.", archivo_seleccionado);
        }
        _ => println!("Selección inválida. Intente nuevamente."),
    }
}

/// Función para listar archivos en una carpeta
fn listar_archivos(carpeta: &str) -> Vec<String> {
    let mut archivos = Vec::new();

    if let Ok(entries) = fs::read_dir(carpeta) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(nombre_archivo) = path.file_name().and_then(|n| n.to_str()) {
                        archivos.push(nombre_archivo.to_string());
                    }
                }
            }
        }
    }
    archivos
}

/// Función para configurar la simulación y ejecutarla
fn configurar_y_simular(estado: &EstadoSimulacion) {
    let archivo = estado.archivo_procesos.as_ref().expect("Archivo no cargado.");
    
    match leer_archivo_procesos(archivo) {
        Ok(procesos) => {
            let configuracion = capturar_configuracion();
            ejecutar_simulacion(configuracion, procesos);
        }
        Err(e) => println!("Error al leer el archivo de procesos: {}", e),
    }
}

/// Captura los parámetros de configuración de la simulación desde la entrada del usuario.
fn capturar_configuracion() -> ConfiguracionSimulacion {
    println!("Configure los parámetros de la simulación:");

    let tamanio_memoria = capturar_numero("Tamaño de memoria (unidades)");
    println!("Seleccione una estrategia de asignación:");
    println!("1. First-Fit");
    println!("2. Best-Fit");
    println!("3. Worst-Fit");
    println!("4. Next-Fit");

    let estrategia = loop {
        print!("Ingrese el número de la estrategia (1-4): ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        match entrada.trim() {
            "1" => break Estrategia::FirstFit,
            "2" => break Estrategia::BestFit,
            "3" => break Estrategia::WorstFit,
            "4" => break Estrategia::NextFit,
            _ => println!("Opción inválida. Por favor, ingrese un número entre 1 y 4."),
        }
    };

    let tiempo_seleccion = capturar_numero("Tiempo de selección (unidades de tiempo)");
    let tiempo_carga = capturar_numero("Tiempo de carga (unidades de tiempo)");
    let tiempo_liberacion = capturar_numero("Tiempo de liberación (unidades de tiempo)");

    ConfiguracionSimulacion {
        tamanio_memoria,
        estrategia,
        tiempo_seleccion,
        tiempo_carga,
        tiempo_liberacion,
    }
}

/// Captura un número ingresado por el usuario con un mensaje personalizado.
fn capturar_numero(mensaje: &str) -> u32 {
    loop {
        print!("{}: ", mensaje);
        io::stdout().flush().unwrap();
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        match entrada.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Por favor, ingrese un número válido."),
        }
    }
}

/// Función para ver simulaciones anteriores (puedes expandir esto según tus necesidades)
fn ver_simulaciones_anteriores() {
    println!("Mostrando simulaciones anteriores...");
}

/// Espera a que el usuario presione Enter antes de continuar
fn esperar_enter() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
