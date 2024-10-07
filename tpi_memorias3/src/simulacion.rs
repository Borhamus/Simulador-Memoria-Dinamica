// simulacion.rs


// simulacion.rs

use crate::proceso::Proceso;
use crate::particion::Particion;
use crate::confsim::ConfiguracionSimulacion;

pub fn ejecutar_simulacion(
    config: &ConfiguracionSimulacion,
    mut procesos: Vec<Proceso>,
    mut particiones: Vec<Particion>,
) {
    // Iteramos por referencia a procesos en lugar de moverlo
    for proceso in &procesos {
        // Lógica de simulación...
        println!("Simulando proceso: {}", proceso.nombre);
        // Más lógica...
    }

    // Podemos usar 'procesos' aquí sin problemas porque no se movió
    calcular_indicadores(&procesos, &particiones);
}

fn calcular_indicadores(procesos: &[Proceso], particiones: &[Particion]) {
    // Implementación del cálculo de indicadores
    println!("Calculando indicadores...");
}
