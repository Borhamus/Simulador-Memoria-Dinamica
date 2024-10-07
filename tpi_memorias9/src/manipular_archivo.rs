use crate::proceso::Proceso;
use crate::configuracion::ConfigurarSimulador;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

/// Función para guardar los procesos generados y la configuración en el archivo
pub fn guardar_procesos_y_configuracion(
    nombre_archivo: &str,
    procesos: &[Proceso],
    configuracion: &ConfigurarSimulador,
) {
    let path = Path::new(nombre_archivo);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect("No se pudo crear el archivo");

    // 1ra parte: Tabla de procesos (con la estructura de tabla solicitada)
    writeln!(file, "Procesos Simulados:").expect("Error al escribir en el archivo");
    writeln!(file, "|-----------------------------------------|").expect("Error al escribir encabezado");
    writeln!(file, "| Nombre  | Arribo  | Duración | Memoria  |").expect("Error al escribir encabezado");
    writeln!(file, "|---------|---------|----------|----------|").expect("Error al escribir encabezado");

    for proceso in procesos {
        writeln!(
            file,
            "| {:<7} | {:<7} | {:<8} | {:<8} |",
            proceso.get_nombre(),
            proceso.get_arribo(),
            proceso.get_duracion(),
            proceso.memoria_requerida
        )
        .expect("Error al escribir proceso");
    }

    // 2da parte: Configuración (llamamos a la función en configuracion.rs)
    writeln!(file, "\n\nConfiguración de la Simulación:").expect("Error al escribir configuración");
    writeln!(file, "-------------------------------").expect("Error al escribir encabezado");
    crate::configuracion::guardar_configuracion_en_archivo(&mut file, configuracion);
}

/// Función para agregar eventos y resultados al archivo
pub fn agregar_eventos_y_resultados(
    nombre_archivo: &str,
    eventos: String,
    resultados: String,
) {
    let path = Path::new(nombre_archivo);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path)
        .expect("No se pudo abrir el archivo para agregar eventos");

    // 3ra parte: Eventos (también con la estructura solicitada)
    writeln!(file, "\n\nEventos de la Simulación:").expect("Error al escribir eventos");
    writeln!(file, "|---------------------------------------------|").expect("Error al escribir encabezado");
    writeln!(file, "| Tiempo  | Proceso  | Partición | Evento     |").expect("Error al escribir encabezado");
    writeln!(file, "|---------|----------|-----------|------------|").expect("Error al escribir encabezado");
    writeln!(file, "{}", eventos).expect("Error al escribir los eventos");

    // 4ta parte: Resultados
    writeln!(file, "\n\nResultados de la Simulación:").expect("Error al escribir resultados");
    writeln!(file, "----------------------------").expect("Error al escribir encabezado");
    writeln!(file, "{}", resultados).expect("Error al escribir los resultados");
}

/// Función para mostrar el archivo completo por pantalla
pub fn mostrar_archivo_completo(nombre_archivo: &str) {
    let path = Path::new(nombre_archivo);
    let file = OpenOptions::new().read(true).open(&path).expect("No se pudo abrir el archivo");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.expect("Error al leer la línea"));
    }
}
