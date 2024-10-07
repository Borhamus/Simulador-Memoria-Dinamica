// particion.rs

/// Define la estructura y el comportamiento de las particiones de memoria
/// que serán asignadas a los procesos. Incluye métodos para ocupar, liberar 
/// y modificar particiones, así como funciones auxiliares para manejar su estado.

#[derive(Debug, Clone)]
pub struct Particion {
    pub direccion_comienzo: u32, // Dirección de comienzo de la partición en la memoria
    pub tamano: u32,             // Tamaño total de la partición
    pub estado: EstadoParticion, // Estado de la partición: Libre o Ocupada
}

#[derive(Debug, Clone)]
pub enum EstadoParticion {
    Libre,
    Ocupada(String), // Ocupada por un proceso identificado por su nombre
}

impl Particion {
    /// Crea una nueva partición de memoria.
    ///
    /// # Parámetros
    /// - `direccion_comienzo`: Dirección de comienzo de la partición en la memoria.
    /// - `tamano`: Tamaño total de la partición.
    pub fn new(direccion_comienzo: u32, tamano: u32) -> Self {
        Particion {
            direccion_comienzo,
            tamano,
            estado: EstadoParticion::Libre,
        }
    }

    /// Libera la partición, marcándola como libre.
    pub fn liberar(&mut self) {
        self.estado = EstadoParticion::Libre;
    }

    /// Marca la partición como ocupada por un proceso.
    ///
    /// # Parámetros
    /// - `nombre_proceso`: Nombre del proceso que ocupa la partición.
    pub fn ocupar(&mut self, nombre_proceso: String) {
        self.estado = EstadoParticion::Ocupada(nombre_proceso);
    }

    /// Verifica si la partición está libre.
    pub fn esta_libre(&self) -> bool {
        matches!(self.estado, EstadoParticion::Libre)
    }

    /// Devuelve el tamaño de la partición.
    pub fn get_tamano(&self) -> u32 {
        self.tamano
    }

    /// Verifica si esta partición es adyacente a otra y está libre.
    ///
    /// # Parámetros
    /// - `otra`: Referencia a la otra partición.
    pub fn es_adyacente(&self, otra: &Particion) -> bool {
        self.direccion_comienzo + self.tamano == otra.direccion_comienzo
    }
}

/// Función para combinar particiones adyacentes que estén libres (coalescing).
///
/// # Parámetros
/// - `particiones`: Referencia mutable al vector de particiones.
pub fn coalescer_particiones(particiones: &mut Vec<Particion>) {
    let mut i = 0;
    while i < particiones.len() - 1 {
        // Si la partición actual y la siguiente están libres y son adyacentes
        if particiones[i].esta_libre() && particiones[i + 1].esta_libre() && particiones[i].es_adyacente(&particiones[i + 1]) {
            // Combinar particiones: sumar el tamaño de la siguiente partición a la actual
            particiones[i].tamano += particiones[i + 1].tamano;
            // Eliminar la partición siguiente
            particiones.remove(i + 1);
        } else {
            i += 1; // Avanzar solo si no se hizo una combinación
        }
    }
}

/// Imprime el estado actual de las particiones de memoria.
///
/// # Parámetros
/// - `particiones`: Referencia al vector de particiones.
pub fn mostrar_estado_memoria(particiones: &Vec<Particion>) {
    println!("Estado actual de la memoria:");
    for particion in particiones {
        match &particion.estado {
            EstadoParticion::Libre => println!(
                "Partición libre - Tamaño: {} - Dirección: {}",
                particion.tamano, particion.direccion_comienzo
            ),
            EstadoParticion::Ocupada(proceso) => println!(
                "Partición ocupada por {} - Tamaño: {} - Dirección: {}",
                proceso, particion.tamano, particion.direccion_comienzo
            ),
        }
    }
}

/// Calcula la cantidad de memoria fragmentada externamente.
/// La fragmentación externa es la suma de todos los espacios libres
/// que no son lo suficientemente grandes para alojar un proceso.
///
/// # Parámetros
/// - `particiones`: Referencia al vector de particiones.
/// - `tamaño_minimo`: Tamaño mínimo que debe tener una partición para que sea útil.
/// # Retorna
/// - La cantidad de memoria fragmentada externamente.
pub fn calcular_fragmentacion_externa(particiones: &Vec<Particion>, tamaño_minimo: u32) -> u32 {
    let mut fragmentacion = 0;
    for particion in particiones {
        if particion.esta_libre() && particion.get_tamano() < tamaño_minimo {
            fragmentacion += particion.get_tamano();
        }
    }
    fragmentacion
}

/// Recompacta la memoria, moviendo procesos a las primeras particiones libres
/// para liberar una gran partición contigua al final.
///
/// # Parámetros
/// - `particiones`: Referencia mutable al vector de particiones.
pub fn recompactar_memoria(particiones: &mut Vec<Particion>) {
    let mut memoria_libre = 0;
    let mut procesos_en_memoria = Vec::new();

    // Recolectar todos los procesos y liberar memoria
    for particion in particiones.iter_mut() {
        if let EstadoParticion::Ocupada(proceso) = &particion.estado {
            // Guardar el proceso que estaba ocupando la partición
            procesos_en_memoria.push((proceso.clone(), particion.get_tamano()));
        }
        memoria_libre += particion.get_tamano();
        particion.liberar(); // Liberar la partición
    }

    // Limpiar todas las particiones actuales
    particiones.clear();

    // Reasignar los procesos en particiones contiguas
    let mut direccion = 0;
    for (proceso, tamano) in procesos_en_memoria {
        particiones.push(Particion {
            direccion_comienzo: direccion,
            tamano,
            estado: EstadoParticion::Ocupada(proceso),
        });
        direccion += tamano;
    }

    // Crear una única partición libre con el espacio restante
    particiones.push(Particion::new(direccion, memoria_libre - direccion));
}
