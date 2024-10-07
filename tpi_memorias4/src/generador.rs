/* generador.rs
   Este archivo se encarga de generar un archivo de procesos con datos aleatorios,
   para simular una tanda de trabajos a ser asignados en la memoria.
*/

use std::fs::{self, File};
use std::io::{Write, BufWriter};
use rand::Rng;

pub fn generar_archivo_procesos(cantidad_procesos: usize, estrategia: &str) {
    let directorio = "files";
    fs::create_dir_all(directorio).expect("No se pudo crear el directorio 'files'.");

    let nombre_archivo = format!("{}/Procesos_{}.txt", directorio, estrategia);
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
