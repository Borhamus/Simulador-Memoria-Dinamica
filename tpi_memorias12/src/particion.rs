//particion.rs

//Todas las particiones miden 1kb.

#[derive(Debug, Clone)]
pub struct Particion {
    pub id_proceso: Option<String>, // Será None si la partición está libre
    pub tiempo_de_arribo: Option<u32>, // Será None si la partición está libre
    pub tiempo_de_vida: Option<u32>, // Será None inicialmente y se asignará cuando se ocupe
    pub estado: EstadoParticion,
}

impl Particion {
    /// Crea una nueva partición libre
    pub fn nueva() -> Particion {
        Particion {
            id_proceso: None, // Libre por defecto
            tiempo_de_arribo: None, // Ningún proceso asignado inicialmente
            tiempo_de_vida: None, // Ningún proceso asignado inicialmente
            estado: EstadoParticion::Libre,
        }
    }

    /// Ocupar la partición con un proceso específico
    pub fn ocupar(&mut self, nombre_proceso: String, tiempo_arribo: u32, tiempo_vida: u32) {
        self.id_proceso = Some(nombre_proceso);
        self.tiempo_de_arribo = Some(tiempo_arribo);
        self.tiempo_de_vida = Some(tiempo_vida);
        self.estado = EstadoParticion::Ocupada;
    }

    /// Liberar la partición
    pub fn liberar(&mut self) {
        self.id_proceso = None;
        self.tiempo_de_arribo = None;
        self.tiempo_de_vida = None;
        self.estado = EstadoParticion::Libre;
    }

    /// Verifica si la partición está libre
    pub fn esta_libre(&self) -> bool {
        matches!(self.estado, EstadoParticion::Libre)
    }
}

/// Define los estados posibles de una partición: libre o ocupada
#[derive(Debug, Clone)]
pub enum EstadoParticion {
    Libre,
    Ocupada,
}