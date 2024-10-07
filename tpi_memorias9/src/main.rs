//main.rs

mod configuracion;
mod estrategias;
mod manipular_archivo;
mod memoria;
mod nueva_simulacion;
mod particion;
mod proceso;
mod simulacion;
mod visualizador;
mod generador;

use std::fs;
use std::path::Path;
use std::process::Command; // Para limpiar la pantalla

fn main() {
    // Limpiar la pantalla
    limpiar_pantalla();
    
    // Crear las carpetas necesarias
    crear_carpetas();

    // Iniciar el menú principal
    menu_principal();
}

// Limpiar la pantalla
fn limpiar_pantalla() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

// Crea la carpeta 'files'
fn crear_carpetas() {
    let path = Path::new("files");
    if !path.exists() {
        fs::create_dir_all(path).expect("No se pudo crear la carpeta 'files'");
        println!("Carpeta 'files' creada.");
    }
}

// Menú principal
fn menu_principal() {
    loop {
        println!("Bienvenido al simulador de memoria dinámica");
        println!("1) Iniciar nueva simulación");
        println!("2) Ver simulaciones anteriores");
        println!("3) Salir");

        let opcion = capturar_opcion();

        match opcion.as_str() {
            "1" => nueva_simulacion::NuevaSimulacion::new().iniciar_nueva_tanda(), // Llama a la función que maneja la simulación
            "2" => visualizador::mostrar_simulaciones(),
            "3" => {
                println!("Saliendo del programa...");
                break;
            }
            _ => println!("Opción no válida. Intente nuevamente."),
        }
    }
}

// Capturar opción del menú
fn capturar_opcion() -> String {
    use std::io::{self, Write};
    let mut opcion = String::new();
    print!("Seleccione una opción: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut opcion).expect("Error al leer la opción");
    opcion.trim().to_string()
}
