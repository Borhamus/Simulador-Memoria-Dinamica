//menu.rs

use std::io;
use std::process::Command;
use crate::manipulaciondearchivos::{listar_archivos_procesos, listar_simulaciones, crear_archivo_procesos};
use crate::simulacion::Simulacion;
use crate::estrategias::EstrategiaAsignacion;


pub fn iniciar_menu() {
    loop {

        limpiar_pantalla();  // Limpia la pantalla al iniciar el menú

        println!("Bienvenido al simulador de memoria dinámica");

        println!("");
        println!("Este programa forma parte de un trabajo practico integrador de sistemas operativos,");
        println!("de tercer año, de la carrera Lic. en Sistemas de la universidad de Tierra del Fuego.");
        println!("");
        println!("Primero usted deberá crear un archivo de tanda de procesos.");
        println!("");
        println!("Dicho archivo tendrá un numero de procesos,");
        println!("el archivo simulara una tanda de procesos en este formato:");
        println!("    * Nombre de proceso.");
        println!("    * Instante de Arribo.");
        println!("    * Duración total del trabajo.");
        println!("    * Cantidad de memoria requerida.");
        println!("");

        println!("0) Salir.");
        println!("1) Crear una nueva simulación.");
        println!("2) Ver simulaciones previas.");

        println!("");
        println!("Seleccione una opción (0-3):");

        let mut opcion = String::new();
        std::io::stdin().read_line(&mut opcion).expect("Error al leer opción");
        let opcion = opcion.trim();

        match opcion {
            "1" => crear_nueva_simulacion(),
            "2" => ver_simulaciones_previas(),
            "0" => {
                println!("Saliendo del programa...");
                break;
            },
            _ => println!("Opción no válida. Intente nuevamente."),
        }
    }
}


// CREO QUE ESTO NO VA ACA.
fn seleccionar_archivo_existente(state: &mut MenuState) {
    if let Some(archivo) = listar_archivos_procesos() {
        println!("Ha seleccionado el archivo: {}", archivo);
        state.archivo_cargado = Some(archivo);
    } else {
        println!("No hay archivos disponibles.");
    }
}

fn crear_nuevo_archivo(state: &mut MenuState) {
    println!("Ingrese el número de procesos:");
    let mut num_procesos = String::new();
    std::io::stdin().read_line(&mut num_procesos).expect("Error al leer número de procesos");
    let num_procesos: u32 = num_procesos.trim().parse().unwrap_or(1);

    let archivo = crear_archivo_procesos(num_procesos);
    println!("Archivo creado con {} procesos.", num_procesos);
    state.archivo_cargado = Some(archivo);
}

fn iniciar_simulacion(state: &mut MenuState) {
    println!("Configuración de la simulación:");

    // Tamaño de la memoria física disponible
    println!("Ingrese el tamaño de la memoria física disponible:");
    let tamanio_memoria = leer_u32();

    // Estrategia de asignación de particiones
    println!("Seleccione la estrategia de asignación de particiones:");
    println!("1) First-Fit");
    println!("2) Best-Fit");
    println!("3) Worst-Fit");
    println!("4) Next-Fit");
    let estrategia_opcion = leer_u32();
    let estrategia = match estrategia_opcion {
        1 => EstrategiaAsignacion::FirstFit,
        2 => EstrategiaAsignacion::BestFit,
        3 => EstrategiaAsignacion::WorstFit,
        4 => EstrategiaAsignacion::NextFit,
        _ => {
            println!("Opción no válida, se selecciona First-Fit por defecto.");
            EstrategiaAsignacion::FirstFit
        }
    };

    // Otros parámetros
    println!("Ingrese el tiempo de selección de partición:");
    let tiempo_seleccion = leer_u32();

    println!("Ingrese el tiempo de carga promedio (memoria secundaria a principal):");
    let tiempo_carga = leer_u32();

    println!("Ingrese el tiempo de liberación de partición:");
    let tiempo_liberacion = leer_u32();

    // Confirmación
    println!("Configuración ingresada:");
    println!("Tamaño de memoria: {}", tamanio_memoria);
    println!("Estrategia de asignación: {:?}", estrategia);
    println!("Tiempo de selección de partición: {}", tiempo_seleccion);
    println!("Tiempo de carga promedio: {}", tiempo_carga);
    println!("Tiempo de liberación de partición: {}", tiempo_liberacion);

    println!("¿Está conforme con la configuración? (S/N)");
    let mut respuesta = String::new();
    std::io::stdin().read_line(&mut respuesta).expect("Error al leer respuesta");
    let respuesta = respuesta.trim().to_uppercase();

    if respuesta == "S" {
        let archivo = state.archivo_cargado.clone().unwrap();
        let mut simulacion = Simulacion::new(
            &archivo,
            tamanio_memoria,
            estrategia,
            tiempo_seleccion,
            tiempo_carga,
            tiempo_liberacion,
        );
        simulacion.ejecutar();
    } else {
        println!("Volviendo al menú principal...");
    }
}

// Función auxiliar para leer un u32
fn leer_u32() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer número");
    input.trim().parse().unwrap_or(0)
}

// Función para limpiar la pantalla, multiplataforma
pub fn limpiar_pantalla() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Error al limpiar pantalla en Windows");
    } else {
        Command::new("clear")
            .status()
            .expect("Error al limpiar pantalla en sistemas Unix");
    }
}

fn ver_simulaciones_previas() {
    listar_simulaciones();
}