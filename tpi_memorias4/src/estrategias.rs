/* estrategias.rs
   Este archivo define las distintas estrategias de asignación de particiones
   dinámicas: First-Fit, Best-Fit, Next-Fit y Worst-Fit. Cada estrategia
   implementa el trait `EstrategiaAsignacion`.
*/


use crate::proceso::Proceso;
use crate::particion::Particion;

/// Definimos el trait `EstrategiaAsignacion`
/// Cada estrategia de asignación implementará este trait.
pub trait EstrategiaAsignacion {
    fn asignar(&self, proceso: &Proceso, particiones: &mut Vec<Particion>) -> Option<usize>;
}

/// Estrategia First-Fit: Encuentra la primera partición libre que pueda acomodar el proceso.
pub struct FirstFit;
impl EstrategiaAsignacion for FirstFit {
    fn asignar(&self, proceso: &Proceso, particiones: &mut Vec<Particion>) -> Option<usize> {
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
}

/// Estrategia Best-Fit: Busca la partición más ajustada, minimizando el espacio libre restante.
pub struct BestFit;
impl EstrategiaAsignacion for BestFit {
    fn asignar(&self, proceso: &Proceso, particiones: &mut Vec<Particion>) -> Option<usize> {
        let mut mejor_indice = None;
        let mut menor_tamano = u32::MAX;  // Cambiamos a u32 para que coincida con el tipo de `espacio_restante`

        for (i, particion) in particiones.iter_mut().enumerate() {
            if particion.esta_libre() && particion.get_tamano() >= proceso.get_memoria_requerida() {
                let espacio_restante = particion.get_tamano() - proceso.get_memoria_requerida();

                if espacio_restante < menor_tamano {  // Comparación ahora es entre `u32`
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
}

/// Estrategia Next-Fit: Encuentra la siguiente partición disponible después de la última asignada.
pub struct NextFit {
    pub ultimo_indice: usize, // Se necesita para recordar la última asignación
}

impl NextFit {
    pub fn new() -> Self {
        Self { ultimo_indice: 0 }
    }
}

impl EstrategiaAsignacion for NextFit {
    fn asignar(&self, proceso: &Proceso, particiones: &mut Vec<Particion>) -> Option<usize> {
        let n = particiones.len();
        let mut indice_actual = self.ultimo_indice;

        for _ in 0..n {
            if particiones[indice_actual].esta_libre() && particiones[indice_actual].get_tamano() >= proceso.get_memoria_requerida() {
                particiones[indice_actual].ocupar();
                return Some(indice_actual);
            }
            indice_actual = (indice_actual + 1) % n;
        }

        println!("No hay partición disponible para el proceso {}.", proceso.get_nombre());
        None
    }
}

/// Estrategia Worst-Fit: Busca la partición más grande que pueda acomodar al proceso.
pub struct WorstFit;
impl EstrategiaAsignacion for WorstFit {
    fn asignar(&self, proceso: &Proceso, particiones: &mut Vec<Particion>) -> Option<usize> {
        let mut peor_indice = None;
        let mut mayor_tamano = 0;

        for (i, particion) in particiones.iter_mut().enumerate() {
            if particion.esta_libre() && particion.get_tamano() >= proceso.get_memoria_requerida() {
                let tamano_disponible = particion.get_tamano() - proceso.get_memoria_requerida();
                if tamano_disponible > mayor_tamano {
                    mayor_tamano = tamano_disponible;
                    peor_indice = Some(i);
                }
            }
        }

        if let Some(indice) = peor_indice {
            particiones[indice].ocupar();
            println!("Proceso {} asignado a la partición {}.", proceso.get_nombre(), indice);
            return Some(indice);
        }

        println!("No hay partición disponible para el proceso {}.", proceso.get_nombre());
        None
    }
}
