//particion.rs


#[derive(Debug, Clone)]
pub struct Particion {
    pub id: usize,
    pub direccion_comienzo: u32,
    pub tamanio: u32,
    pub estado: EstadoParticion,
}

impl Particion {
    /// Crea una nueva partición.
    pub fn new(id: usize, direccion_comienzo: u32, tamanio: u32) -> Particion {
        Particion {
            id,
            direccion_comienzo,
            tamanio,
            estado: EstadoParticion::Libre,
        }
    }

    /// Devuelve el espacio libre en la partición (si está libre).
    pub fn espacio_libre(&self) -> u32 {
        match self.estado {
            EstadoParticion::Libre => self.tamanio,
            EstadoParticion::Ocupada(_) => 0,
        }
    }

    /// Marca la partición como ocupada por un proceso.
    pub fn ocupar(&mut self, nombre_proceso: &str) {
        self.estado = EstadoParticion::Ocupada(nombre_proceso.to_string());
    }

    /// Marca la partición como libre.
    pub fn liberar(&mut self) {
        self.estado = EstadoParticion::Libre;
    }
}

/// Define los estados posibles de una partición: libre o ocupada por un proceso.
#[derive(Debug, Clone)]
pub enum EstadoParticion {
    Libre,
    Ocupada(String), // Almacena el nombre del proceso que ocupa la partición
}
