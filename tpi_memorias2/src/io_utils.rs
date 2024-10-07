// io_utils.rs


use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Lee el archivo de procesos línea por línea y devuelve un vector de `Proceso`.
pub fn leer_archivo_procesos(ruta_archivo: &str) -> Result<Vec<crate::proceso::Proceso>, std::io::Error> {
    let archivo = File::open(Path::new(ruta_archivo))?;
    let lector = BufReader::new(archivo);
    let mut procesos: Vec<crate::proceso::Proceso> = Vec::new();

    for linea in lector.lines() {
        let linea = linea?;
        let partes: Vec<&str> = linea.split(',').collect();

        if partes.len() != 4 {
            // Validación adicional para asegurarse de que haya exactamente 4 partes en la línea.
            eprintln!("Error: la línea no contiene los 4 campos esperados. Línea: {}", linea);
            continue; // Saltar líneas con formato incorrecto.
        }

        let nombre = partes[0].to_string();
        let arribo = match partes[1].parse::<u32>() {
            Ok(valor) => valor,
            Err(_) => {
                eprintln!("Error al parsear el tiempo de arribo en la línea: {}", linea);
                continue;
            }
        };
        let duracion = match partes[2].parse::<u32>() {
            Ok(valor) => valor,
            Err(_) => {
                eprintln!("Error al parsear la duración en la línea: {}", linea);
                continue;
            }
        };
        let memoria_requerida = match partes[3].parse::<u32>() {
            Ok(valor) => valor,
            Err(_) => {
                eprintln!("Error al parsear la memoria requerida en la línea: {}", linea);
                continue;
            }
        };

        let proceso = crate::proceso::Proceso::new(nombre, arribo, duracion, memoria_requerida);
        procesos.push(proceso);
    }

    Ok(procesos)
}

/// Función que lista los archivos de procesos disponibles en la carpeta `files`.
pub fn listar_archivos_procesos() -> Vec<String> {
    let mut archivos: Vec<String> = Vec::new();
    
    if let Ok(entradas) = fs::read_dir("files") {
        for entrada in entradas {
            if let Ok(entrada) = entrada {
                let path = entrada.path();
                if let Some(extension) = path.extension() {
                    if extension == "txt" {
                        if let Some(nombre_archivo) = path.file_name() {
                            if let Ok(nombre_archivo) = nombre_archivo.to_os_string().into_string() {
                                archivos.push(nombre_archivo);
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Error: no se pudo leer el directorio 'files'.");
    }
    
    archivos
}
