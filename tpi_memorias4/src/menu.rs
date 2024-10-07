//menu.rs

use std::io::{self, Write};
use crossterm::{execute, terminal::{Clear, ClearType}};
use crate::confsim::capturar_configuracion;
use crate::simulacion::ejecutar_simulacion;
use crate::io_utils::leer_archivo_procesos;

// Función para mostrar la pantalla de bienvenida
pub fn mostrar_bienvenida() {
    execute!(io::stdout(), Clear(ClearType::All)).unwrap();

    println!("=================================================");
    println!("MemSim3000 - Simulador de Memoria");
    println!("=================================================");
    println!("Creado por: Franco Joaquín Gómez - Alias: Borhamus Fynolt");
    println!("=================================================");
    println!("Presiona Enter para continuar...");
    
    let mut _enter = String::new();
    io::stdin().read_line(&mut _enter).unwrap();
}

// Función para mostrar el menú principal
pub fn mostrar_menu() {
    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
    
    println!("=================================================");
    println!("MemSim3000 - Sistema de Simulación de Memoria");
    println!("=================================================");
    
    println!("Seleccione una de las siguientes opciones:");
    println!("1. Configurar y comenzar una simulación");
    println!("2. Ver simulaciones anteriores");
    println!("3. Salir");
    
    print!("Seleccione una opción (1-3): ");
    io::stdout().flush().unwrap();
    
    let mut seleccion = String::new();
    io::stdin().read_line(&mut seleccion).expect("Error al leer la opción.");
    
    match seleccion.trim() {
        "1" => configurar_simulacion(),
        "2" => ver_simulaciones_anteriores(),
        "3" => salir(),
        _ => {
            println!("Opción no válida. Intente nuevamente.");
            mostrar_menu();
        }
    }
}

// Función para capturar configuración y comenzar la simulación
fn configurar_simulacion() {
    let configuracion = capturar_configuracion();

    // Asumiendo que los procesos se obtienen de un archivo
    println!("Por favor, selecciona el archivo de procesos para la simulación:");
    let archivo_procesos = seleccionar_archivo_procesos();

    match leer_archivo_procesos(&archivo_procesos) {
        Ok(procesos) => {
            // Llama a ejecutar_simulacion pasando la referencia y los procesos
            ejecutar_simulacion(&configuracion, procesos);
        }
        Err(e) => {
            println!("Error al leer el archivo de procesos: {}", e);
        }
    }
}

// Función para seleccionar un archivo de procesos
fn seleccionar_archivo_procesos() -> String {
    println!("Aquí deberías listar los archivos de procesos existentes o crear uno nuevo.");
    println!("Por ahora, devolviendo un nombre de archivo ficticio.");
    // Este código debe ser reemplazado por una selección real de archivos.
    "01.Procesos(10).best-fit.txt".to_string()
}

// Función para ver simulaciones anteriores
fn ver_simulaciones_anteriores() {
    println!("Ver simulaciones anteriores - Funcionalidad pendiente.");
}

// Función para salir del programa
fn salir() {
    println!("Saliendo del programa...");
    std::process::exit(0);
}
