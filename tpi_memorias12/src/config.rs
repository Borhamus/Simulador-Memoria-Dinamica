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
        println!("=============================");
        println!("Configuración de la Simulación");
        println!("=============================");
        println!("A continuación, vamos a configurar los parámetros necesarios");
        println!("para simular la asignación de memoria en un sistema multiprogramado.");
        println!("Por favor, siga las indicaciones para cada uno de los elementos.");

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
            println!("\n1. Tamaño de la memoria física disponible:");
            println!("Este valor define cuánta memoria está disponible en KB para los procesos.");
            println!("Recuerde que el sistema operativo ya utiliza parte de la memoria, así que ingrese");
            println!("un valor que represente la memoria disponible para los usuarios.");
            println!("<<advertencia: No debe crear una memoria menor al proceso mas grande que tiene>>");
            print!("Ingrese el tamaño de la memoria física disponible (en KB): ");
            io::stdout().flush().unwrap();

            let mut tamanio_memoria = String::new();
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
            println!("\n2. Selección de la Estrategia de Asignación:");
            println!("Escoja la estrategia que se utilizará para asignar los procesos a la memoria.");
            println!("Estas son las opciones disponibles:");
            println!("1) First-fit (Primer ajuste): Asigna el primer espacio libre que sea suficiente.");
            println!("2) Best-fit (Mejor ajuste): Busca la partición más pequeña posible que sea suficiente.");
            println!("3) Next-fit (Siguiente ajuste): Busca el siguiente espacio libre desde el último utilizado.");
            println!("4) Worst-fit (Peor ajuste): Busca la partición más grande disponible.");
            print!("Seleccione una opción (1-4): ");
            io::stdout().flush().unwrap();

            let mut opcion = String::new();
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
        println!("\n3. Tiempo de Selección de Partición:");
        println!("Este valor representa el tiempo que toma seleccionar la partición de memoria");
        println!("para un proceso (en milisegundos). Un valor más alto simulará un sistema más lento.");
        Config::pedir_tiempo("selección de partición")
    }

    /// Función para pedir el tiempo de carga promedio
    fn tiempo_carga_promedio() -> u32 {
        println!("\n4. Tiempo de Carga Promedio:");
        println!("Este es el tiempo que toma cargar un proceso desde la memoria secundaria a la principal.");
        println!("Ingrese un valor en milisegundos. Un valor más alto simulará un proceso de carga más lento.");
        Config::pedir_tiempo("carga promedio")
    }

    /// Función para pedir el tiempo de liberación de partición
    fn tiempo_liberacion() -> u32 {
        println!("\n5. Tiempo de Liberación de Partición:");
        println!("Este valor representa el tiempo necesario para liberar una partición de memoria");
        println!("cuando un proceso termina. Ingrese un valor en milisegundos.");
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