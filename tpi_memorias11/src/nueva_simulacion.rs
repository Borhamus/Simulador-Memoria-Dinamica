use crate::config::Config;
use crate::generador;
use crate::manipular_archivo;
use crate::proceso::Proceso;
use crate::simulador;

/// Estructura principal para manejar la simulación
pub struct NuevaSimulacion;

impl NuevaSimulacion {
    pub fn nueva_tanda() {
        // Paso 1: Definir las variables antes de usarlas: procesos, configuración, eventos, resultados.
        let procesos: Vec<Proceso>;
        let configuracion: Config;
        let mut eventos: Vec<String> = Vec::new(); // Se usará directamente
        let mut resultados: Vec<String> = Vec::new(); // Se usará directamente

        // Paso 2: Generar la tanda de procesos ordenada por instancia de arribo.
        procesos = generador::generar_procesos();
        
        // Paso 3: Configurar la simulación
        configuracion = Config::nueva_configuracion();
        
        // Paso 4: Ejecutar la simulación, que llenará los eventos y resultados
        simulador::Simulador::ejecutar_simulacion(&procesos, &configuracion, &mut eventos, &mut resultados);

        // Paso 5: Llamada única a manipular_archivo para guardar todo
        manipular_archivo::ManipularArchivo::crear_archivo_simulacion(
            &procesos,
            &configuracion,
            &eventos,
            &resultados
        );
    }
}
