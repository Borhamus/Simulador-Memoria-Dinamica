//estrategia.rs

use crate::particion::Particion;
use crate::proceso::Proceso;

#[derive(Clone, Copy)]
pub enum EstrategiaAsignacion {
    FirstFit,
    BestFit,
    NextFit,
    WorstFit,
}

impl EstrategiaAsignacion {
    pub fn asignar_tanda(
        &self,
        particiones: &mut Vec<Particion>,
        procesos: &Vec<Proceso>,
    ) -> Vec<Option<usize>> {
        let mut ultima_asignada: usize = 0;
        let mut asignaciones = Vec::new();

        for proceso in procesos {
            let asignacion = match self {
                EstrategiaAsignacion::FirstFit => first_fit(particiones, proceso),
                EstrategiaAsignacion::BestFit => best_fit(particiones, proceso),
                EstrategiaAsignacion::NextFit => next_fit(particiones, proceso, &mut ultima_asignada),
                EstrategiaAsignacion::WorstFit => worst_fit(particiones, proceso),
            };
            asignaciones.push(asignacion);
        }

        asignaciones
    }
}

pub fn first_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    for (i, particion) in particiones.iter_mut().enumerate() {
        if particion.espacio_libre() >= proceso.memoria_requerida {
            particion.ocupar(&proceso.nombre); // Corregido: Pasamos &proceso.nombre
            return Some(i);
        }
    }
    None
}

pub fn best_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    let mut mejor_particion: Option<(usize, u32)> = None;
    for (i, particion) in particiones.iter_mut().enumerate() {
        let espacio_libre = particion.espacio_libre();
        if espacio_libre >= proceso.memoria_requerida {
            if mejor_particion.is_none() || espacio_libre < mejor_particion.unwrap().1 {
                mejor_particion = Some((i, espacio_libre));
            }
        }
    }

    if let Some((i, _)) = mejor_particion {
        particiones[i].ocupar(&proceso.nombre); // Corregido: Pasamos &proceso.nombre
        Some(i)
    } else {
        None
    }
}

pub fn next_fit(particiones: &mut Vec<Particion>, proceso: &Proceso, ultima_asignada: &mut usize) -> Option<usize> {
    for i in (*ultima_asignada..particiones.len()).chain(0..*ultima_asignada) {
        if particiones[i].espacio_libre() >= proceso.memoria_requerida {
            particiones[i].ocupar(&proceso.nombre); // Corregido: Pasamos &proceso.nombre
            *ultima_asignada = i;
            return Some(i);
        }
    }
    None
}

pub fn worst_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    let mut peor_particion: Option<(usize, u32)> = None;
    let mut mayor_tamanio: u32 = 0;
    for (i, particion) in particiones.iter_mut().enumerate() {
        let espacio_libre = particion.espacio_libre();
        if espacio_libre >= proceso.memoria_requerida {
            if espacio_libre > mayor_tamanio {
                mayor_tamanio = espacio_libre;
                peor_particion = Some((i, espacio_libre));
            }
        }
    }

    if let Some((i, _)) = peor_particion {
        particiones[i].ocupar(&proceso.nombre); // Corregido: Pasamos &proceso.nombre
        Some(i)
    } else {
        None
    }
}
