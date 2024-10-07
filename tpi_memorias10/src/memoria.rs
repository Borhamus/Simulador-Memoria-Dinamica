//memoria.rs


use crate::proceso::Proceso;
use crate::estrategias::EstrategiaAsignacion;
use std::rc::Rc;
use std::cell::RefCell;

// Nodo que representa una partición de memoria en la lista enlazada
pub struct Particion {
    pub id: usize,
    pub direccion_comienzo: u32,
    pub tamanio: u32,
    pub estado: EstadoParticion,
    pub siguiente: Option<Rc<RefCell<Particion>>>,  // Referencia a la siguiente partición
}

#[derive(Debug, Clone)]
pub enum EstadoParticion {
    Libre,
    Ocupada(String),  // Almacena el nombre del proceso
}

impl Particion {
    /// Crea una nueva partición libre
    pub fn new(id: usize, direccion_comienzo: u32, tamanio: u32) -> Particion {
        Particion {
            id,
            direccion_comienzo,
            tamanio,
            estado: EstadoParticion::Libre,
            siguiente: None,  // Sin siguiente por defecto
        }
    }

    /// Marca la partición como ocupada por un proceso
    pub fn ocupar(&mut self, nombre_proceso: String) {
        self.estado = EstadoParticion::Ocupada(nombre_proceso);
    }

    /// Libera la partición
    pub fn liberar(&mut self) {
        self.estado = EstadoParticion::Libre;
    }

    /// Devuelve el espacio libre en la partición
    pub fn espacio_libre(&self) -> u32 {
        match self.estado {
            EstadoParticion::Libre => self.tamanio,
            EstadoParticion::Ocupada(_) => 0,
        }
    }
}

// Estructura que representa la memoria como una lista enlazada de particiones
pub struct Memoria {
    pub primer_particion: Option<Rc<RefCell<Particion>>>,  // Referencia a la primera partición
    pub ultima_asignada: Option<Rc<RefCell<Particion>>>,   // Usado para Next Fit
}

impl Memoria {
    /// Crea una nueva memoria con una lista de particiones
    pub fn new(primer_particion: Option<Rc<RefCell<Particion>>>) -> Memoria {
        Memoria {
            primer_particion,
            ultima_asignada: None,  // Inicializamos sin asignar
        }
    }

    /// Asigna una tanda de procesos a las particiones según la estrategia dada
    pub fn asignar_tanda(
        &mut self,
        procesos: &Vec<Proceso>,
        estrategia: EstrategiaAsignacion,
    ) -> Vec<Option<usize>> {
        let mut asignaciones = Vec::new();
        let mut ultima_asignada = self.ultima_asignada.clone();

        for proceso in procesos {
            let asignacion = match estrategia {
                EstrategiaAsignacion::FirstFit => self.first_fit(proceso),
                EstrategiaAsignacion::BestFit => self.best_fit(proceso),
                EstrategiaAsignacion::NextFit => self.next_fit(proceso, &mut ultima_asignada),
                EstrategiaAsignacion::WorstFit => self.worst_fit(proceso),
            };
            asignaciones.push(asignacion);
        }

        self.ultima_asignada = ultima_asignada;  // Actualizamos la última asignada (para NextFit)
        asignaciones
    }

    /// Implementación de First Fit
    fn first_fit(&mut self, proceso: &Proceso) -> Option<usize> {
        let mut particion_actual = self.primer_particion.clone();
        while let Some(particion) = particion_actual {
            let mut particion_mut = particion.borrow_mut();
            if particion_mut.espacio_libre() >= proceso.memoria_requerida {
                particion_mut.ocupar(proceso.nombre.clone());
                return Some(particion_mut.id);
            }
            particion_actual = particion_mut.siguiente.clone();
        }
        None
    }

    /// Implementación de Best Fit
    fn best_fit(&mut self, proceso: &Proceso) -> Option<usize> {
        let mut particion_actual = self.primer_particion.clone();
        let mut mejor_particion: Option<Rc<RefCell<Particion>>> = None;
        let mut menor_diferencia: u32 = u32::MAX;

        while let Some(particion) = particion_actual {
            let particion_ref = particion.borrow();
            let espacio_libre = particion_ref.espacio_libre();
            if espacio_libre >= proceso.memoria_requerida && espacio_libre < menor_diferencia {
                mejor_particion = Some(particion.clone());
                menor_diferencia = espacio_libre;
            }
            particion_actual = particion_ref.siguiente.clone();
        }

        if let Some(mejor_particion) = mejor_particion {
            mejor_particion.borrow_mut().ocupar(proceso.nombre.clone());
            return Some(mejor_particion.borrow().id);
        }

        None
    }

    /// Implementación de Next Fit
    fn next_fit(&mut self, proceso: &Proceso, ultima_asignada: &mut Option<Rc<RefCell<Particion>>>) -> Option<usize> {
        let mut particion_actual = ultima_asignada.clone().or_else(|| self.primer_particion.clone());

        while let Some(particion) = particion_actual {
            let mut particion_mut = particion.borrow_mut();
            if particion_mut.espacio_libre() >= proceso.memoria_requerida {
                particion_mut.ocupar(proceso.nombre.clone());
                *ultima_asignada = Some(particion.clone());
                return Some(particion_mut.id);
            }
            particion_actual = particion_mut.siguiente.clone();
        }
        None
    }

    /// Implementación de Worst Fit
    fn worst_fit(&mut self, proceso: &Proceso) -> Option<usize> {
        let mut particion_actual = self.primer_particion.clone();
        let mut peor_particion: Option<Rc<RefCell<Particion>>> = None;
        let mut mayor_tamanio: u32 = 0;

        while let Some(particion) = particion_actual {
            let particion_ref = particion.borrow();
            let espacio_libre = particion_ref.espacio_libre();
            if espacio_libre >= proceso.memoria_requerida && espacio_libre > mayor_tamanio {
                peor_particion = Some(particion.clone());
                mayor_tamanio = espacio_libre;
            }
            particion_actual = particion_ref.siguiente.clone();
        }

        if let Some(peor_particion) = peor_particion {
            peor_particion.borrow_mut().ocupar(proceso.nombre.clone());
            return Some(peor_particion.borrow().id);
        }

        None
    }

    /// Libera una partición dado su índice
    pub fn liberar_particion(&mut self, indice_particion: usize) {
        let mut particion_actual = self.primer_particion.clone();

        while let Some(particion) = particion_actual {
            if particion.borrow().id == indice_particion {
                particion.borrow_mut().liberar();
                return;
            }
            particion_actual = particion.borrow().siguiente.clone();
        }
    }

    /// Muestra el estado actual de la memoria (particiones)
    pub fn mostrar_estado(&self) {
        println!("{:<10} {:<15} {:<10} {:<10}", "ID", "Dirección", "Tamaño", "Estado");
        println!("---------------------------------------------------");
        let mut particion_actual = self.primer_particion.clone();

        while let Some(particion) = particion_actual {
            let particion_ref = particion.borrow();
            let estado = match &particion_ref.estado {
                EstadoParticion::Libre => "Libre".to_string(),
                EstadoParticion::Ocupada(nombre_proceso) => format!("Ocupada ({})", nombre_proceso),
            };
            println!(
                "{:<10} {:<15} {:<10} {:<10}",
                particion_ref.id, particion_ref.direccion_comienzo, particion_ref.tamanio, estado
            );
            particion_actual = particion_ref.siguiente.clone();
        }
    }
}
