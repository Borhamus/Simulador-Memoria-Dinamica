//menu.rs

mod generador;
mod io_utils;
mod simulacion;
mod proceso;
mod particion;

use std::io::{self, Write}; 
// Asegúrate de que esta función está correctamente definida en simulacion.rs
use crate::simulacion::ejecutar_simulacion; 

// Importar las funciones necesarias de io_utils
use crate::io_utils::{listar_archivos_simulaciones, leer_archivo_procesos, leer_archivo_simulacion}; 



/// Muestra el menú principal y maneja las opciones del usuario
pub fn manejar_menu() {
    loop {
        mostrar_bienvenida();

        let mut opcion = String::new();
        print!("Selecciona una opción: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut opcion).unwrap();
        let opcion = opcion.trim();

        match opcion {
            "1" => seleccionar_o_crear_archivo(),
            "2" => ver_simulaciones_anteriores(),
            "3" => {
                println!("Saliendo del programa...");
                break;
            },
            _ => println!("Opción no válida. Intenta nuevamente."),
        }
    }
}

/// Muestra el mensaje de bienvenida
fn mostrar_bienvenida() {
    println!("================================================");
    println!("Bienvenido al programa de simulación de memoria");
    println!("Se trata de un sistema multiprogramado y mono-procesador.");
    println!("================================================");
    println!("1. Seleccionar o crear un archivo de procesos");
    println!("2. Ver simulaciones anteriores");
    println!("3. Salir");
}

/// Gestiona la selección o creación de un archivo de procesos
fn seleccionar_o_crear_archivo() {
    listar_archivos_o_generar();

    println!("Ingrese el nombre del archivo de procesos a cargar:");
    let mut nombre_archivo = String::new();
    io::stdin().read_line(&mut nombre_archivo).unwrap();
    let nombre_archivo = nombre_archivo.trim();

    match io_utils::leer_archivo_procesos(nombre_archivo) {
        Ok(procesos) => {
            println!("Archivo cargado correctamente.");
            mostrar_procesos(&procesos);
            configurar_simulacion(procesos); // Pasamos los procesos a la simulación
        }
        Err(_) => println!("Error al cargar el archivo."),
    }
}

/// Muestra los procesos cargados
fn mostrar_procesos(procesos: &[Proceso]) {
    println!("Procesos cargados:");
    for proceso in procesos {
        println!(
            "Proceso: {}, Arribo: {}, Duración: {}, Memoria: {}",
            proceso.nombre, proceso.arribo, proceso.duracion, proceso.memoria_requerida
        );
    }
}

/// Configura la simulación después de cargar los procesos
fn configurar_simulacion(procesos: Vec<Proceso>) {
    println!("Ingrese los parámetros de la simulación.");

    // Tamaño de memoria
    println!("Tamaño de la memoria física disponible (KB):");
    let mut tamanio_memoria = String::new();
    io::stdin().read_line(&mut tamanio_memoria).unwrap();
    let tamanio_memoria: u32 = tamanio_memoria.trim().parse().expect("Debe ser un número");

    // Estrategia de asignación
    println!("Estrategia de asignación (first-fit, best-fit, next-fit, worst-fit):");
    let mut estrategia = String::new();
    io::stdin().read_line(&mut estrategia).unwrap();

    // Tiempo de selección de partición
    println!("Tiempo de selección de partición (en milisegundos):");
    let mut tiempo_seleccion = String::new();
    io::stdin().read_line(&mut tiempo_seleccion).unwrap();
    let tiempo_seleccion: u32 = tiempo_seleccion.trim().parse().expect("Debe ser un número");

    // Tiempo de carga promedio
    println!("Tiempo de carga promedio (en milisegundos):");
    let mut tiempo_carga = String::new();
    io::stdin().read_line(&mut tiempo_carga).unwrap();
    let tiempo_carga: u32 = tiempo_carga.trim().parse().expect("Debe ser un número");

    // Tiempo de liberación de partición
    println!("Tiempo de liberación de partición (en milisegundos):");
    let mut tiempo_liberacion = String::new();
    io::stdin().read_line(&mut tiempo_liberacion).unwrap();
    let tiempo_liberacion: u32 = tiempo_liberacion.trim().parse().expect("Debe ser un número");

    // Ejecutar simulación
    println!("Trabajando...");
    println!("====== Simulación en progreso ======");

    ejecutar_simulacion(
        tamanio_memoria,
        estrategia.trim(),
        tiempo_seleccion,
        tiempo_carga,
        tiempo_liberacion,
        procesos,
    );
}

/// Ver simulaciones anteriores
fn ver_simulaciones_anteriores() {
    let archivos = io_utils::listar_archivos_simulaciones(); 
    if archivos.is_empty() {
        println!("No se encontraron simulaciones anteriores.");
    } else {
        println!("Simulaciones disponibles:");
        for archivo in archivos {
            println!("{}", archivo);
        }
    }

    println!("Ingrese el nombre del archivo de simulación para ver:");
    let mut nombre_archivo = String::new();
    io::stdin().read_line(&mut nombre_archivo).unwrap();
    let nombre_archivo = nombre_archivo.trim();

    match io_utils::leer_archivo_simulacion(nombre_archivo) {
        Ok(simulacion) => {
            println!("Simulación cargada:");
            // Mostrar los resultados de la simulación
            for linea in simulacion {
                println!("{}", linea);
            }
        }
        Err(_) => println!("Error al cargar el archivo de simulación."),
    }
}
