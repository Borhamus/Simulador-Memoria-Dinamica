use crate::estrategias::EstrategiaAsignacion;
use std::io::{self, Write};

pub struct ConfigurarSimulador {
    pub estrategia: EstrategiaAsignacion,
    pub tamanio_memoria: u32,
    pub tiempo_seleccion_particion: u32,
    pub tiempo_carga_promedio: u32,
    pub tiempo_liberacion: u32,
}

impl ConfigurarSimulador {
    pub fn nueva_configuracion() -> Self {
        let tamanio_memoria = Self::pedir_tamanio_memoria();
        let estrategia = Self::pedir_estrategia();
        let tiempo_seleccion_particion = Self::pedir_tiempo("selección de partición");
        let tiempo_carga_promedio = Self::pedir_tiempo("carga promedio");
        let tiempo_liberacion = Self::pedir_tiempo("liberación de partición");

        ConfigurarSimulador {
            estrategia,
            tamanio_memoria,
            tiempo_seleccion_particion,
            tiempo_carga_promedio,
            tiempo_liberacion,
        }
    }

    // Función para pedir el tamaño de la memoria física disponible
    fn pedir_tamanio_memoria() -> u32 {
        loop {
            let mut tamanio_memoria = String::new();
            print!("Ingrese el tamaño de la memoria física disponible en KB: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut tamanio_memoria).expect("Error al leer el tamaño de memoria");

            match tamanio_memoria.trim().parse::<u32>() {
                Ok(tamanio) if tamanio > 0 => return tamanio,
                _ => println!("Por favor ingrese un valor válido de memoria en KB."),
            }
        }
    }

    // Función para pedir la estrategia de asignación de particiones
    fn pedir_estrategia() -> EstrategiaAsignacion {
        loop {
            println!("Seleccione la estrategia de asignación de particiones:");
            println!("1) First-fit (Primer ajuste)");
            println!("2) Best-fit (Mejor ajuste)");
            println!("3) Next-fit (Siguiente ajuste)");
            println!("4) Worst-fit (Peor ajuste)");

            let mut opcion = String::new();
            print!("Seleccione una opción (1-4): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut opcion).expect("Error al leer la opción");

            match opcion.trim() {
                "1" => return EstrategiaAsignacion::FirstFit,
                "2" => return EstrategiaAsignacion::BestFit,
                "3" => return EstrategiaAsignacion::NextFit,
                "4" => return EstrategiaAsignacion::WorstFit,
                _ => println!("Opción no válida. Por favor ingrese 1, 2, 3 o 4."),
            }
        }
    }

    // Función para pedir los tiempos (se utiliza para selección de partición, carga y liberación)
    fn pedir_tiempo(nombre: &str) -> u32 {
        loop {
            let mut tiempo = String::new();
            print!("Ingrese el tiempo de {} (en ms): ", nombre);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut tiempo).expect("Error al leer el tiempo");

            match tiempo.trim().parse::<u32>() {
                Ok(tiempo) if tiempo > 0 => return tiempo,
                _ => println!("Por favor ingrese un valor válido para el tiempo en ms."),
            }
        }
    }

    // Función para guardar la configuración en el archivo de salida con formato tabla
    pub fn guardar_configuracion_en_archivo(file: &mut impl Write, configuracion: &ConfigurarSimulador) {
        writeln!(file, "| Tamaño de Memoria: {:<25} KB |", configuracion.tamanio_memoria)
            .expect("Error al escribir el tamaño de la memoria");
        writeln!(file, "| Estrategia de Asignación: {:<15} |", configuracion.estrategia)
            .expect("Error al escribir la estrategia");
        writeln!(file, "| Tiempo de Selección de Partición: {:<10} ms |", configuracion.tiempo_seleccion_particion)
            .expect("Error al escribir el tiempo de selección");
        writeln!(file, "| Tiempo de Carga Promedio: {:<15} ms |", configuracion.tiempo_carga_promedio)
            .expect("Error al escribir el tiempo de carga");
        writeln!(file, "| Tiempo de Liberación de Partición: {:<10} ms |", configuracion.tiempo_liberacion)
            .expect("Error al escribir el tiempo de liberación");
    }
}
