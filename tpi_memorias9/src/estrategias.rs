use crate::particion::Particion;
use crate::proceso::Proceso;
use std::fmt; // Para implementar el rasgo Display

/// Enumeración para las estrategias de asignación de memoria
#[derive(Clone)]
pub enum EstrategiaAsignacion {
    FirstFit,  // Primer ajuste
    BestFit,   // Mejor ajuste
    NextFit,   // Siguiente ajuste
    WorstFit,  // Peor ajuste
}

/// Implementamos el rasgo Display para poder convertir EstrategiaAsignacion en una cadena de texto
impl fmt::Display for EstrategiaAsignacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nombre = match self {
            EstrategiaAsignacion::FirstFit => "First Fit",
            EstrategiaAsignacion::BestFit => "Best Fit",
            EstrategiaAsignacion::NextFit => "Next Fit",
            EstrategiaAsignacion::WorstFit => "Worst Fit",
        };
        write!(f, "{}", nombre) // Escribimos el nombre de la estrategia
    }
}

impl EstrategiaAsignacion {
    /// Asigna un proceso a una partición utilizando la estrategia seleccionada
    pub fn asignar_proceso(
        &self,
        particiones: &mut Vec<Particion>,
        proceso: &Proceso,
        ultima_asignada: &mut usize,
    ) -> Option<usize> {
        match self {
            EstrategiaAsignacion::FirstFit => self.first_fit(particiones, proceso, ultima_asignada),
            EstrategiaAsignacion::BestFit => self.best_fit(particiones, proceso),
            EstrategiaAsignacion::NextFit => self.next_fit(particiones, proceso, ultima_asignada),
            EstrategiaAsignacion::WorstFit => self.worst_fit(particiones, proceso),
        }
    }

    /// Estrategia First Fit (Primer ajuste)
    /// Busca la primera partición que tenga suficiente espacio para el proceso
    fn first_fit(
        &self,
        particiones: &mut Vec<Particion>,
        proceso: &Proceso,
        _ultima_asignada: &mut usize,
    ) -> Option<usize> {
        // Iteramos sobre todas las particiones y buscamos la primera libre con suficiente espacio
        for (i, particion) in particiones.iter_mut().enumerate() {
            if particion.espacio_libre() >= proceso.memoria_requerida {
                particion.ocupar(proceso.get_nombre()); // Asignamos el proceso a la partición
                return Some(i); // Devolvemos el índice de la partición asignada
            }
        }
        None // No se encontró una partición adecuada
    }

    /// Estrategia Best Fit (Mejor ajuste)
    /// Busca la partición más pequeña que tenga suficiente espacio para el proceso
    fn best_fit(&self, particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
        let mut mejor_particion: Option<(usize, u32)> = None;
        // Iteramos sobre todas las particiones buscando la más pequeña que pueda acomodar el proceso
        for (i, particion) in particiones.iter_mut().enumerate() {
            let espacio_libre = particion.espacio_libre();
            if espacio_libre >= proceso.memoria_requerida {
                if mejor_particion.is_none() || espacio_libre < mejor_particion.unwrap().1 {
                    mejor_particion = Some((i, espacio_libre)); // Guardamos la partición si es más pequeña
                }
            }
        }

        if let Some((i, _)) = mejor_particion {
            particiones[i].ocupar(proceso.get_nombre()); // Asignamos la mejor partición encontrada
            Some(i)
        } else {
            None // No se encontró una partición adecuada
        }
    }

    /// Estrategia Next Fit (Siguiente ajuste)
    /// Similar a First Fit, pero empieza a buscar desde la última partición asignada
    fn next_fit(
        &self,
        particiones: &mut Vec<Particion>,
        proceso: &Proceso,
        ultima_asignada: &mut usize,
    ) -> Option<usize> {
        let n = particiones.len();
        let mut i = *ultima_asignada;

        // Buscamos desde la última partición asignada hasta el final del vector
        while i < n {
            if particiones[i].espacio_libre() >= proceso.memoria_requerida {
                particiones[i].ocupar(proceso.get_nombre());
                *ultima_asignada = i; // Actualizamos el índice de la última partición asignada
                return Some(i);
            }
            i += 1;
        }

        // Si no encontramos desde la última asignada, buscamos desde el inicio hasta donde comenzamos
        i = 0;
        while i < *ultima_asignada {
            if particiones[i].espacio_libre() >= proceso.memoria_requerida {
                particiones[i].ocupar(proceso.get_nombre());
                *ultima_asignada = i;
                return Some(i);
            }
            i += 1;
        }
        None // No se encontró una partición adecuada
    }

    /// Estrategia Worst Fit (Peor ajuste)
    /// Busca la partición más grande para reducir al máximo el fragmento sobrante
    fn worst_fit(&self, particiones: &mut Vec<Particion>, proceso: &Proceso) -> Option<usize> {
        let mut peor_particion: Option<(usize, u32)> = None;
        // Iteramos sobre todas las particiones buscando la más grande que pueda acomodar el proceso
        for (i, particion) in particiones.iter_mut().enumerate() {
            let espacio_libre = particion.espacio_libre();
            if espacio_libre >= proceso.memoria_requerida {
                if peor_particion.is_none() || espacio_libre > peor_particion.unwrap().1 {
                    peor_particion = Some((i, espacio_libre)); // Guardamos la partición si es más grande
                }
            }
        }

        if let Some((i, _)) = peor_particion {
            particiones[i].ocupar(proceso.get_nombre()); // Asignamos la peor (más grande) partición encontrada
            Some(i)
        } else {
            None // No se encontró una partición adecuada
        }
    }
}
