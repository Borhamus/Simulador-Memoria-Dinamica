// estrategias.rs
//
// Este archivo define las distintas estrategias de asignación de particiones
// dinámicas: First-Fit, Best-Fit, Next-Fit y Worst-Fit. Cada estrategia
// implementa el trait `EstrategiaAsignacion`.

use crate::proceso::Proceso;
use crate::particion::Particion;

/// Define el trait `EstrategiaAsignacion`, que especifica cómo se asignan
/// los procesos a las particiones de memoria según diferentes estrategias.
pub trait EstrategiaAsignacion {
    /// Asigna un proceso a una partición de memoria.
    ///
    /// # Parámetros
    /// - `&mut self`: Permite modificar el estado interno de la estrategia.
    /// - `proceso`: Referencia al proceso que se desea asignar.
    /// - `particiones`: Referencia mutable al vector de particiones disponibles.
    ///
    /// # Retorna
    /// - `Option<usize>`: Índice de la partición asignada si la asignación es exitosa.
    fn asignar(&mut self, proceso: &Proceso, particiones: &mut Vec<Particion>) -> Option<usize>;
}

/// Estrategia First-Fit: Asigna el proceso a la primera partición libre que sea lo suficientemente grande.
pub struct FirstFit;

impl EstrategiaAsignacion for FirstFit {
    fn asignar(
        &mut self,
        proceso: &Proceso,
        particiones: &mut Vec<Particion>,
    ) -> Option<usize> {
        let mut indice_encontrado = None;

        for (i, particion) in particiones.iter().enumerate() {
            if particion.esta_libre() && particion.get_tamano() >= proceso.get_memoria_requerida() {
                indice_encontrado = Some(i);
                break;
            }
        }

        if let Some(i) = indice_encontrado {
            let tamano_requerido = proceso.get_memoria_requerida();
            let direccion_comienzo = particiones[i].get_direccion_comienzo();
            let tamano_actual = particiones[i].get_tamano();

            if tamano_actual > tamano_requerido {
                // Dividir la partición
                let nueva_particion = Particion::new(
                    direccion_comienzo + tamano_requerido,
                    tamano_actual - tamano_requerido,
                );
                particiones[i].set_tamano(tamano_requerido);
                // Insertar la nueva partición sin mantener una referencia mutable a particiones[i]
                particiones.insert(i + 1, nueva_particion);
            }

            // Ahora podemos tomar una referencia mutable para ocupar la partición
            particiones[i].ocupar(proceso.get_nombre().to_string());

            println!(
                "Proceso {} asignado a la partición {}.",
                proceso.get_nombre(),
                i
            );
            return Some(i);
        }

        println!(
            "No hay partición disponible para el proceso {}.",
            proceso.get_nombre()
        );
        None
    }
}

/// Estrategia Best-Fit: Asigna el proceso a la partición libre más ajustada (mínimo espacio sobrante).
pub struct BestFit;

impl EstrategiaAsignacion for BestFit {
    fn asignar(
        &mut self,
        proceso: &Proceso,
        particiones: &mut Vec<Particion>,
    ) -> Option<usize> {
        let mut mejor_indice = None;
        let mut menor_espacio_sobrante = u32::MAX;

        for (i, particion) in particiones.iter().enumerate() {
            if particion.esta_libre() && particion.get_tamano() >= proceso.get_memoria_requerida() {
                let espacio_sobrante = particion.get_tamano() - proceso.get_memoria_requerida();
                if espacio_sobrante < menor_espacio_sobrante {
                    menor_espacio_sobrante = espacio_sobrante;
                    mejor_indice = Some(i);
                }
            }
        }

        if let Some(i) = mejor_indice {
            let tamano_requerido = proceso.get_memoria_requerida();
            let direccion_comienzo = particiones[i].get_direccion_comienzo();
            let tamano_actual = particiones[i].get_tamano();

            if tamano_actual > tamano_requerido {
                let nueva_particion = Particion::new(
                    direccion_comienzo + tamano_requerido,
                    tamano_actual - tamano_requerido,
                );
                particiones[i].set_tamano(tamano_requerido);
                particiones.insert(i + 1, nueva_particion);
            }

            particiones[i].ocupar(proceso.get_nombre().to_string());

            println!(
                "Proceso {} asignado a la partición {}.",
                proceso.get_nombre(),
                i
            );
            return Some(i);
        }

        println!(
            "No hay partición disponible para el proceso {}.",
            proceso.get_nombre()
        );
        None
    }
}

/// Estrategia Worst-Fit: Asigna el proceso a la partición libre más grande.
pub struct WorstFit;

impl EstrategiaAsignacion for WorstFit {
    fn asignar(
        &mut self,
        proceso: &Proceso,
        particiones: &mut Vec<Particion>,
    ) -> Option<usize> {
        let mut peor_indice = None;
        let mut mayor_tamano = 0;

        for (i, particion) in particiones.iter().enumerate() {
            if particion.esta_libre() && particion.get_tamano() >= proceso.get_memoria_requerida() {
                if particion.get_tamano() > mayor_tamano {
                    mayor_tamano = particion.get_tamano();
                    peor_indice = Some(i);
                }
            }
        }

        if let Some(i) = peor_indice {
            let tamano_requerido = proceso.get_memoria_requerida();
            let direccion_comienzo = particiones[i].get_direccion_comienzo();
            let tamano_actual = particiones[i].get_tamano();

            if tamano_actual > tamano_requerido {
                let nueva_particion = Particion::new(
                    direccion_comienzo + tamano_requerido,
                    tamano_actual - tamano_requerido,
                );
                particiones[i].set_tamano(tamano_requerido);
                particiones.insert(i + 1, nueva_particion);
            }

            particiones[i].ocupar(proceso.get_nombre().to_string());

            println!(
                "Proceso {} asignado a la partición {}.",
                proceso.get_nombre(),
                i
            );
            return Some(i);
        }

        println!(
            "No hay partición disponible para el proceso {}.",
            proceso.get_nombre()
        );
        None
    }
}

/// Estrategia Next-Fit: Similar a First-Fit, pero comienza la búsqueda desde donde se asignó la última vez.
pub struct NextFit {
    ultimo_indice: usize,
}

impl NextFit {
    /// Crea una nueva instancia de NextFit con el índice inicial en 0.
    pub fn new() -> Self {
        Self { ultimo_indice: 0 }
    }
}

impl EstrategiaAsignacion for NextFit {
    fn asignar(
        &mut self,
        proceso: &Proceso,
        particiones: &mut Vec<Particion>,
    ) -> Option<usize> {
        let n = particiones.len();
        let mut indice_actual = self.ultimo_indice;

        for _ in 0..n {
            if particiones[indice_actual].esta_libre()
                && particiones[indice_actual].get_tamano() >= proceso.get_memoria_requerida()
            {
                let tamano_requerido = proceso.get_memoria_requerida();
                let direccion_comienzo = particiones[indice_actual].get_direccion_comienzo();
                let tamano_actual = particiones[indice_actual].get_tamano();

                if tamano_actual > tamano_requerido {
                    let nueva_particion = Particion::new(
                        direccion_comienzo + tamano_requerido,
                        tamano_actual - tamano_requerido,
                    );
                    particiones[indice_actual].set_tamano(tamano_requerido);
                    particiones.insert(indice_actual + 1, nueva_particion);
                }

                particiones[indice_actual].ocupar(proceso.get_nombre().to_string());

                println!(
                    "Proceso {} asignado a la partición {}.",
                    proceso.get_nombre(),
                    indice_actual
                );
                self.ultimo_indice = indice_actual;
                return Some(indice_actual);
            }
            indice_actual = (indice_actual + 1) % n;
        }

        println!(
            "No hay partición disponible para el proceso {}.",
            proceso.get_nombre()
        );
        None
    }
}
