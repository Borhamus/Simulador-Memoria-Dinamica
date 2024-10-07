// proceso.rs
//
// Este módulo define la estructura `Proceso` y sus métodos asociados.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proceso {
    nombre: String,
    arribo: u32,
    duracion: u32,
    memoria_requerida: u32,
    tiempo_inicio: Option<u32>,
    tiempo_finalizacion: Option<u32>,
}

impl Proceso {
    /// Crea una nueva instancia de `Proceso`.
    pub fn new(nombre: String, arribo: u32, duracion: u32, memoria_requerida: u32) -> Self {
        Self {
            nombre,
            arribo,
            duracion,
            memoria_requerida,
            tiempo_inicio: None,
            tiempo_finalizacion: None,
        }
    }

    /// Obtiene el nombre del proceso.
    pub fn get_nombre(&self) -> &str {
        &self.nombre
    }

    /// Obtiene el tiempo de arribo del proceso.
    pub fn get_arribo(&self) -> u32 {
        self.arribo
    }

    /// Obtiene la duración del proceso.
    pub fn get_duracion(&self) -> u32 {
        self.duracion
    }

    /// Obtiene la memoria requerida por el proceso.
    pub fn get_memoria_requerida(&self) -> u32 {
        self.memoria_requerida
    }

    /// Establece el tiempo de inicio del proceso.
    pub fn set_tiempo_inicio(&mut self, tiempo: u32) {
        self.tiempo_inicio = Some(tiempo);
    }

    /// Obtiene el tiempo de inicio del proceso.
    pub fn get_tiempo_inicio(&self) -> Option<u32> {
        self.tiempo_inicio
    }

    /// Establece el tiempo de finalización del proceso.
    pub fn set_tiempo_finalizacion(&mut self, tiempo: u32) {
        self.tiempo_finalizacion = Some(tiempo);
    }

    /// Obtiene el tiempo de finalización del proceso.
    pub fn get_tiempo_finalizacion(&self) -> Option<u32> {
        self.tiempo_finalizacion
    }

    /// Calcula el tiempo de retorno del proceso.
    pub fn tiempo_retorno(&self) -> Option<u32> {
        if let (Some(tiempo_final), Some(tiempo_inicio)) = (self.tiempo_finalizacion, self.tiempo_inicio) {
            Some(tiempo_final - self.arribo)
        } else {
            None
        }
    }
}
