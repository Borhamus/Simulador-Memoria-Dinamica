// generador.rs


use std::fs::{self, File};
use std::io::{Write, BufWriter}; // Corregimos la importación aquí
use rand::Rng;
use std::io::{stdin, stdout}; // Esto parece que lo usas para I/O básico

/// Genera un archivo de procesos con el nombre dado y la cantidad de procesos especificada.
pub fn generar_archivo_procesos(cantidad_procesos: usize) {
    // Buscamos el último número de archivo generado para continuar el conteo.
    let numero_archivo = obtener_siguiente_numero_archivo();
    let nombre_archivo = format!("{:02}.Procesos({}).txt", numero_archivo, cantidad_procesos);

    let archivo = File::create(&nombre_archivo).expect("No se pudo crear el archivo.");
    let mut writer = BufWriter::new(archivo);
    
    let mut rng = rand::thread_rng();

    for i in 1..=cantidad_procesos {
        let nombre_proceso = format!("Proceso{}", i);
        let arribo = rng.gen_range(0..100);
        let duracion = rng.gen_range(1..20);
        let memoria_requerida = rng.gen_range(10..500);

        writeln!(writer, "{},{},{},{}", nombre_proceso, arribo, duracion, memoria_requerida)
            .expect("Error al escribir en el archivo.");
    }

    println!("Archivo '{}' con {} procesos generado correctamente.", nombre_archivo, cantidad_procesos);
}

/// Devuelve el siguiente número de archivo disponible en la secuencia (01, 02, 03...).
fn obtener_siguiente_numero_archivo() -> u32 {
    let archivos_existentes = obtener_lista_archivos();
    if let Some(ultimo_archivo) = archivos_existentes.last() {
        // Extraemos el número del archivo más reciente
        if let Some(numero) = ultimo_archivo.split('.').next() {
            return numero.parse::<u32>().unwrap_or(0) + 1;
        }
    }
    1 // Si no hay archivos, empezamos en 1
}

/// Devuelve una lista de archivos existentes que cumplen el patrón "{:02}.Procesos(...).txt".
fn obtener_lista_archivos() -> Vec<String> {
    fs::read_dir("./")
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let filename = entry.file_name().into_string().ok()?;
            if filename.starts_with("Procesos") && filename.ends_with(".txt") {
                Some(filename)
            } else {
                None
            }
        })
        .collect()
}
