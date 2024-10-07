//proceso.rs

#[derive(Clone)]  // Para que podamos clonar procesos si es necesario
pub struct Proceso {
    pub nombre: String,
    pub arribo: usize,
    pub duracion: usize,
    pub memoria_requerida: u32,
}

impl Proceso {
    /// Crea un nuevo proceso
    pub fn new(nombre: &str, arribo: usize, duracion: usize, memoria_requerida: u32) -> Proceso {
        Proceso {
            nombre: nombre.to_string(),
            arribo,
            duracion,
            memoria_requerida,
        }
    }

    /// Obtener el nombre del proceso
    pub fn get_nombre(&self) -> &str {
        &self.nombre
    }

    /// Obtener el instante de arribo del proceso
    pub fn get_arribo(&self) -> usize {
        self.arribo
    }

    /// Obtener la duración del proceso
    pub fn get_duracion(&self) -> usize {
        self.duracion
    }

    /// Obtener la memoria requerida por el proceso
    pub fn get_memoria_requerida(&self) -> u32 {
        self.memoria_requerida
    }
}
