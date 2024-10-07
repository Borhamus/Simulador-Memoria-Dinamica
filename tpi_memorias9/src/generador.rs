use crate::proceso::Proceso;
use rand::Rng;
use std::io::{self, Write};

pub struct Generador;

impl Generador {
    /// Función para generar un número de procesos aleatorios
    pub fn generar_procesos() -> Vec<Proceso> {
        let num_procesos = Self::pedir_numero_procesos();
        Self::generar_procesos_aleatorios(num_procesos)
    }

    // Función para pedir al usuario el número de procesos
    fn pedir_numero_procesos() -> usize {
        loop {
            let mut input = String::new();
            print!("Ingrese la cantidad de procesos que desea generar: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Error al leer la entrada");

            match input.trim().parse::<usize>() {
                Ok(num) if num > 0 => return num,
                _ => println!("Por favor, ingrese un número válido mayor que 0."),
            }
        }
    }

    // Generar procesos aleatorios con datos lógicos
    fn generar_procesos_aleatorios(num_procesos: usize) -> Vec<Proceso> {
        let mut procesos = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 1..=num_procesos {
            let nombre = format!("P{}", i);
            let arribo = rng.gen_range(0..100); // Instante de arribo aleatorio
            let duracion = rng.gen_range(10..100); // Duración aleatoria entre 10 y 100 unidades
            let memoria_requerida = rng.gen_range(10..500); // Memoria requerida entre 10KB y 500KB

            let proceso = Proceso::new(&nombre, arribo, duracion, memoria_requerida);
            procesos.push(proceso);
        }

        procesos
    }
}
