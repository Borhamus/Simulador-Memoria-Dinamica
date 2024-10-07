// main.rs
//
// Este es el archivo principal que inicia la ejecución del simulador de
// asignación de particiones dinámicas. Maneja el flujo del programa.

use std::fs;
use std::path::Path;
mod confsim;
mod estrategias;
mod generador;
mod io_utils;
mod menu;
mod particion;
mod proceso;
mod simulacion;

fn main() {
    // Verificar y crear carpetas necesarias
    verificar_o_crear_carpetas();

    // Inicia el programa mostrando el menú principal
    menu::mostrar_menu();
}

/// Verifica si las carpetas 'file/simulacion' y 'file/procesos' existen.
/// Si no existen, las crea.
fn verificar_o_crear_carpetas() {
    let carpetas = ["file/simulacion", "file/procesos"];

    for carpeta in carpetas.iter() {
        let path = Path::new(carpeta);
        if !path.exists() {
            match fs::create_dir_all(path) {
                Ok(_) => println!("Carpeta creada: {}", carpeta),
                Err(e) => println!("Error al crear la carpeta {}: {}", carpeta, e),
            }
        }
    }
}
