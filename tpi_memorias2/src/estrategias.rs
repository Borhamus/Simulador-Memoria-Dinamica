//estrategias.rs


use crate::simulaciones::proceso::Proceso;
use crate::simulaciones::particion::Particion;

// Estrategia First-Fit: Encuentra la primera partición libre que pueda acomodar el proceso.
pub fn first_fit(proceso: &Proceso, particiones: &mut Vec<Particion>) -> Option<usize> {
    for (i, particion) in particiones.iter_mut().enumerate() {
        if particion.esta_libre() && particion.get_tamano() >= proceso.get_memoria_requerida() {
            particion.ocupar();
            println!("Proceso {} asignado a la partición {}.", proceso.get_nombre(), i);
            return Some(i);
        }
    }
    println!("No hay partición disponible para el proceso {}.", proceso.get_nombre());
    None
}

// Estrategia Best-Fit: Busca la partición más ajustada, minimizando el espacio libre restante.
pub fn best_fit(proceso: &Proceso, particiones: &mut Vec<Particion>) -> Option<usize> {
    let mut mejor_indice = None;
    let mut menor_tamano = usize::MAX; // Inicializamos con el valor más alto posible.

    for (i, particion) in particiones.iter_mut().enumerate() {
        if particion.esta_libre() && particion.get_tamano() as usize >= proceso.get_memoria_requerida() as usize {
            let espacio_restante = particion.get_tamano() as usize - proceso.get_memoria_requerida() as usize;
            if espacio_restante < menor_tamano {
                menor_tamano = espacio_restante;
                mejor_indice = Some(i);
            }
        }
    }

    if let Some(indice) = mejor_indice {
        particiones[indice].ocupar();
        println!("Proceso {} asignado a la partición {}.", proceso.get_nombre(), indice);
        return Some(indice);
    }

    println!("No hay partición disponible para el proceso {}.", proceso.get_nombre());
    None
}


// Estrategia Next-Fit: Encuentra la siguiente partición disponible después de la última asignada
pub fn next_fit(proceso: &Proceso, particiones: &mut Vec<Particion>, ultimo_indice: &mut usize) -> Option<usize> {
    let n = particiones.len();
    let mut indice_actual = *ultimo_indice;

    for _ in 0..n {
        if particiones[indice_actual].esta_libre() && particiones[indice_actual].get_tamano() >= proceso.get_memoria_requerida() {
            particiones[indice_actual].ocupar();
            *ultimo_indice = (indice_actual + 1) % n; // Avanza el índice circularmente
            return Some(indice_actual);
        }
        indice_actual = (indice_actual + 1) % n;
    }

    println!("No hay partición disponible para el proceso {}.", proceso.get_nombre());
    None
}


// Estrategia Worst-Fit: Busca la partición más grande que pueda acomodar al proceso.
pub fn worst_fit(proceso: &Proceso, particiones: &mut Vec<Particion>) -> Option<usize> {
    let mut peor_indice = None;
    let mut mayor_tamano = 0;

    // Iteramos sobre las particiones.
    for (i, particion) in particiones.iter_mut().enumerate() {
        if particion.esta_libre() && particion.get_tamano() >= proceso.get_memoria_requerida() {
            let tamano_disponible = particion.get_tamano() - proceso.get_memoria_requerida();
            
            // Si esta partición tiene más espacio disponible que la peor opción actual, la seleccionamos.
            if tamano_disponible > mayor_tamano {
                mayor_tamano = tamano_disponible;
                peor_indice = Some(i);
            }
        }
    }

    // Si encontramos una partición adecuada, la marcamos como ocupada.
    if let Some(indice) = peor_indice {
        particiones[indice].ocupar();
        println!("Proceso {} asignado a la partición {}.", proceso.get_nombre(), indice);
        return Some(indice);
    }

    println!("No hay partición disponible para el proceso {}.", proceso.get_nombre());
    None
}
