// proceso.rs
//
// Este módulo define la estructura `Proceso` y sus métodos asociados.


#[derive(Debug, Clone, PartialEq, Eq)]  // Agregamos PartialEq y Eq para la comparación
pub struct Proceso {
    nombre: String,
    arribo: u32,         // Tiempo de llegada del proceso
    duracion: u32,       // Duración total del proceso
    memoria_requerida: u32,  // Cantidad de memoria que el proceso necesita
}

impl Proceso {
    /// Crea un nuevo proceso.
    ///
    /// # Parámetros
    /// - `nombre`: Nombre del proceso.
    /// - `arribo`: Tiempo de arribo del proceso.
    /// - `duracion`: Duración total del proceso.
    /// - `memoria_requerida`: Cantidad de memoria que el proceso necesita.
    pub fn new(nombre: String, arribo: u32, duracion: u32, memoria_requerida: u32) -> Self {
        Proceso {
            nombre,
            arribo,
            duracion,
            memoria_requerida,
        }
    }

    /// Devuelve el nombre del proceso.
    pub fn get_nombre(&self) -> &str {
        &self.nombre
    }

    /// Devuelve el tiempo de arribo del proceso.
    pub fn get_arribo(&self) -> u32 {
        self.arribo
    }

    /// Devuelve la duración total del proceso.
    pub fn get_duracion(&self) -> u32 {
        self.duracion
    }

    /// Devuelve la cantidad de memoria requerida por el proceso.
    pub fn get_memoria_requerida(&self) -> u32 {
        self.memoria_requerida
    }
}
