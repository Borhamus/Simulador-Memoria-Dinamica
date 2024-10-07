//memoria.rs

use crate::estrategias::{first_fit, best_fit, worst_fit, next_fit, EstrategiaAsignacion}; // Importamos EstrategiaAsignacion
use crate::proceso::Proceso; // Importamos Proceso
use crate::particion::{Particion, EstadoParticion};


pub struct Memoria {
    pub particiones: Vec<Particion>,
    pub tamanio_total: u32,
    pub ultima_asignada: Option<usize>,
}

impl Memoria {
    pub fn new(tamanio: u32) -> Memoria {
        Memoria {
            particiones: vec![Particion {
                id: 0,
                direccion_comienzo: 0,
                tamanio,
                estado: EstadoParticion::Libre,
            }],
            tamanio_total: tamanio,
            ultima_asignada: None,
        }
    }

    // Método de asignación según estrategia
    pub fn asignar_proceso(&mut self, proceso: &Proceso, estrategia: EstrategiaAsignacion) -> Option<usize> {
        match estrategia {
            EstrategiaAsignacion::FirstFit => first_fit(&mut self.particiones, proceso),
            EstrategiaAsignacion::BestFit => best_fit(&mut self.particiones, proceso),
            EstrategiaAsignacion::WorstFit => worst_fit(&mut self.particiones, proceso),
            EstrategiaAsignacion::NextFit => {
                let resultado = next_fit(&mut self.particiones, proceso, &mut self.ultima_asignada);
                resultado
            }
        }
    }

    pub fn liberar_particion(&mut self, proceso: &Proceso) {
        for particion in self.particiones.iter_mut() {
            if let EstadoParticion::Ocupada(nombre_proceso) = &particion.estado {
                if nombre_proceso == &proceso.nombre {
                    particion.estado = EstadoParticion::Libre;
                    break;
                }
            }
        }
    }

    pub fn calcular_fragmentacion_externa(&self) -> f32 {
        let memoria_libre_total: u32 = self.particiones
            .iter()
            .filter(|p| matches!(p.estado, EstadoParticion::Libre))
            .map(|p| p.tamanio)
            .sum();

        // Aquí puedes agregar la lógica para calcular la memoria no asignable

        0.0 // Retornamos 0.0 por simplicidad
    }
}
