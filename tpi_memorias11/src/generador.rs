use crate::proceso::Proceso;
use rand::Rng;
use std::io::{self, Write};

/// Función que genera un conjunto de procesos
pub fn generar_procesos() -> Vec<Proceso> {
    // Paso 1: Solicitar al usuario que ingrese el número de procesos a generar
    let num_procesos = pidenum();
    let mut procesos = generar_procesos_aleatorios(num_procesos);
    
    // Paso 2: Ordenar los procesos por tiempo de arribo
    procesos.sort_by_key(|p| p.arribo);

    println!("\nProcesos generados con éxito y ordenados por tiempo de arribo.");
    println!("A continuación, procederemos con la configuración de la simulación.\n");

    procesos
}

/// Función que pide al usuario el número de procesos
fn pidenum() -> usize {
    loop {
        // Explicación para el usuario
        println!("\n=========================================");
        println!("Generación de Procesos");
        println!("=========================================");
        println!("Vamos a generar un conjunto de procesos que serán utilizados");
        println!("en la simulación de asignación de memoria.");
        println!("Cada proceso tendrá un instante de arribo, una duración y");
        println!("una cantidad de memoria requerida.");
        println!("\nPor favor, elija cuántos procesos desea generar.");
        println!("Recuerde que este número debe ser mayor que 0.");
        print!("Ingrese el número de procesos a generar: ");
        
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer el número de procesos");

        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => {
                println!("\nSe generarán {} procesos.", num);
                return num;
            },
            _ => println!("Por favor ingrese un número válido mayor que 0."),
        }
    }
}

/// Función que genera una cantidad de procesos aleatorios
fn generar_procesos_aleatorios(num_procesos: usize) -> Vec<Proceso> {
    let mut procesos = Vec::new();
    let mut rng = rand::thread_rng();

    println!("\nGenerando procesos aleatorios...");

    for i in 1..=num_procesos {
        let nombre = format!("P{}", i);
        let arribo = rng.gen_range(0..100); // Instante de arribo aleatorio
        let duracion = rng.gen_range(5..100); // Duración aleatoria entre 5 y 100 unidades de tiempo
        let memoria_requerida = rng.gen_range(10..500); // Memoria requerida entre 10KB y 500KB

        let proceso = Proceso::new(&nombre, arribo, duracion, memoria_requerida);
        procesos.push(proceso);

        // Mostrar el proceso generado
        println!(
            "Proceso {} generado: | Nombre: {} | Instante de arribo: {} | Duración: {} | Memoria: {}KB |",
            i, nombre, arribo, duracion, memoria_requerida
        );
    }

    procesos
}
