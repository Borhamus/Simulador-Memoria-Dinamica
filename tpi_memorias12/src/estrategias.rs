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
    let memoria_requerida = proceso.memoria_requerida as usize; // Número de particiones necesarias
    let mut indice_inicial = None;
    let mut particiones_contiguas = 0;

    for (index, particion) in particiones.iter().enumerate() {
        if particion.esta_libre() {
            if indice_inicial.is_none() {
                indice_inicial = Some(index);
            }
            particiones_contiguas += 1;

            if particiones_contiguas >= memoria_requerida {
                // Ocupar las particiones desde indice_inicial hasta index inclusive
                for i in indice_inicial.unwrap()..=index {
                    particiones[i].ocupar(
                        proceso.nombre.clone(),
                        proceso.arribo as u32,
                        proceso.duracion as u32,
                    );
                }
                return indice_inicial; // Retornamos el índice inicial del bloque asignado
            }
        } else {
            // Si encontramos una partición ocupada, reiniciamos la búsqueda
            indice_inicial = None;
            particiones_contiguas = 0;
        }
    }
    None // No se encontró un bloque adecuado
}


pub fn best_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    let memoria_requerida = proceso.memoria_requerida as usize;
    let mut mejor_indice = None;
    let mut mejor_tamanio = usize::MAX;
    let mut indice_inicial = None;
    let mut particiones_contiguas = 0;

    for (index, particion) in particiones.iter().enumerate() {
        if particion.esta_libre() {
            if indice_inicial.is_none() {
                indice_inicial = Some(index);
            }
            particiones_contiguas += 1;
        } else {
            if particiones_contiguas >= memoria_requerida && particiones_contiguas < mejor_tamanio {
                mejor_indice = indice_inicial;
                mejor_tamanio = particiones_contiguas;
            }
            indice_inicial = None;
            particiones_contiguas = 0;
        }
    }

    // Verificar al final del vector
    if particiones_contiguas >= memoria_requerida && particiones_contiguas < mejor_tamanio {
        mejor_indice = indice_inicial;
        mejor_tamanio = particiones_contiguas;
    }

    if let Some(indice) = mejor_indice {
        let fin = indice + memoria_requerida;
        for i in indice..fin {
            particiones[i].ocupar(
                proceso.nombre.clone(),
                proceso.arribo as u32,
                proceso.duracion as u32,
            );
        }
        return Some(indice);
    }
    None
}


pub fn next_fit(
    particiones: &mut Vec<Particion>,
    proceso: &Proceso,
    ultima_asignada: &mut usize,
) -> Option<usize> {
    let memoria_requerida = proceso.memoria_requerida as usize;
    let mut indice_inicial = None;
    let mut particiones_contiguas = 0;
    let n = particiones.len();
    let mut index = *ultima_asignada;

    let mut vueltas = 0;
    while vueltas < n {
        let particion = &particiones[index % n];
        if particion.esta_libre() {
            if indice_inicial.is_none() {
                indice_inicial = Some(index % n);
            }
            particiones_contiguas += 1;
            if particiones_contiguas >= memoria_requerida {
                let inicio = indice_inicial.unwrap();
                let fin = (inicio + memoria_requerida) % n;
                for i in 0..memoria_requerida {
                    let idx = (inicio + i) % n;
                    particiones[idx].ocupar(
                        proceso.nombre.clone(),
                        proceso.arribo as u32,
                        proceso.duracion as u32,
                    );
                }
                *ultima_asignada = fin % n;
                return Some(inicio);
            }
        } else {
            indice_inicial = None;
            particiones_contiguas = 0;
        }
        index += 1;
        vueltas += 1;
    }
    None
}


pub fn worst_fit(particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
    let memoria_requerida = proceso.memoria_requerida as usize;
    let mut peor_indice = None;
    let mut peor_tamanio = 0;
    let mut indice_inicial = None;
    let mut particiones_contiguas = 0;

    for (index, particion) in particiones.iter().enumerate() {
        if particion.esta_libre() {
            if indice_inicial.is_none() {
                indice_inicial = Some(index);
            }
            particiones_contiguas += 1;
        } else {
            if particiones_contiguas >= memoria_requerida && particiones_contiguas > peor_tamanio {
                peor_indice = indice_inicial;
                peor_tamanio = particiones_contiguas;
            }
            indice_inicial = None;
            particiones_contiguas = 0;
        }
    }

    // Verificar al final del vector
    if particiones_contiguas >= memoria_requerida && particiones_contiguas > peor_tamanio {
        peor_indice = indice_inicial;
        peor_tamanio = particiones_contiguas;
    }

    if let Some(indice) = peor_indice {
        let fin = indice + memoria_requerida;
        for i in indice..fin {
            particiones[i].ocupar(
                proceso.nombre.clone(),
                proceso.arribo as u32,
                proceso.duracion as u32,
            );
        }
        return Some(indice);
    }
    None
}
