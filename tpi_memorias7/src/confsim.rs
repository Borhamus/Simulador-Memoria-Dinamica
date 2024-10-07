
// confsim.rs

// Este archivo contiene la configuración de la simulación, incluyendo
// el tamaño de la memoria, la estrategia de asignación, y los tiempos
// de selección, carga y liberación.

use std::io::{self, Write};

#[derive(Clone)]
/// Representa la configuración de la simulación.
pub struct ConfiguracionSimulacion {
    pub tamanio_memoria: u32,
    pub estrategia: Estrategia,
    pub tiempo_seleccion: u32,
    pub tiempo_carga: u32,
    pub tiempo_liberacion: u32,
}

/// Enumera las estrategias de asignación de memoria disponibles.
#[derive(Debug, Clone, Copy)]
pub enum Estrategia {
    FirstFit,
    BestFit,
    WorstFit,
    NextFit,
}

impl std::fmt::Display for Estrategia {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Estrategia::FirstFit => write!(f, "First-Fit"),
            Estrategia::BestFit => write!(f, "Best-Fit"),
            Estrategia::WorstFit => write!(f, "Worst-Fit"),
            Estrategia::NextFit => write!(f, "Next-Fit"),
        }
    }
}

/// Captura la configuración de la simulación a través de la entrada del usuario.
pub fn capturar_configuracion() -> ConfiguracionSimulacion {
    println!("Ingrese el tamaño de la memoria física disponible (en kilobytes):");
    println!("Este valor representa la cantidad de memoria disponible para los trabajos en la simulación, excluyendo la utilizada por el sistema operativo.");
    println!("Por ejemplo, si tiene 16 MB de memoria disponible, ingrese 16384 (kilobytes).");
    let tamanio_memoria = capturar_numero("Tamaño de la memoria");

    println!("Seleccione la estrategia de asignación de particiones:");
    println!("First-Fit: Asigna el trabajo a la primera partición libre que sea lo suficientemente grande.");
    println!("Best-Fit: Asigna el trabajo a la partición libre más pequeña que sea lo suficientemente grande.");
    println!("Worst-Fit: Asigna el trabajo a la partición libre más grande.");
    println!("Next-Fit: Similar a First-Fit, pero comienza la búsqueda desde la última partición utilizada.");
    let estrategia = capturar_estrategia();

    println!("Ingrese el tiempo de selección (en unidades de tiempo):");
    println!("Este valor representa el tiempo que tarda el sistema en elegir una partición para el siguiente proceso.");
    println!("Un valor mayor simula un sistema más lento en tomar decisiones de asignación de memoria.");
    let tiempo_seleccion = capturar_numero("Tiempo de selección");

    println!("Ingrese el tiempo de carga (en unidades de tiempo):");
    println!("Este valor representa el tiempo promedio que tarda el sistema en cargar un proceso desde la memoria secundaria a la memoria principal.");
    println!("Un valor mayor simula un sistema con un almacenamiento secundario más lento.");
    let tiempo_carga = capturar_numero("Tiempo de carga");

    println!("Ingrese el tiempo de liberación (en unidades de tiempo):");
    println!("Este valor representa el tiempo que tarda el sistema en liberar una partición de memoria cuando un proceso termina.");
    println!("Un valor mayor simula un sistema con operaciones de liberación de memoria más lentas.");
    let tiempo_liberacion = capturar_numero("Tiempo de liberación");

    ConfiguracionSimulacion {
        tamanio_memoria,
        estrategia,
        tiempo_seleccion,
        tiempo_carga,
        tiempo_liberacion,
    }
}

/// Función auxiliar para capturar un número del usuario.
fn capturar_numero(mensaje: &str) -> u32 {
    print!("{}: ", mensaje);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().parse().unwrap_or(0)
}

/// Función auxiliar para capturar la estrategia de asignación seleccionada por el usuario.
fn capturar_estrategia() -> Estrategia {
    loop {
        let opcion = capturar_numero("Ingrese el número de la estrategia (1-4)");
        match opcion {
            1 => return Estrategia::FirstFit,
            2 => return Estrategia::BestFit,
            3 => return Estrategia::WorstFit,
            4 => return Estrategia::NextFit,
            _ => println!("Opción no válida. Por favor, seleccione una estrategia entre 1 y 4."),
        }
    }
}
