//visualizador.rs

use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

/// Función que lista las simulaciones previas y permite seleccionar una para mostrarla por pantalla.
pub fn mostrar_simulaciones() {
    // Listamos los archivos de simulaciones almacenados en la carpeta correspondiente
    let simulaciones = enlistar_simulaciones();

    // Si no hay simulaciones, lo notificamos al usuario
    if simulaciones.is_empty() {
        println!("No se encontraron simulaciones previas.");
        return;
    }

    // Mostramos las simulaciones disponibles
    println!("Simulaciones disponibles:");
    for (i, archivo) in simulaciones.iter().enumerate() {
        println!("{}: {}", i + 1, archivo);
    }

    // Solicitamos al usuario que seleccione una simulación para ver
    println!("Seleccione el número de la simulación que desea ver (0 para volver):");

    let mut opcion = String::new();
    io::stdin()
        .read_line(&mut opcion)
        .expect("Error al leer opción");
    let opcion: usize = opcion.trim().parse().unwrap_or(0);

    // Si el usuario selecciona 0 o un número fuera de rango, volvemos al menú principal
    if opcion == 0 || opcion > simulaciones.len() {
        println!("Volviendo al menú principal...");
        return;
    }

    // Cargamos y mostramos el archivo de simulación seleccionado
    let archivo_seleccionado = &simulaciones[opcion - 1];
    mostrar_por_pantalla(archivo_seleccionado);
}

/// Función que lista los archivos de simulaciones existentes en el directorio.
fn enlistar_simulaciones() -> Vec<String> {
    let mut archivos_simulaciones = Vec::new();
    let path = Path::new("files");

    // Verificamos si el directorio existe
    if path.exists() {
        // Leemos el contenido del directorio de simulaciones
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(e) => {
                println!("Error al leer la carpeta de simulaciones: {}", e);
                return Vec::new(); // Retornamos un vector vacío en caso de error
            }
        };

        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name = file_name.to_string_lossy().to_string();
                archivos_simulaciones.push(file_name);
            }
        }
    } else {
        println!("La carpeta 'files' no existe.");
    }

    archivos_simulaciones
}

/// Función que muestra por pantalla el contenido de un archivo de simulación seleccionado.
pub fn mostrar_por_pantalla(nombre_archivo: &str) {
    let path = format!("files/{}", nombre_archivo);
    let path = Path::new(&path);

    // Intentamos abrir el archivo de simulación
    if let Ok(file) = File::open(&path) {
        let reader = io::BufReader::new(file);

        println!("\nContenido de la simulación '{}':\n", nombre_archivo);
        // Mostramos el contenido del archivo línea por línea
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    } else {
        println!("No se pudo abrir el archivo de simulación.");
    }
}
