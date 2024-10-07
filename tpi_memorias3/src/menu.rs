//menu.rs


use std::io::{self, Write};
use crate::io_utils::{self, leer_archivo_procesos, listar_archivos_o_generar};
use crate::proceso::Proceso; // Asegúrate de importar Proceso correctamente
use crate::simulacion::ejecutar_simulacion;
use crate::confsim::ConfiguracionSimulacion;

pub fn mostrar_menu() {
    println!("================================================");
    println!("1. Seleccionar o crear un archivo de procesos");
    println!("2. Ver simulaciones anteriores");
    println!("3. Salir");
}

/// Gestiona la selección o creación de un archivo de procesos
pub fn seleccionar_o_crear_archivo() {
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
    println!("Tiempo de selección de partición (ms):");
    let mut tiempo_seleccion = String::new();
    io::stdin().read_line(&mut tiempo_seleccion).unwrap();
    let tiempo_seleccion: u32 = tiempo_seleccion.trim().parse().expect("Debe ser un número");

    // Tiempo de carga promedio
    println!("Tiempo de carga promedio (ms):");
    let mut tiempo_carga = String::new();
    io::stdin().read_line(&mut tiempo_carga).unwrap();
    let tiempo_carga: u32 = tiempo_carga.trim().parse().expect("Debe ser un número");

    // Tiempo de liberación de partición
    println!("Tiempo de liberación de partición (ms):");
    let mut tiempo_liberacion = String::new();
    io::stdin().read_line(&mut tiempo_liberacion).unwrap();
    let tiempo_liberacion: u32 = tiempo_liberacion.trim().parse().expect("Debe ser un número");

    // Configuración de la simulación
    let config = ConfiguracionSimulacion {
        tamanio_memoria,
        estrategia: estrategia.trim().to_string(),
        tiempo_seleccion,
        tiempo_carga,
        tiempo_liberacion,
    };

    // Llamamos a la simulación
    ejecutar_simulacion(&config, procesos);
}
