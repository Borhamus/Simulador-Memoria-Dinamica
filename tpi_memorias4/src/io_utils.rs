use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::proceso::Proceso;

pub fn leer_archivo_procesos(nombre_archivo: &str) -> Result<Vec<Proceso>, String> {
    let path = Path::new(nombre_archivo);
    let file = File::open(&path).map_err(|_| format!("No se pudo abrir el archivo: {}", nombre_archivo))?;
    let buf_reader = io::BufReader::new(file);

    let mut procesos = Vec::new();

    for (index, line) in buf_reader.lines().enumerate() {
        let linea = line.map_err(|_| format!("Error al leer la línea {} en el archivo.", index + 1))?;
        let partes: Vec<&str> = linea.split(',').collect();
        
        if partes.len() == 4 {
            // Suponemos que el formato del archivo tiene 4 columnas: nombre, arribo, duración, memoria
            let nombre = partes[0].to_string();
            let arribo = partes[1].parse::<u32>().map_err(|_| format!("Error de formato en la línea {}: arribo no es un número", index + 1))?;
            let duracion = partes[2].parse::<u32>().map_err(|_| format!("Error de formato en la línea {}: duración no es un número", index + 1))?;
            let memoria = partes[3].parse::<u32>().map_err(|_| format!("Error de formato en la línea {}: memoria no es un número", index + 1))?;

            procesos.push(Proceso::new(nombre, arribo, duracion, memoria));
        } else {
            return Err(format!("Error de formato en la línea {}: se esperaban 4 columnas.", index + 1));
        }
    }

    Ok(procesos)
}
