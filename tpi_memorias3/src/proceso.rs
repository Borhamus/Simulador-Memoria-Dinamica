//proceso.rs

/// Representa un proceso en el sistema.
pub struct Proceso {
    pub nombre: String,          // Nombre del proceso.
    pub arribo: u32,             // Tiempo en el que el proceso llega al sistema.
    pub duracion: u32,           // Duración del proceso (tiempo que debe permanecer en memoria).
    pub memoria_requerida: u32,  // Cantidad de memoria requerida por el proceso.
}

impl Proceso {
    /// Crea un nuevo proceso con los parámetros dados.
    pub fn new(nombre: String, arribo: u32, duracion: u32, memoria_requerida: u32) -> Self {
        Proceso {
            nombre,
            arribo,
            duracion,
            memoria_requerida,
        }
    }

    /// Obtiene el nombre del proceso.
    pub fn get_nombre(&self) -> &str {
        &self.nombre
    }

    /// Obtiene la cantidad de memoria requerida por el proceso.
    pub fn get_memoria_requerida(&self) -> u32 {
        self.memoria_requerida
    }

    /// Obtiene el tiempo de llegada del proceso.
    pub fn get_arribo(&self) -> u32 {
        self.arribo
    }

    /// Obtiene la duración del proceso.
    pub fn get_duracion(&self) -> u32 {
        self.duracion
    }

    pub fn set_duracion(&mut self, nueva_duracion: u32) {
        self.duracion = nueva_duracion;
    }
}
