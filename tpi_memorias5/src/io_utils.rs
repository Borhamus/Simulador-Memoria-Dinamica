// io_utils.rs
//
// Este módulo contiene funciones para leer archivos de procesos y escribir eventos
// de la simulación a archivos de salida.

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use crate::proceso::Proceso;

/// Lee un archivo de procesos y devuelve un vector de procesos.
///
/// # Parámetros
///
/// - `nombre_archivo`: Ruta al archivo de procesos que se desea leer.
///
/// # Retorna
///
/// - `Result<Vec<Proceso>, String>`: Un vector de `Proceso` si la lectura es exitosa,
///   o un mensaje de error en caso contrario.
///
/// # Formato del Archivo
///
/// El archivo debe tener el siguiente formato, con cada proceso en una línea:
///
/// ```text
/// NombreProceso,InstanteArribo,Duracion,MemoriaRequerida
/// ```
///
/// Ejemplo:
///
/// ```text
/// Proceso1,0,5,100
/// Proceso2,2,3,150
/// ```
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
            // Desestructuramos las partes para mayor claridad
            let (nombre, arribo_str, duracion_str, memoria_str) = (partes[0], partes[1], partes[2], partes[3]);

            // Parseamos los valores numéricos y manejamos posibles errores
            let arribo = arribo_str.parse::<u32>()
                .map_err(|_| format!("Error de formato en la línea {}: el instante de arribo '{}' no es un número válido.", index + 1, arribo_str))?;
            let duracion = duracion_str.parse::<u32>()
                .map_err(|_| format!("Error de formato en la línea {}: la duración '{}' no es un número válido.", index + 1, duracion_str))?;
            let memoria_requerida = memoria_str.parse::<u32>()
                .map_err(|_| format!("Error de formato en la línea {}: la memoria requerida '{}' no es un número válido.", index + 1, memoria_str))?;

            let proceso = Proceso::new(nombre.to_string(), arribo, duracion, memoria_requerida);
            procesos.push(proceso);
        } else {
            return Err(format!("Error de formato en la línea {}: se esperaban 4 columnas separadas por comas.", index + 1));
        }
    }

    Ok(procesos)
}
