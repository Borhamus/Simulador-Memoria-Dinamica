//estrategias.rs

use crate::particion::{Particion, EstadoParticion};
use crate::proceso::Proceso;

#[derive(Debug, Clone, Copy)]
pub enum EstrategiaAsignacion {
    FirstFit,
    BestFit,
    NextFit,
    WorstFit,
}

pub fn first_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    for i in 0..particiones.len() {
        if let EstadoParticion::Libre = particiones[i].estado {
            if particiones[i].tamanio >= proceso.memoria_requerida {
                let particion_id = particiones[i].id;
                let direccion_comienzo = particiones[i].direccion_comienzo;

                let tamanio_restante = particiones[i].tamanio - proceso.memoria_requerida;

                particiones[i].tamanio = proceso.memoria_requerida;
                particiones[i].estado = EstadoParticion::Ocupada(proceso.nombre.clone());

                if tamanio_restante > 0 {
                    let nueva_particion = Particion {
                        id: particiones.len(),
                        direccion_comienzo: direccion_comienzo + proceso.memoria_requerida,
                        tamanio: tamanio_restante,
                        estado: EstadoParticion::Libre,
                    };
                    particiones.insert(i + 1, nueva_particion);
                }

                return Some(particion_id);
            }
        }
    }
    None
}

pub fn best_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    let mut mejor_particion_idx: Option<usize> = None;
    let mut menor_diferencia: u32 = u32::MAX;

    for (i, particion) in particiones.iter().enumerate() {
        if let EstadoParticion::Libre = particion.estado {
            if particion.tamanio >= proceso.memoria_requerida {
                let diferencia = particion.tamanio - proceso.memoria_requerida;
                if diferencia < menor_diferencia {
                    menor_diferencia = diferencia;
                    mejor_particion_idx = Some(i);
                }
            }
        }
    }

    if let Some(i) = mejor_particion_idx {
        let particion_id = particiones[i].id;
        let direccion_comienzo = particiones[i].direccion_comienzo;

        let tamanio_restante = particiones[i].tamanio - proceso.memoria_requerida;

        particiones[i].tamanio = proceso.memoria_requerida;
        particiones[i].estado = EstadoParticion::Ocupada(proceso.nombre.clone());

        if tamanio_restante > 0 {
            let nueva_particion = Particion {
                id: particiones.len(),
                direccion_comienzo: direccion_comienzo + proceso.memoria_requerida,
                tamanio: tamanio_restante,
                estado: EstadoParticion::Libre,
            };
            particiones.insert(i + 1, nueva_particion);
        }

        return Some(particion_id);
    }
    None
}

pub fn worst_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    let mut peor_particion_idx: Option<usize> = None;
    let mut mayor_tamanio: u32 = 0;

    for (i, particion) in particiones.iter().enumerate() {
        if let EstadoParticion::Libre = particion.estado {
            if particion.tamanio >= proceso.memoria_requerida {
                if particion.tamanio > mayor_tamanio {
                    mayor_tamanio = particion.tamanio;
                    peor_particion_idx = Some(i);
                }
            }
        }
    }

    if let Some(i) = peor_particion_idx {
        let particion_id = particiones[i].id;
        let direccion_comienzo = particiones[i].direccion_comienzo;

        let tamanio_restante = particiones[i].tamanio - proceso.memoria_requerida;

        particiones[i].tamanio = proceso.memoria_requerida;
        particiones[i].estado = EstadoParticion::Ocupada(proceso.nombre.clone());

        if tamanio_restante > 0 {
            let nueva_particion = Particion {
                id: particiones.len(),
                direccion_comienzo: direccion_comienzo + proceso.memoria_requerida,
                tamanio: tamanio_restante,
                estado: EstadoParticion::Libre,
            };
            particiones.insert(i + 1, nueva_particion);
        }

        return Some(particion_id);
    }
    None
}

pub fn next_fit(particiones: &mut Vec<Particion>, proceso: &Proceso, ultima_asignada: &mut Option<usize>) -> Option<usize> {
    let mut desde = 0;
    if let Some(ultima_id) = *ultima_asignada {
        if let Some(pos) = particiones.iter().position(|p| p.id == ultima_id) {
            desde = pos + 1;
            if desde >= particiones.len() {
                desde = 0;
            }
        }
    }

    let mut indices = (desde..particiones.len()).chain(0..desde);

    for i in indices {
        if let EstadoParticion::Libre = particiones[i].estado {
            if particiones[i].tamanio >= proceso.memoria_requerida {
                let particion_id = particiones[i].id;
                let direccion_comienzo = particiones[i].direccion_comienzo;

                let tamanio_restante = particiones[i].tamanio - proceso.memoria_requerida;

                particiones[i].tamanio = proceso.memoria_requerida;
                particiones[i].estado = EstadoParticion::Ocupada(proceso.nombre.clone());

                if tamanio_restante > 0 {
                    let nueva_particion = Particion {
                        id: particiones.len(),
                        direccion_comienzo: direccion_comienzo + proceso.memoria_requerida,
                        tamanio: tamanio_restante,
                        estado: EstadoParticion::Libre,
                    };
                    particiones.insert(i + 1, nueva_particion);
                }

                *ultima_asignada = Some(particion_id);
                return Some(particion_id);
            }
        }
    }

    None
}
