// particion.rs
//
// Este módulo define la estructura y el comportamiento de las particiones de memoria
// que serán asignadas a los procesos. Incluye métodos para ocupar, liberar y
// modificar particiones, así como funciones auxiliares para manejar su estado.

#[derive(Debug, Clone)]
pub struct Particion {
    direccion_comienzo: u32, // Dirección de comienzo de la partición en la memoria
    tamano: u32,             // Tamaño total de la partición
    estado: EstadoParticion, // Estado de la partición: Libre o Ocupada
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

    /// Retorna la dirección de comienzo de la partición.
    pub fn get_direccion_comienzo(&self) -> u32 {
        self.direccion_comienzo
    }

    /// Retorna el tamaño de la partición.
    pub fn get_tamano(&self) -> u32 {
        self.tamano
    }

    /// Establece un nuevo tamaño para la partición.
    pub fn set_tamano(&mut self, nuevo_tamano: u32) {
        self.tamano = nuevo_tamano;
    }

    /// Verifica si la partición está libre.
    pub fn esta_libre(&self) -> bool {
        matches!(self.estado, EstadoParticion::Libre)
    }

    /// Ocupar la partición con el nombre del proceso.
    ///
    /// # Parámetros
    /// - `nombre_proceso`: Nombre del proceso que ocupará la partición.
    pub fn ocupar(&mut self, nombre_proceso: String) {
        self.estado = EstadoParticion::Ocupada(nombre_proceso);
    }

    /// Liberar la partición.
    pub fn liberar(&mut self) {
        self.estado = EstadoParticion::Libre;
    }

    /// Verifica si la partición está ocupada por un proceso en particular.
    ///
    /// # Parámetros
    /// - `nombre_proceso`: Nombre del proceso a verificar.
    ///
    /// # Retorna
    /// - `bool`: `true` si la partición está ocupada por el proceso, `false` en caso contrario.
    pub fn esta_ocupada_por(&self, nombre_proceso: &str) -> bool {
        matches!(self.estado, EstadoParticion::Ocupada(ref p) if p == nombre_proceso)
    }
}
