//nueva_simulacion.rs

use crate::config::Config;
use crate::generador;
use crate::manipular_archivo;
use crate::proceso::Proceso;
use crate::simulador;
use std::fs;

/// Estructura principal para manejar la simulación
pub struct NuevaSimulacion;

impl NuevaSimulacion {
    pub fn nueva_tanda() {
        // Paso 1: Generar la tanda de procesos
        let procesos: Vec<Proceso> = generador::generar_procesos();
        
        // Paso 2: Configurar la simulación
        let configuracion: Config = Config::nueva_configuracion();
        
        // Paso 3: Ejecutar la simulación y obtener los eventos y resultados
        let (eventos, resultados): (Vec<String>, Vec<String>) = simulador::Simulador::ejecutar_simulacion(&procesos, &configuracion);

        // Paso 4: Llamada única a manipular_archivo para guardar todo
        manipular_archivo::ManipularArchivo::guardar_procesos_y_configuracion(
            &procesos,
            &configuracion,
            &eventos,
            &resultados
        );
    }
}
