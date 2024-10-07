// particion.rs

pub struct Particion {
    pub id: usize,
    pub direccion_comienzo: u32,
    pub tamanio: u32,
    pub estado: EstadoParticion,
}

pub enum EstadoParticion {
    Libre,
    Ocupada(String),
}
