use crate::estrategias::EstrategiaAsignacion;  // Asegúrate de que está importando correctamente
use std::io::{self, Write};

pub struct Config {
    pub estrategia: EstrategiaAsignacion,  // Usamos la estrategia de asignación definida en estrategias.rs
    pub tamanio_memoria: u32,
    pub tiempo_seleccion: u32,
    pub tiempo_carga: u32,
    pub tiempo_liberacion: u32,
}

impl Config {
    /// Función principal para obtener una nueva configuración desde el usuario
    pub fn nueva_configuracion() -> Config {
        // Obtener los valores para la configuración
        let tamanio_memoria = Config::set_tamanio_memoria();
        let estrategia = Config::set_estrategia();  // Selecciona la estrategia
        let tiempo_seleccion = Config::tiempo_seleccion();
        let tiempo_carga = Config::tiempo_carga_promedio();
        let tiempo_liberacion = Config::tiempo_liberacion();

        // Retorna una nueva configuración con los valores obtenidos
        Config {
            estrategia,
            tamanio_memoria,
            tiempo_seleccion,
            tiempo_carga,
            tiempo_liberacion,
        }
    }

    /// Función para pedir el tamaño de la memoria física disponible
    fn set_tamanio_memoria() -> u32 {
        loop {
            let mut tamanio_memoria = String::new();
            print!("Ingrese el tamaño de la memoria física disponible (en KB): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut tamanio_memoria).expect("Error al leer el tamaño de la memoria");

            match tamanio_memoria.trim().parse::<u32>() {
                Ok(tamanio) if tamanio > 0 => return tamanio,
                _ => println!("Por favor ingrese un tamaño de memoria válido (número mayor que 0)."),
            }
        }
    }

    /// Función para seleccionar la estrategia de asignación
    fn set_estrategia() -> EstrategiaAsignacion {
        loop {
            println!("Seleccione la estrategia de asignación de memoria:");
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

    /// Función para pedir el tiempo de selección de partición
    fn tiempo_seleccion() -> u32 {
        Config::pedir_tiempo("selección de partición")
    }

    /// Función para pedir el tiempo de carga promedio
    fn tiempo_carga_promedio() -> u32 {
        Config::pedir_tiempo("carga promedio")
    }

    /// Función para pedir el tiempo de liberación de partición
    fn tiempo_liberacion() -> u32 {
        Config::pedir_tiempo("liberación de partición")
    }

    /// Función genérica para pedir tiempos (selección, carga, liberación)
    fn pedir_tiempo(tipo: &str) -> u32 {
        loop {
            let mut tiempo = String::new();
            print!("Ingrese el tiempo de {} (en milisegundos): ", tipo);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut tiempo).expect("Error al leer el tiempo");

            match tiempo.trim().parse::<u32>() {
                Ok(tiempo) if tiempo > 0 => return tiempo,
                _ => println!("Por favor ingrese un valor válido para el tiempo (en milisegundos)."),
            }
        }
    }
}
