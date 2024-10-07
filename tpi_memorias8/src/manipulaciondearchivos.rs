//manipulaciondearchivos.rs

use crate::proceso::Proceso;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn listar_archivos_procesos() -> Option<String> {
    let path = Path::new("file/tandasdeprocesos");
    if path.exists() {
        let entries = fs::read_dir(path).expect("No se puede leer la carpeta");
        let mut archivos = vec![];

        for entry in entries {
            let entry = entry.expect("Error al leer entrada");
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            archivos.push(file_name.to_string());
        }

        if archivos.is_empty() {
            return None;
        }

        println!("Archivos disponibles:");
        for (i, archivo) in archivos.iter().enumerate() {
            println!("{}: {}", i + 1, archivo);
        }
        
        println!("Seleccione un archivo (0 para volver):");
        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Error al leer opción");
        let opcion: usize = opcion.trim().parse().unwrap_or(0);

        if opcion == 0 || opcion > archivos.len() {
            return None;
        }

        Some(archivos[opcion - 1].clone())
    } else {
        None
    }
}

pub fn crear_archivo_procesos(num_procesos: u32) -> String {
    let path = format!("file/tandasdeprocesos/tanda_{}.txt", num_procesos);
    let mut archivo = File::create(&path).expect("No se pudo crear el archivo");

    // Escribir procesos en el archivo...

    println!("Archivo creado: {}", path);
    path
}

pub fn listar_simulaciones() {
    let path = Path::new("file/simulaciones");
    if path.exists() {
        let entries = fs::read_dir(path).expect("No se puede leer la carpeta");

        println!("Simulaciones previas:");
        for entry in entries {
            let entry = entry.expect("Error al leer entrada");
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            println!("{}", file_name);
        }
    } else {
        println!("No hay simulaciones previas.");
    }
}

// Función corregida para cargar los procesos desde el archivo con los campos correctos
pub fn cargar_procesos_desde_archivo(nombre_archivo: &str) -> Vec<Proceso> {
    let mut procesos = Vec::new();
    let path = Path::new(nombre_archivo);

    if let Ok(file) = File::open(&path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let partes: Vec<&str> = line.split(';').collect();
                if partes.len() == 4 {
                    let proceso = Proceso {
                        nombre: partes[0].to_string(),
                        instante_arribo: partes[1].parse().unwrap_or(0),
                        duracion_total: partes[2].parse().unwrap_or(0),
                        memoria_requerida: partes[3].parse().unwrap_or(0),
                        tiempo_inicio: 0, // Añadido
                        tiempo_fin: 0,    // Añadido
                    };
                    procesos.push(proceso);
                }
            }
        }
    }

    procesos
}
