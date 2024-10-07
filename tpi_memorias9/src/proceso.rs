#[derive(Clone)]
pub struct Proceso {
    pub nombre: String,
    pub arribo: usize,
    pub duracion: usize,
    pub memoria_requerida: u32,
    pub tiempo_inicio: Option<usize>,
    pub tiempo_fin: Option<usize>,
}

impl Proceso {
    /// Crea un nuevo proceso.
    pub fn new(nombre: &str, arribo: usize, duracion: usize, memoria_requerida: u32) -> Proceso {
        Proceso {
            nombre: nombre.to_string(),
            arribo,
            duracion,
            memoria_requerida,
            tiempo_inicio: None,
            tiempo_fin: None,
        }
    }

    /// Define el tiempo de inicio del proceso.
    pub fn set_tiempo_inicio(&mut self, tiempo: usize) {
        self.tiempo_inicio = Some(tiempo);
    }

    /// Define el tiempo de fin del proceso.
    pub fn set_tiempo_fin(&mut self, tiempo: usize) {
        self.tiempo_fin = Some(tiempo);
    }

    /// Devuelve el nombre del proceso.
    pub fn get_nombre(&self) -> &str {
        &self.nombre
    }

    /// Devuelve el tiempo de inicio del proceso.
    pub fn get_tiempo_inicio(&self) -> Option<usize> {
        self.tiempo_inicio
    }

    /// Devuelve el tiempo de fin del proceso.
    pub fn get_tiempo_fin(&self) -> Option<usize> {
        self.tiempo_fin
    }

    /// Devuelve el arribo del proceso.
    pub fn get_arribo(&self) -> usize {
        self.arribo
    }

    /// Devuelve la duración del proceso.
    pub fn get_duracion(&self) -> usize {
        self.duracion
    }
}
