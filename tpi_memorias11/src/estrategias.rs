use crate::particion::Particion;
use crate::proceso::Proceso;

#[derive(Debug)]
pub enum EstrategiaAsignacion {
    FirstFit,
    BestFit,
    NextFit,
    WorstFit,
}

impl EstrategiaAsignacion {
    pub fn asignar_proceso(
        &self,
        particiones: &mut Vec<Particion>,
        proceso: &Proceso,
        ultima_asignada: &mut usize,
    ) -> Option<usize> {
        match self {
            EstrategiaAsignacion::FirstFit => first_fit(particiones, proceso),
            EstrategiaAsignacion::BestFit => best_fit(particiones, proceso),
            EstrategiaAsignacion::NextFit => next_fit(particiones, proceso, ultima_asignada),
            EstrategiaAsignacion::WorstFit => worst_fit(particiones, proceso),
        }
    }
}

pub fn first_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    for (index, particion) in particiones.iter_mut().enumerate() {
        if particion.esta_libre() {
            particion.ocupar(proceso.nombre.clone(), proceso.duracion);
            return Some(index);
        }
    }
    None
}

pub fn best_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    for (index, particion) in particiones.iter_mut().enumerate() {
        if particion.esta_libre() {
            particion.ocupar(proceso.nombre.clone(), proceso.duracion);
            return Some(index);
        }
    }
    None
}

pub fn next_fit(particiones: &mut Vec<Particion>, proceso: &Proceso, ultima_asignada: &mut usize) -> Option<usize> {
    let mut index = *ultima_asignada;
    let mut vueltas = 0;

    while vueltas < particiones.len() {
        let particion = &mut particiones[index];
        if particion.esta_libre() {
            particion.ocupar(proceso.nombre.clone(), proceso.duracion);
            *ultima_asignada = index;
            return Some(index);
        }
        index = (index + 1) % particiones.len();
        vueltas += 1;
    }

    None
}

pub fn worst_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    for (index, particion) in particiones.iter_mut().enumerate() {
        if particion.esta_libre() {
            particion.ocupar(proceso.nombre.clone(), proceso.duracion);
            return Some(index);
        }
    }
    None
}