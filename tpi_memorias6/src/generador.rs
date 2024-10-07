//generador.rs

use std::fs::{File};
use std::io::{self, Write};
use crate::proceso::Proceso;
use crate::confsim::Estrategia;

/// Genera un nuevo archivo de procesos y lo guarda en la carpeta 'file/procesos/'.
///
/// El nombre del archivo sigue la convención: 'NN_Procesos(CANTIDAD).txt', donde
/// - 'NN' es un número incremental,
/// - 'CANTIDAD' es el número de procesos contenidos en el archivo.
///
/// # Retorna
/// - `Result<String, std::io::Error>`: Nombre del archivo generado en caso de éxito, o un error.
pub fn generar_archivo_procesos() -> Result<String, std::io::Error> {
    // Solicitar al usuario la cantidad de procesos
    println!("Ingrese la cantidad de procesos que desea generar:");
    let cantidad_procesos = capturar_numero("Cantidad de procesos") as usize;

    // Generar los procesos aleatorios
    let procesos = generar_procesos_aleatorios(cantidad_procesos);

    // Obtener el siguiente número de archivo en la carpeta 'file/procesos/'
    let numero_archivo = obtener_siguiente_numero_archivo("file/procesos", "_Procesos(")?;
    let nombre_archivo = format!("file/procesos/{:02}_Procesos({}).txt", numero_archivo, cantidad_procesos);

    // Crear y escribir en el archivo
    match File::create(&nombre_archivo) {
        Ok(mut archivo) => {
            for proceso in &procesos {
                writeln!(
                    archivo,
                    "{},{},{},{}",
                    proceso.get_nombre(),
                    proceso.get_arribo(),
                    proceso.get_duracion(),
                    proceso.get_memoria_requerida()
                )?;
            }
            println!("Archivo '{}' generado exitosamente.", nombre_archivo);
            Ok(nombre_archivo)
        }
        Err(e) => {
            println!("Error al crear el archivo: {}", e);
            Err(e)
        }
    }
}

/// Guarda los resultados de la simulación en un archivo.
///
/// # Parámetros
/// - `nombre_simulacion`: Nombre del archivo de simulación.
/// - `resultados`: Vec con los eventos de la simulación a guardar.
///
/// # Retorna
/// - `Result<(), std::io::Error>`: Ok si la operación fue exitosa, o un error si falló.
pub fn guardar_simulacion(nombre_simulacion: &str, resultados: Vec<String>) -> Result<(), std::io::Error> {
    // Crear la carpeta donde se almacenarán las simulaciones, si no existe
    let carpeta_simulaciones = "file/simulacion";
    std::fs::create_dir_all(carpeta_simulaciones)?;

    // Crear y abrir el archivo para escribir los resultados
    let ruta_archivo = format!("{}/{}.txt", carpeta_simulaciones, nombre_simulacion);
    let mut archivo = File::create(&ruta_archivo)?;

    // Escribir cada línea de resultados en el archivo
    for evento in resultados {
        writeln!(archivo, "{}", evento)?;
    }

    println!("Simulación guardada exitosamente en '{}'.", ruta_archivo);
    Ok(())
}

/// Captura un número entero positivo desde la entrada estándar.
///
/// # Parámetros
/// - `mensaje`: Mensaje a mostrar al solicitar el número.
///
/// # Retorna
/// - `u32`: Número capturado.
fn capturar_numero(mensaje: &str) -> u32 {
    loop {
        print!("{}: ", mensaje);
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer entrada");

        match entrada.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Entrada inválida. Por favor, ingrese un número válido."),
        }
    }
}

/// Genera una lista de procesos aleatorios.
///
/// # Parámetros
/// - `cantidad`: Número de procesos a generar.
///
/// # Retorna
/// - `Vec<Proceso>`: Vector de procesos generados.
fn generar_procesos_aleatorios(cantidad: usize) -> Vec<Proceso> {
    use rand::Rng;
    let mut procesos = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..cantidad {
        let nombre = format!("P{}", i + 1);
        let arribo = rng.gen_range(0..50);
        let duracion = rng.gen_range(1..20);
        let memoria_requerida = rng.gen_range(10..100);

        procesos.push(Proceso::new(
            nombre,
            arribo,
            duracion,
            memoria_requerida,
        ));
    }

    procesos
}

/// Obtiene el siguiente número incremental para el archivo.
///
/// # Parámetros
/// - `carpeta`: Carpeta donde se almacenan los archivos.
/// - `prefijo`: Prefijo que los archivos deben tener.
///
/// # Retorna
/// - `u32`: Siguiente número disponible.
fn obtener_siguiente_numero_archivo(carpeta: &str, prefijo: &str) -> Result<u32, std::io::Error> {
    let mut max_num = 0;

    // Iterar sobre los archivos en la carpeta y buscar el número más alto
    for entry in std::fs::read_dir(carpeta)? {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.path().file_name() {
                if let Some(filename_str) = filename.to_str() {
                    if filename_str.contains(prefijo) {
                        let num_str: String = filename_str.chars().take_while(|c| c.is_digit(10)).collect();
                        if let Ok(num) = num_str.parse::<u32>() {
                            if num > max_num {
                                max_num = num;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(max_num + 1)
}
