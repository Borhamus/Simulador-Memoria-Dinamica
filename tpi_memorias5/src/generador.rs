// generador.rs
//
// Este módulo se encarga de la creación y manejo de archivos para los procesos y las simulaciones.
// Proporciona funciones para generar archivos de procesos y guardar los resultados de simulaciones,
// siguiendo las convenciones de nombres y directorios especificadas.

use std::fs::{self, File, create_dir_all};
use std::io::{self, Write, BufWriter};
use crate::proceso::Proceso;
use crate::simulacion::Simulacion;
use crate::confsim::Estrategia;
use chrono::Local;

/// Genera un nuevo archivo de procesos y lo guarda en la carpeta 'Procesos/'.
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

    // Generar los procesos (puedes ajustar esta lógica según tus necesidades)
    let procesos = generar_procesos_aleatorios(cantidad_procesos);

    // Asegurarse de que el directorio 'Procesos' existe
    create_dir_all("Procesos")?;

    // Obtener el siguiente número de archivo
    let numero_archivo = obtener_siguiente_numero_archivo("Procesos", "_Procesos(")?;
    let nombre_archivo = format!("Procesos/{:02}_Procesos({}).txt", numero_archivo, cantidad_procesos);

    // Crear y escribir en el archivo
    let mut archivo = File::create(&nombre_archivo)?;
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

    println!("Archivo de procesos '{}' generado exitosamente.", nombre_archivo);

    Ok(nombre_archivo)
}

/// Guarda los resultados de una simulación en un archivo en la carpeta 'Simulaciones/'.
///
/// El nombre del archivo sigue la convención: 'NN_SimulacionExitosa_(CANTIDAD)_(ESTRATEGIA).txt', donde
/// - 'NN' es un número incremental,
/// - 'CANTIDAD' es el número de procesos simulados,
/// - 'ESTRATEGIA' es la estrategia utilizada.
///
/// # Parámetros
/// - `simulacion`: Referencia a la simulación cuyos resultados se desean guardar.
///
/// # Retorna
/// - `Result<String, std::io::Error>`: Nombre del archivo generado en caso de éxito, o un error.
pub fn guardar_simulacion(simulacion: &Simulacion) -> Result<String, std::io::Error> {
    // Asegurarse de que el directorio 'Simulaciones' existe
    create_dir_all("Simulaciones")?;

    // Obtener el siguiente número de simulación
    let numero_simulacion = obtener_siguiente_numero_archivo("Simulaciones", "_SimulacionExitosa_")?;
    let cantidad_procesos = simulacion.get_procesos().len();
    let estrategia_nombre = match simulacion.get_configuracion().estrategia {
        Estrategia::FirstFit => "First-Fit",
        Estrategia::BestFit => "Best-Fit",
        Estrategia::WorstFit => "Worst-Fit",
        Estrategia::NextFit => "Next-Fit",
    };

    let nombre_archivo = format!(
        "Simulaciones/{:02}_SimulacionExitosa_({})_({}).txt",
        numero_simulacion, cantidad_procesos, estrategia_nombre
    );

    let archivo = File::create(&nombre_archivo)?;
    let mut writer = BufWriter::new(archivo);

    // Escribir las especificaciones de configuración
    writeln!(writer, "=== Especificaciones de Configuración ===")?;
    writeln!(writer, "Fecha y Hora: {}", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(writer, "Tamaño de Memoria: {}", simulacion.get_configuracion().tamanio_memoria)?;
    writeln!(writer, "Estrategia: {}", estrategia_nombre)?;
    writeln!(writer, "Tiempo de Selección: {}", simulacion.get_configuracion().tiempo_seleccion)?;
    writeln!(writer, "Tiempo de Carga: {}", simulacion.get_configuracion().tiempo_carga)?;
    writeln!(writer, "Tiempo de Liberación: {}", simulacion.get_configuracion().tiempo_liberacion)?;
    writeln!(writer, "\n=== Procesos Realizados ===")?;

    // Escribir los procesos y sus tiempos de retorno
    for proceso in simulacion.get_procesos() {
        if let Some(tiempo_retorno) = proceso.tiempo_retorno() {
            writeln!(
                writer,
                "Proceso {}: Tiempo de Retorno = {}",
                proceso.get_nombre(),
                tiempo_retorno
            )?;
        } else {
            writeln!(writer, "Proceso {}: No completado", proceso.get_nombre())?;
        }
    }

    // Escribir los resultados
    writeln!(writer, "\n=== Resultados ===")?;
    writeln!(writer, "Tiempo Medio de Retorno: {:.2}", simulacion.calcular_tiempo_medio_retorno())?;
    writeln!(
        writer,
        "Índice de Fragmentación Externa: {:.2}%",
        simulacion.calcular_fragmentacion_externa()
    )?;

    // Escribir el log de eventos
    writeln!(writer, "\n=== Log de Eventos ===")?;
    for evento in simulacion.get_log_eventos() {
        writeln!(writer, "{}", evento)?;
    }

    println!("Archivo de simulación '{}' guardado exitosamente.", nombre_archivo);

    Ok(nombre_archivo)
}

/// Obtiene el siguiente número de archivo para la nomenclatura especificada.
///
/// # Parámetros
/// - `directorio`: El directorio donde buscar los archivos.
/// - `prefijo`: El prefijo que identifica los archivos a contar.
///
/// # Retorna
/// - `Result<u32, std::io::Error>`: El siguiente número de archivo disponible.
fn obtener_siguiente_numero_archivo(directorio: &str, prefijo: &str) -> Result<u32, std::io::Error> {
    let paths = fs::read_dir(directorio)?;
    let mut max_num = 0;

    for path in paths {
        if let Ok(entry) = path {
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
