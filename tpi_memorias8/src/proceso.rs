//proceso.rs

#[derive(Debug, Clone)]
pub struct Proceso {
    pub nombre: String,
    pub instante_arribo: u32,
    pub duracion_total: u32,
    pub memoria_requerida: u32,
    pub tiempo_inicio: u32,
    pub tiempo_fin: u32,
}