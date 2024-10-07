// confsim.rs
//
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
    println!("Ingrese el tamaño de la memoria física disponible:");
    let tamanio_memoria = capturar_numero("Tamaño de la memoria");

    println!("Seleccione la estrategia de asignación de particiones:");
    println!("1. First-Fit");
    println!("2. Best-Fit");
    println!("3. Worst-Fit");
    println!("4. Next-Fit");
    let estrategia = capturar_estrategia();

    println!("Ingrese el tiempo de selección de partición:");
    let tiempo_seleccion = capturar_numero("Tiempo de selección");

    println!("Ingrese el tiempo de carga promedio:");
    let tiempo_carga = capturar_numero("Tiempo de carga");

    println!("Ingrese el tiempo de liberación de la partición:");
    let tiempo_liberacion = capturar_numero("Tiempo de liberación");

    ConfiguracionSimulacion {
        tamanio_memoria,
        estrategia,
        tiempo_seleccion,
        tiempo_carga,
        tiempo_liberacion,
    }
}

/// Captura un número entero positivo desde la entrada estándar.
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

/// Captura la estrategia de asignación seleccionada por el usuario.
fn capturar_estrategia() -> Estrategia {
    loop {
        print!("Opción: ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer entrada");

        match entrada.trim() {
            "1" => return Estrategia::FirstFit,
            "2" => return Estrategia::BestFit,
            "3" => return Estrategia::WorstFit,
            "4" => return Estrategia::NextFit,
            _ => println!("Opción no válida. Intente nuevamente."),
        }
    }
}
