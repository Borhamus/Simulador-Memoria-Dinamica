/* confsim.rs
   Este archivo contiene la configuración de la simulación, incluyendo
   el tamaño de la memoria, la estrategia de asignación, y los tiempos
   de selección, carga y liberación.
*/

use std::io::{self, Write};  // Agregado para manejar la entrada/salida

pub struct ConfiguracionSimulacion {
    pub tamanio_memoria: u32,
    pub estrategia: String,
    pub tiempo_seleccion: u32,
    pub tiempo_carga: u32,
    pub tiempo_liberacion: u32,
}

// Función para capturar la configuración de la simulación
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

fn capturar_numero(mensaje: &str) -> u32 {
    print!("{}: ", mensaje);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer entrada");
    entrada.trim().parse().expect("Debe ingresar un número válido.")
}

fn capturar_estrategia() -> String {
    loop {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Error al leer entrada");
        match entrada.trim() {
            "1" => return "First-Fit".to_string(),
            "2" => return "Best-Fit".to_string(),
            "3" => return "Worst-Fit".to_string(),
            "4" => return "Next-Fit".to_string(),
            _ => println!("Opción no válida. Intente nuevamente."),
        }
    }
}
