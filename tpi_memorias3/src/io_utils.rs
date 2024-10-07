// io_utils.rs


use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::path::Path;
use crate::proceso::Proceso; // Asegúrate de tener la estructura `Proceso` definida

/// Lee un archivo de texto que contiene la lista de procesos.
/// Retorna un `Vec<Proceso>` o un `Result::Err` si ocurre un error.
pub fn leer_archivo_procesos(nombre_archivo: &str) -> Result<Vec<Proceso>, String> {
    let path = Path::new(nombre_archivo);
    let file = File::open(&path).map_err(|_| "Error al abrir el archivo".to_string())?;
    let reader = BufReader::new(file);
    
    let mut procesos = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|_| "Error al leer el archivo".to_string())?;
        let campos: Vec<&str> = line.split(',').collect();
        if campos.len() == 4 {
            let nombre = campos[0].to_string();
            let arribo = campos[1].parse::<u32>().map_err(|_| "Error en el formato de arribo".to_string())?;
            let duracion = campos[2].parse::<u32>().map_err(|_| "Error en el formato de duración".to_string())?;
            let memoria_requerida = campos[3].parse::<u32>().map_err(|_| "Error en el formato de memoria requerida".to_string())?;
            
            procesos.push(Proceso {
                nombre,
                arribo,
                duracion,
                memoria_requerida,
            });
        } else {
            return Err("Formato incorrecto en el archivo de procesos".to_string());
        }
    }
    
    Ok(procesos)
}

/// Lee el archivo de simulación anterior si existe
/// Retorna un `Result` con la lista de procesos simulados
pub fn leer_archivo_simulacion(nombre_archivo: &str) -> Result<Vec<Proceso>, String> {
    // Reutilizando la misma lógica que `leer_archivo_procesos`, puedes ajustarlo si el formato es diferente
    leer_archivo_procesos(nombre_archivo)
}

/// Lista los archivos de simulación disponibles en el directorio actual
/// Retorna un vector con los nombres de los archivos
pub fn listar_archivos_simulaciones() -> Vec<String> {
    let mut archivos = Vec::new();
    
    if let Ok(entries) = fs::read_dir("./") {
        for entry in entries.flatten() {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            if file_name.starts_with("Simulacion") && file_name.ends_with(".txt") {
                archivos.push(file_name);
            }
        }
    }
    
    archivos
}

/// Guarda el estado de la simulación en un archivo de texto
pub fn guardar_simulacion(nombre_archivo: &str, contenido: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(nombre_archivo)
        .map_err(|_| "Error al abrir o crear el archivo".to_string())?;
    
    writeln!(file, "{}", contenido).map_err(|_| "Error al escribir en el archivo".to_string())?;
    
    Ok(())
}

/// Lista los archivos de procesos disponibles en el directorio actual
/// o genera un nuevo archivo de procesos si no existen
pub fn listar_archivos_o_generar() {
    let archivos = obtener_lista_archivos();

    if archivos.is_empty() {
        println!("No se encontraron archivos de procesos. Generando uno nuevo...");
        generar_archivo_procesos(5); // Ejemplo: Genera 5 procesos
    } else {
        println!("Archivos de procesos encontrados:");
        for archivo in archivos {
            println!("{}", archivo);
        }
    }
}

/// Función auxiliar para obtener una lista de archivos de procesos
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

/// Genera un archivo de procesos con la cantidad especificada
pub fn generar_archivo_procesos(cantidad_procesos: usize) {
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

/// Devuelve el siguiente número de archivo disponible
fn obtener_siguiente_numero_archivo() -> u32 {
    let archivos_existentes = obtener_lista_archivos();
    if let Some(ultimo_archivo) = archivos_existentes.last() {
        if let Some(numero) = ultimo_archivo.split('.').next() {
            return numero.parse::<u32>().unwrap_or(0) + 1;
        }
    }
    1 // Si no hay archivos, empezamos en 1
}
