//simulador.rs

use crate::proceso::Proceso;
use crate::config::Config;
use crate::manipular_archivo::ManipularArchivo;

pub struct Simulador;

impl Simulador {
    pub fn ejecutar_simulacion(procesos: &Vec<Proceso>, configuracion: &Config) -> (Vec<String>, Vec<String>) {
        let mut eventos = Vec::new();
        let mut resultados = Vec::new();

        // Aquí se debe implementar la lógica de la simulación
        // Simular eventos
        for proceso in procesos {
            eventos.push(format!("Se carga el proceso {}", proceso.nombre));
            eventos.push(format!("Termina el proceso {}", proceso.nombre));
        }

        // Generar resultados de la simulación
        resultados.push("Tiempo medio de retorno: 120ms".to_string());  // Ejemplo

        // Retornar los eventos y los resultados
        (eventos, resultados)
    }
}
