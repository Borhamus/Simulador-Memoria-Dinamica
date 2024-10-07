/* particion.rs
   Este archivo contiene la definición de las particiones de memoria que
   serán asignadas a los procesos. Incluye funciones para liberar y ocupar
   particiones.
*/


/// Representa una partición de memoria en el sistema.
pub struct Particion {
    pub direccion_comienzo: u32,  // Dirección de inicio de la partición en memoria.
    pub tamano: u32,              // Tamaño de la partición de memoria.
    libre: bool,                  // Indica si la partición está libre o ocupada.
}

impl Particion {
    /// Crea una nueva partición con la dirección de comienzo y el tamaño dados.
    pub fn new(direccion_comienzo: u32, tamano: u32) -> Self {
        Particion {
            direccion_comienzo,
            tamano,
            libre: true,  // Todas las particiones comienzan libres al crearse.
        }
    }

    /// Indica si la partición está libre.
    pub fn esta_libre(&self) -> bool {
        self.libre
    }

    /// Marca la partición como ocupada.
    pub fn ocupar(&mut self) {
        self.libre = false;
    }

    /// Marca la partición como libre.
    pub fn liberar(&mut self) {
        self.libre = true;
    }

    /// Devuelve el tamaño de la partición.
    pub fn get_tamano(&self) -> u32 {
        self.tamano
    }

    /// Devuelve la dirección de comienzo de la partición.
    pub fn get_direccion_comienzo(&self) -> u32 {
        self.direccion_comienzo
    }
}
