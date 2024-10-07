use crate::particion::Particion;
use crate::proceso::Proceso;
use crate::estrategias::EstrategiaAsignacion;

pub struct Memoria {
    pub particiones: Vec<Particion>,
    pub ultima_asignada: usize, // Para estrategias como Next Fit
}

impl Memoria {
    /// Crea una nueva memoria con un conjunto de particiones
    pub fn new(particiones: Vec<Particion>) -> Memoria {
        Memoria {
            particiones,
            ultima_asignada: 0, // Inicializamos en 0 la última asignada
        }
    }

    /// Asigna un proceso a una partición según la estrategia dada
    pub fn asignar_proceso(
        &mut self,
        proceso: &Proceso,
        estrategia: EstrategiaAsignacion,
    ) -> Option<usize> {
        // Llamamos a la estrategia de asignación de procesos
        estrategia.asignar_proceso(&mut self.particiones, proceso, &mut self.ultima_asignada)
    }

    /// Libera una partición dado su índice
    pub fn liberar_particion(&mut self, indice_particion: usize) {
        if let Some(particion) = self.particiones.get_mut(indice_particion) {
            particion.liberar(); // Marcamos la partición como libre
        }
    }

    /// Calcula la fragmentación externa en la memoria
    /// Fragmentación externa es el porcentaje de espacio libre que no puede ser utilizado porque está dividido
    pub fn calcular_fragmentacion_externa(&self) -> f64 {
        let mut total_espacio_libre = 0;
        let mut mayor_espacio_libre = 0;

        // Recorremos todas las particiones para calcular espacio libre
        for particion in &self.particiones {
            let espacio_libre = particion.espacio_libre();
            if espacio_libre > 0 {
                total_espacio_libre += espacio_libre;
                if espacio_libre > mayor_espacio_libre {
                    mayor_espacio_libre = espacio_libre;
                }
            }
        }

        // Si no hay espacio libre, no hay fragmentación externa
        if total_espacio_libre == 0 {
            return 0.0;
        }

        // La fragmentación externa es la diferencia entre el total libre y el mayor bloque libre
        let fragmentacion_externa = (total_espacio_libre - mayor_espacio_libre) as f64 / total_espacio_libre as f64;
        fragmentacion_externa
    }

    /// Muestra el estado actual de la memoria (particiones)
    pub fn mostrar_estado(&self) {
        println!("{:<10} {:<15} {:<10} {:<10}", "ID", "Dirección", "Tamaño", "Estado");
        println!("---------------------------------------------------");
        for particion in &self.particiones {
            let estado = match &particion.estado {
                crate::particion::EstadoParticion::Libre => "Libre".to_string(),
                crate::particion::EstadoParticion::Ocupada(nombre_proceso) => format!("Ocupada ({})", nombre_proceso),
            };
            println!(
                "{:<10} {:<15} {:<10} {:<10}",
                particion.id, particion.direccion_comienzo, particion.tamanio, estado
            );
        }
    }
}
