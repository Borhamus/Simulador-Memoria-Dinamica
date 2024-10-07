//confsim.rs
// Este struct guarda toda la configuración de la simulación que el usuario elige en el menú.
pub struct ConfiguracionSimulacion {
    pub nombre_usuario: String,     // Nombre del usuario.
    pub tamanio_memoria: u32,       // Tamaño de la memoria física disponible.
    pub estrategia: String,         // Estrategia de asignación elegida (First-fit, Best-fit, etc.).
    pub tiempo_seleccion: u32,      // Tiempo de selección de partición.
    pub tiempo_carga: u32,          // Tiempo de carga promedio de un proceso.
    pub tiempo_liberacion: u32,     // Tiempo de liberación de la partición.
}

// Implementamos un método para mostrar un resumen de la configuración.
impl ConfiguracionSimulacion {
    // Muestra el resumen de la configuración seleccionada por el usuario.
    pub fn mostrar_resumen(&self) {
        println!("\n--- Resumen de la Configuración ---");
        println!("Usuario: {}", self.nombre_usuario);
        println!("Tamaño de la memoria: {} unidades", self.tamanio_memoria);
        println!("Estrategia seleccionada: {}", self.estrategia);
        println!("Tiempo de selección de partición: {} unidades", self.tiempo_seleccion);
        println!("Tiempo de carga: {} unidades", self.tiempo_carga);
        println!("Tiempo de liberación: {} unidades", self.tiempo_liberacion);
        println!("-----------------------------------");
    }

    // Valida si la configuración ingresada es razonable.
    pub fn validar_configuracion(&self) -> Result<(), String> {
        if self.tamanio_memoria == 0 {
            return Err("El tamaño de la memoria no puede ser 0.".to_string());
        }
        if self.tiempo_seleccion == 0 {
            return Err("El tiempo de selección de partición no puede ser 0.".to_string());
        }
        if self.tiempo_carga == 0 {
            return Err("El tiempo de carga no puede ser 0.".to_string());
        }
        if self.tiempo_liberacion == 0 {
            return Err("El tiempo de liberación no puede ser 0.".to_string());
        }
        Ok(())
    }
}
