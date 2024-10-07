use crate::proceso::Proceso;
use crate::config::Config;
use std::fs::{File};
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct ManipularArchivo;

impl ManipularArchivo {
    /// Función para crear el archivo de la simulación con procesos, configuración, eventos y resultados
    pub fn crear_archivo_simulacion(
        procesos: &[Proceso],
        configuracion: &Config,
        eventos: &[String],
        resultados: &[String],
    ) -> String {
        // Contar la cantidad de archivos existentes en "files" para definir el número xx del archivo
        let files_path = Path::new("files");
        let archivo_numero = if files_path.exists() {
            std::fs::read_dir(files_path).unwrap().count() + 1
        } else {
            std::fs::create_dir_all(files_path).expect("No se pudo crear la carpeta 'files'");
            1
        };

        // Crear el nombre del archivo con el formato `XX_procesos(YY)_estrategia(ZZ).txt`
        let nombre_archivo = format!(
            "files/{}_procesos({})_estrategia({:?}).txt",
            archivo_numero,
            procesos.len(),
            configuracion.estrategia
        );
        let path = Path::new(&nombre_archivo);

        // Crear o abrir el archivo para escribir
        let file = File::create(&path).expect("No se pudo crear el archivo");
        let mut writer = BufWriter::new(file);

        // Escribir la tabla de procesos
        writeln!(writer, "Procesos de la Tanda:").expect("Error al escribir en el archivo");
        writeln!(writer, "-------------------------------------------").expect("Error al escribir línea divisora");
        writeln!(writer, "| Nombre  | Arribo | Duración | Memoria |").expect("Error al escribir encabezado");
        writeln!(writer, "-------------------------------------------").expect("Error al escribir línea divisora");

        for proceso in procesos {
            writeln!(
                writer,
                "| {:<7} | {:<6} | {:<8} | {:<7} |",
                proceso.nombre, proceso.arribo, proceso.duracion, proceso.memoria_requerida
            ).expect("Error al escribir los procesos de la tanda");
        }
        writeln!(writer, "-------------------------------------------").expect("Error al escribir línea divisora");

        // Escribir la tabla de configuración del simulador
        writeln!(writer, "\nConfiguración del Simulador:").expect("Error al escribir encabezado de configuración");
        writeln!(writer, "-------------------------------------------").expect("Error al escribir línea divisora");
        writeln!(writer, "Tamaño de memoria: {} KB", configuracion.tamanio_memoria).expect("Error al escribir configuración");
        writeln!(writer, "Estrategia de asignación: {:?}", configuracion.estrategia).expect("Error al escribir configuración");
        writeln!(writer, "Tiempo de selección de partición: {} ms", configuracion.tiempo_seleccion).expect("Error al escribir configuración");
        writeln!(writer, "Tiempo de carga promedio: {} ms", configuracion.tiempo_carga).expect("Error al escribir configuración");
        writeln!(writer, "Tiempo de liberación de partición: {} ms", configuracion.tiempo_liberacion).expect("Error al escribir configuración");
        writeln!(writer, "-------------------------------------------").expect("Error al escribir línea divisora");

        // Escribir la tabla de eventos
        writeln!(writer, "\nEventos de la Simulación:").expect("Error al escribir encabezado de eventos");
        writeln!(writer, "-------------------------------------------").expect("Error al escribir línea divisora");
        for evento in eventos {
            writeln!(writer, "{}", evento).expect("Error al escribir evento");
        }
        writeln!(writer, "-------------------------------------------").expect("Error al escribir línea divisora");

        // Escribir la tabla de resultados
        writeln!(writer, "\nResultados de la Simulación:").expect("Error al escribir encabezado de resultados");
        writeln!(writer, "-------------------------------------------").expect("Error al escribir línea divisora");
        for resultado in resultados {
            writeln!(writer, "{}", resultado).expect("Error al escribir resultado");
        }
        writeln!(writer, "-------------------------------------------").expect("Error al escribir línea divisora");

        // Retornar el nombre del archivo creado
        nombre_archivo
    }
}
