/* simulacion.rs
   Este archivo contiene la lógica de la simulación. Ejecuta la asignación
   de procesos a particiones según la estrategia seleccionada y genera
   el archivo de salida con los resultados de la simulación.
*/


use crate::proceso::Proceso;
use crate::particion::Particion;
use crate::confsim::ConfiguracionSimulacion;
use crate::generador::generar_archivo_procesos;
use crate::estrategias::{FirstFit, BestFit, NextFit, WorstFit, EstrategiaAsignacion};

/// Ejecuta la simulación de acuerdo con la configuración proporcionada.
pub fn ejecutar_simulacion(config: &ConfiguracionSimulacion, procesos: Vec<Proceso>) {
    let mut particiones: Vec<Particion> = Vec::new(); // Inicializamos las particiones
    let estrategia = obtener_estrategia(&config.estrategia);

    // Simulamos la asignación de procesos a particiones según la estrategia elegida
    for proceso in &procesos {
        let resultado = estrategia.asignar(proceso, &mut particiones);
        if let Some(particion_index) = resultado {
            println!(
                "Proceso {} asignado a la partición {}.",
                proceso.get_nombre(),
                particion_index
            );
        } else {
            println!(
                "No se pudo asignar el proceso {} a ninguna partición.",
                proceso.get_nombre()
            );
        }
    }

    // Guardamos los resultados en un archivo al finalizar la simulación
    generar_archivo_procesos(procesos.len(), &config.estrategia);
}

/// Obtiene la estrategia de asignación según el nombre.
fn obtener_estrategia(nombre: &str) -> Box<dyn EstrategiaAsignacion> {
    match nombre {
        "first-fit" => Box::new(FirstFit),
        "best-fit" => Box::new(BestFit),
        "next-fit" => Box::new(NextFit { ultimo_indice: 0 }),
        "worst-fit" => Box::new(WorstFit),
        _ => panic!("Estrategia no válida: {}", nombre),
    }
}
