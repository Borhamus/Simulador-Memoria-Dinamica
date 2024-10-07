// menu.rs
//
// Este módulo maneja el menú principal del programa MemSim3000, permitiendo al usuario
// generar archivos de procesos, seleccionar archivos existentes, configurar y ejecutar simulaciones,
// ver simulaciones anteriores y salir del programa.

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::generador::{generar_archivo_procesos, guardar_simulacion};
use crate::simulacion::ejecutar_simulacion;
use crate::confsim::{ConfiguracionSimulacion, Estrategia};
use crate::io_utils::leer_archivo_procesos;

// Variable global para almacenar el archivo de procesos seleccionado
static mut ARCHIVO_PROCESOS_SELECCIONADO: Option<String> = None;

/// Muestra el menú principal y maneja las opciones seleccionadas por el usuario.
pub fn mostrar_menu() {
    loop {
        // Limpiar la pantalla
        limpiar_pantalla();

        // Mostrar el menú
        println!("=================================================");
        println!("       MemSim3000 - Sistema de Simulación de Memoria");
        println!("=================================================");
        println!("Bienvenido a MemSim3000, un simulador de gestión de memoria que te permite probar diferentes estrategias de asignación de memoria dinámica. Con este software, podrás generar archivos de procesos personalizados, configurar simulaciones con distintos parámetros y analizar los resultados.");

        println!("\nSeleccione una de las siguientes opciones:\n");

        println!("1. Generar un nuevo archivo de procesos.");
        println!("\t(Aquí podrás crear un archivo de procesos a gusto, para poner a prueba el simulador.)\n");

        println!("2. Seleccionar un archivo de procesos.");
        println!("\t(Sirve para poder cargar un archivo y poder comenzar la simulación. Sin un archivo seleccionado no podrás iniciar la configuración de la simulación.)\n");

        println!("3. Configurar y comenzar una simulación.");
        println!("\t(Una vez cargado el archivo, podrás configurar la simulación para poder comenzar. Las simulaciones serán guardadas.)\n");

        println!("4. Ver simulaciones anteriores.");
        println!("\t(Aquí podrás seleccionar un archivo de simulación y poder verlo en pantalla.)\n");

        println!("5. Salir\n");

        print!("Seleccione una opción (1-5): ");
        io::stdout().flush().unwrap();

        let mut seleccion = String::new();
        io::stdin()
            .read_line(&mut seleccion)
            .expect("Error al leer la opción.");

        match seleccion.trim() {
            "1" => generar_nuevo_archivo_procesos(),
            "2" => seleccionar_archivo_procesos(),
            "3" => configurar_y_comenzar_simulacion(),
            "4" => ver_simulaciones_anteriores(),
            "5" => {
                salir();
                break;
            }
            _ => {
                println!("Opción no válida. Presione Enter para intentar nuevamente.");
                esperar_enter();
            }
        }
    }
}

/// Limpia la pantalla de la consola.
fn limpiar_pantalla() {
    // Comando para limpiar la pantalla en Windows y Unix
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .args(&["/C", "cls"])
        .status();

    #[cfg(not(target_os = "windows"))]
    let _ = std::process::Command::new("clear")
        .status();
}

/// Espera a que el usuario presione Enter.
fn esperar_enter() {
    let mut _enter = String::new();
    io::stdin().read_line(&mut _enter).unwrap();
}

/// Genera un nuevo archivo de procesos utilizando el módulo `generador.rs`.
fn generar_nuevo_archivo_procesos() {
    match generar_archivo_procesos() {
        Ok(nombre_archivo) => {
            println!("Archivo de procesos '{}' generado exitosamente.", nombre_archivo);
        }
        Err(e) => {
            println!("Error al generar el archivo de procesos: {}", e);
        }
    }
    println!("Presione Enter para continuar...");
    esperar_enter();
}

/// Permite al usuario seleccionar un archivo de procesos existente.
fn seleccionar_archivo_procesos() {
    println!("Seleccione un archivo de procesos para la simulación:");
    listar_archivos_procesos();

    println!("Ingrese el nombre del archivo (o 'volver' para regresar al menú anterior):");
    let mut nombre_archivo = String::new();
    io::stdin().read_line(&mut nombre_archivo).unwrap();
    let nombre_archivo = nombre_archivo.trim();

    if nombre_archivo.eq_ignore_ascii_case("volver") {
        // Regresa al menú anterior
        return;
    }

    let ruta_archivo = format!("Procesos/{}", nombre_archivo);
    if Path::new(&ruta_archivo).exists() {
        unsafe {
            ARCHIVO_PROCESOS_SELECCIONADO = Some(nombre_archivo.to_string());
        }
        println!("Archivo de procesos '{}' seleccionado correctamente.", nombre_archivo);
    } else {
        println!("El archivo '{}' no existe en la carpeta 'Procesos'. Intente nuevamente.", nombre_archivo);
    }

    println!("Presione Enter para continuar...");
    esperar_enter();
}

/// Lista los archivos disponibles en la carpeta 'Procesos/'.
fn listar_archivos_procesos() {
    let directorio = "Procesos";
    match fs::read_dir(directorio) {
        Ok(entries) => {
            println!("Archivos disponibles en '{}':", directorio);
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(nombre) = entry.file_name().to_str() {
                                println!("- {}", nombre);
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            println!("No se encontró el directorio '{}'. Asegúrate de haber generado archivos de procesos primero.", directorio);
        }
    }
}

/// Configura los parámetros de la simulación y la ejecuta.
fn configurar_y_comenzar_simulacion() {
    // Verificar si se ha seleccionado un archivo de procesos
    let archivo_procesos = unsafe {
        ARCHIVO_PROCESOS_SELECCIONADO.clone()
    };

    if archivo_procesos.is_none() {
        println!("No se ha seleccionado un archivo de procesos. Por favor, selecciona uno antes de continuar.");
        println!("Presione Enter para continuar...");
        esperar_enter();
        return;
    }

    let nombre_archivo = archivo_procesos.unwrap();
    let ruta_archivo = format!("Procesos/{}", nombre_archivo);

    // Leer los procesos del archivo seleccionado
    match leer_archivo_procesos(&ruta_archivo) {
        Ok(procesos) => {
            // Capturar la configuración de la simulación del usuario
            let configuracion = capturar_configuracion();

            // Ejecutar la simulación
            ejecutar_simulacion(configuracion, procesos);
        }
        Err(e) => {
            println!("Error al leer el archivo de procesos: {}", e);
            println!("Presione Enter para continuar...");
            esperar_enter();
        }
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

/// Captura un número entero positivo desde la entrada estándar.
///
/// # Parámetros
/// - `mensaje`: Mensaje a mostrar al solicitar el número.
///
/// # Retorna
/// - `u32`: Número capturado.
fn capturar_numero(mensaje: &str) -> u32 {
    loop {
        print!("{}: ", mensaje);
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer entrada");

        match entrada.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Entrada inválida. Por favor, ingrese un número válido."),
        }
    }
}

/// Permite al usuario ver las simulaciones anteriores.
fn ver_simulaciones_anteriores() {
    println!("Seleccione un archivo de simulación para visualizar:");
    listar_archivos_simulaciones();

    println!("Ingrese el nombre del archivo (o 'volver' para regresar al menú anterior):");
    let mut nombre_archivo = String::new();
    io::stdin().read_line(&mut nombre_archivo).unwrap();
    let nombre_archivo = nombre_archivo.trim();

    if nombre_archivo.eq_ignore_ascii_case("volver") {
        // Regresa al menú anterior
        return;
    }

    let ruta_archivo = format!("Simulaciones/{}", nombre_archivo);
    if Path::new(&ruta_archivo).exists() {
        // Leer y mostrar el contenido del archivo
        match fs::read_to_string(&ruta_archivo) {
            Ok(contenido) => {
                println!("Contenido de '{}':\n", nombre_archivo);
                println!("{}", contenido);
            }
            Err(e) => {
                println!("Error al leer el archivo '{}': {}", nombre_archivo, e);
            }
        }
    } else {
        println!("El archivo '{}' no existe en la carpeta 'Simulaciones'. Intente nuevamente.", nombre_archivo);
    }

    println!("Presione Enter para continuar...");
    esperar_enter();
}

/// Lista los archivos disponibles en la carpeta 'Simulaciones/'.
fn listar_archivos_simulaciones() {
    let directorio = "Simulaciones";
    match fs::read_dir(directorio) {
        Ok(entries) => {
            println!("Archivos disponibles en '{}':", directorio);
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(nombre) = entry.file_name().to_str() {
                                println!("- {}", nombre);
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            println!("No se encontró el directorio '{}'.", directorio);
        }
    }
}

/// Termina el programa.
fn salir() {
    println!("¡Gracias por usar MemSim3000! Hasta luego.");
}
