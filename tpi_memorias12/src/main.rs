mod config;
mod generador;
mod nueva_simulacion;
mod visualizador;
mod manipular_archivo;
mod proceso;
mod simulador;
mod estrategias;
mod particion;

fn limpiar_consola() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}

fn crear_carpetas() {
    // Crear carpetas si no existen
    let path = std::path::Path::new("files");
    if !path.exists() {
        std::fs::create_dir_all(path).expect("No se pudo crear la carpeta 'files'");
    }
}

fn menu() {
    loop {
        limpiar_consola();
        // Texto de bienvenida e información
        println!("================================================");
        println!("Bienvenido al programa de simulación de memoria");
        println!("Este programa simula la asignación de memoria en un sistema multiprogramado y mono-procesador.");
        println!("Se implementan estrategias como First-Fit, Best-Fit, Next-Fit, y Worst-Fit.");
        println!("El objetivo es estudiar el comportamiento de distintas estrategias de administración de memoria.");
        println!("================================================");
        println!("Desarrollado por: Franco Joaquin Gomez");
        println!("Materia: Sistemas Operativos");
        println!("================================================");
        println!("1) Crear Nueva Simulación");
        println!("2) Ver simulaciones");
        println!("3) Salir");
        println!("");
        println!("Seleccione una opción (1-3): ");

        let mut opcion = String::new();
        std::io::stdin().read_line(&mut opcion).expect("Error al leer opción");

        match opcion.trim() {
            "1" => {
                limpiar_consola(); // Limpiar la pantalla antes de iniciar la simulación
                nueva_simulacion::NuevaSimulacion::nueva_tanda();
            },
            "2" => {
                limpiar_consola(); // Limpiar la pantalla antes de mostrar las simulaciones anteriores
                visualizador::mostrar_simulaciones();
                esperar_entrada_usuario(); // Esperar a que el usuario presione una tecla
            },
            "3" => {
                limpiar_consola(); // Limpiar la pantalla antes de salir
                break;
            },
            _ => {
                limpiar_consola(); // Limpiar la pantalla antes de mostrar que la opción no es válida
                println!("Opción no válida");
            },
        }
    }
}

fn esperar_entrada_usuario() {
    use std::io::{self, Write};
    let mut stdout = io::stdout();
    writeln!(stdout, "\nPresione Enter para continuar...").unwrap();
    stdout.flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}

fn main() {
    crear_carpetas();
    menu();
}
