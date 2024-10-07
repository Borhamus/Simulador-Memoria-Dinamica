// particion.rs
//
// Este módulo define la estructura y el comportamiento de las particiones de memoria
// que serán asignadas a los procesos. Incluye métodos para ocupar, liberar y
// modificar particiones, así como funciones auxiliares para manejar su estado.

/// Representa una partición de memoria en el sistema.
#[derive(Debug, Clone)]
pub struct Particion {
    direccion_comienzo: u32,    // Dirección de inicio de la partición en memoria.
    tamano: u32,                // Tamaño de la partición de memoria.
    estado: EstadoParticion,    // Estado de la partición (libre u ocupada).
    nombre_proceso: Option<String>, // Nombre del proceso que ocupa la partición (si está ocupada).
}

/// Enumera los posibles estados de una partición: Libre u Ocupada.
#[derive(Debug, Clone, PartialEq)]
pub enum EstadoParticion {
    Libre,
    Ocupada,
}

impl Particion {
    /// Crea una nueva partición con la dirección de comienzo y el tamaño dados.
    ///
    /// # Parámetros
    ///
    /// - `direccion_comienzo`: Dirección de inicio de la partición en memoria.
    /// - `tamano`: Tamaño de la partición en unidades de memoria.
    ///
    /// # Retorna
    ///
    /// - `Particion`: Una nueva instancia de `Particion`.
    pub fn new(direccion_comienzo: u32, tamano: u32) -> Self {
        Particion {
            direccion_comienzo,
            tamano,
            estado: EstadoParticion::Libre,
            nombre_proceso: None,
        }
    }

    /// Verifica si la partición está libre.
    ///
    /// # Retorna
    ///
    /// - `bool`: `true` si la partición está libre, `false` si está ocupada.
    pub fn esta_libre(&self) -> bool {
        self.estado == EstadoParticion::Libre
    }

    /// Ocupa la partición con el proceso dado.
    ///
    /// # Parámetros
    ///
    /// - `nombre_proceso`: Nombre del proceso que ocupará la partición.
    pub fn ocupar(&mut self, nombre_proceso: String) {
        self.estado = EstadoParticion::Ocupada;
        self.nombre_proceso = Some(nombre_proceso);
    }

    /// Libera la partición, dejándola disponible para otros procesos.
    pub fn liberar(&mut self) {
        self.estado = EstadoParticion::Libre;
        self.nombre_proceso = None;
    }

    /// Obtiene el tamaño de la partición.
    ///
    /// # Retorna
    ///
    /// - `u32`: Tamaño de la partición.
    pub fn get_tamano(&self) -> u32 {
        self.tamano
    }

    /// Establece un nuevo tamaño para la partición.
    ///
    /// # Parámetros
    ///
    /// - `nuevo_tamano`: Nuevo tamaño para la partición.
    pub fn set_tamano(&mut self, nuevo_tamano: u32) {
        self.tamano = nuevo_tamano;
    }

    /// Obtiene la dirección de comienzo de la partición.
    ///
    /// # Retorna
    ///
    /// - `u32`: Dirección de inicio de la partición.
    pub fn get_direccion_comienzo(&self) -> u32 {
        self.direccion_comienzo
    }

    /// Establece una nueva dirección de comienzo para la partición.
    ///
    /// # Parámetros
    ///
    /// - `nueva_direccion`: Nueva dirección de inicio para la partición.
    pub fn set_direccion_comienzo(&mut self, nueva_direccion: u32) {
        self.direccion_comienzo = nueva_direccion;
    }

    /// Obtiene el nombre del proceso que ocupa la partición, si está ocupada.
    ///
    /// # Retorna
    ///
    /// - `Option<&String>`: Nombre del proceso si la partición está ocupada, `None` si está libre.
    pub fn get_nombre_proceso(&self) -> Option<&String> {
        self.nombre_proceso.as_ref()
    }

    /// Obtiene una representación en cadena del estado de la partición.
    ///
    /// # Retorna
    ///
    /// - `String`: "Libre" o "Ocupada".
    pub fn get_estado(&self) -> &EstadoParticion {
        &self.estado
    }
}
