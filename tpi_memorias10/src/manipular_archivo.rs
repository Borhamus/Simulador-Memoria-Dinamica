//manipular_archivos.rs

use crate::proceso::Proceso;
use crate::config::Config;
use crate::particion::{Particion, EstadoParticion};  
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, BufReader, BufRead};
use std::path::Path;

pub struct ManipularArchivo;

impl ManipularArchivo {
    /// Función para leer "Procesos de la Tanda" y "Datos de la Configuración del Simulador", crear un archivo con nombre según el formato y guardar la información
    pub fn guardar_procesos_y_configuracion(
        procesos: &[Proceso],
        configuracion: &Config,
    ) -> String {
        // Crear el nombre del archivo con el formato `XX_procesos(YY)_estrategia(ZZ).txt`
        let nombre_archivo = format!("files/{}_procesos({})_estrategia({}).txt", "simulacion", num_procesos, estrategia);
        let path = Path::new(&nombre_archivo);

        // Crear o abrir el archivo para escribir
        let file = File::create(&path).expect("No se pudo crear el archivo");
        let mut writer = BufWriter::new(file);

        // Escribir la tabla explicativa para los procesos
        writeln!(writer, "Procesos de la Tanda:").expect("Error al escribir en el archivo");
        writeln!(writer, "------------------------------").expect("Error al escribir línea divisora");
        writeln!(writer, "| Nombre  | Arribo | Duración | Memoria |").expect("Error al escribir encabezado");
        writeln!(writer, "------------------------------").expect("Error al escribir línea divisora");

        // Escribir los procesos, cada uno en una nueva línea
        for proceso in procesos.split(';') {
            if !proceso.trim().is_empty() {
                writeln!(writer, "{};", proceso).expect("Error al escribir los procesos de la tanda");
            }
        }

        // Guardar los datos de la configuración del simulador en el archivo
        writeln!(writer, "\nDatos de la Configuración del Simulador:").expect("Error al escribir configuración");
        writeln!(writer, "{}", configuracion).expect("Error al escribir la configuración del simulador");

        // Retornar el nombre del archivo para su uso posterior
        nombre_archivo
    }

    /// Función para cargar los procesos y particiones desde el archivo
    pub fn cargar_datos_desde_archivo(nombre_archivo: &str) -> (Vec<Proceso>, Vec<Particion>) {
        let mut procesos = Vec::new();
        let mut particiones = Vec::new();
        let path = Path::new(nombre_archivo);
        let file = File::open(&path).expect("No se pudo abrir el archivo");

        let reader = BufReader::new(file);
        let mut leyendo_procesos = false;
        let mut leyendo_memoria = false;

        // Leer el archivo línea por línea
        for line in reader.lines() {
            let line = line.expect("Error al leer la línea");

            // Detección de las secciones para empezar a leer procesos y memoria
            if line.contains("Procesos de la Tanda:") {
                leyendo_procesos = true;
                leyendo_memoria = false;
                continue;
            }
            if line.contains("Particiones:") {
                leyendo_procesos = false;
                leyendo_memoria = true;
                continue;
            }

            // Cargar los procesos (formato: "proceso1,arribo,duración,memoria;")
            if leyendo_procesos && line.contains(',') {
                let procesos_linea = line.split(';').collect::<Vec<&str>>();
                for proceso_str in procesos_linea {
                    let datos: Vec<&str> = proceso_str.split(',').collect();
                    if datos.len() == 4 {
                        let nombre = datos[0].to_string();
                        let arribo = datos[1].parse::<usize>().expect("Error al parsear arribo");
                        let duracion = datos[2].parse::<usize>().expect("Error al parsear duración");
                        let memoria_requerida = datos[3].parse::<u32>().expect("Error al parsear memoria");

                        let proceso = Proceso::new(&nombre, arribo, duracion, memoria_requerida);
                        procesos.push(proceso);
                    }
                }
            }

            // Cargar la memoria física (particiones)
            if leyendo_memoria && line.contains('|') {
                let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
                if parts.len() == 5 {
                    let id = parts[1].parse::<usize>().expect("Error al parsear ID");
                    let direccion_comienzo = parts[2].parse::<u32>().expect("Error al parsear dirección");
                    let tamanio = parts[3].parse::<u32>().expect("Error al parsear tamaño");
                    let estado = match parts[4] {
                        "Libre" => EstadoParticion::Libre,
                        _ => EstadoParticion::Ocupada(parts[4].to_string()),
                    };

                    let particion = Particion {
                        id,
                        direccion_comienzo,
                        tamanio,
                        estado,
                    };
                    particiones.push(particion);
                }
            }
        }

        (procesos, particiones)
    }

    /// Función para guardar eventos y resultados en el archivo
    pub fn guardar_eventos_y_resultados(
        nombre_archivo: &str,
        eventos: Vec<String>,
        resultados: Vec<String>,
    ) {
        let path = Path::new(nombre_archivo);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&path)
            .expect("No se pudo abrir el archivo para escribir eventos y resultados");

        // Guardar los eventos
        writeln!(file, "\nEventos de la simulación:").expect("Error al escribir encabezado de eventos");
        for evento in eventos {
            writeln!(file, "{}", evento).expect("Error al escribir evento");
        }

        // Guardar los resultados
        writeln!(file, "\nResultados finales de la simulación:").expect("Error al escribir encabezado de resultados");
        for resultado in resultados {
            writeln!(file, "{}", resultado).expect("Error al escribir resultado");
        }
    }
}
