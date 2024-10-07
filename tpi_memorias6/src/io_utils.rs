
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use crate::proceso::Proceso;

pub fn leer_archivo_procesos(nombre_archivo: &str) -> Result<Vec<Proceso>, String> {
    let path = Path::new(nombre_archivo);
    let file = File::open(&path)
        .map_err(|_| format!("No se pudo abrir el archivo: {}", nombre_archivo))?;
    let buf_reader = BufReader::new(file);

    let mut procesos = Vec::new();

    for (index, line) in buf_reader.lines().enumerate() {
        let linea = line.map_err(|_| format!("Error al leer la línea {} en el archivo.", index + 1))?;
        let partes: Vec<&str> = linea.trim().split(',').collect();

        if partes.len() == 4 {
            let nombre = partes[0].to_string();
            let instante_arribo = partes[1].parse::<u32>().map_err(|_| format!("Error en el formato del instante de arribo en la línea {}", index + 1))?;
            let duracion = partes[2].parse::<u32>().map_err(|_| format!("Error en el formato de la duración en la línea {}", index + 1))?;
            let memoria_requerida = partes[3].parse::<u32>().map_err(|_| format!("Error en el formato de la memoria requerida en la línea {}", index + 1))?;

            procesos.push(Proceso::new(nombre, instante_arribo, duracion, memoria_requerida));
        } else {
            return Err(format!("Formato incorrecto en la línea {}: Se esperaban 4 campos.", index + 1));
        }
    }

    Ok(procesos)
}


// Función para limpiar la pantalla
pub fn limpiar_pantalla() {
    // Este comando funciona en Windows y Unix para limpiar la consola.
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").arg("/C").arg("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}
