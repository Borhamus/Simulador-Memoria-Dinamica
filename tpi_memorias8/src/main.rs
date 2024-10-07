//main.rs

mod menu;
mod manipulaciondearchivos;
mod simulacion;
mod memoria;
mod estrategias;
mod proceso;
mod particion;

use std::fs;
use std::path::Path;

fn main() {
    // Crear las carpetas necesarias
    crear_carpetas();

    // Iniciar el estado del menú
    let mut state = menu::MenuState::new();

    // Iniciar el menú principal con el estado
    menu::iniciar_menu(&mut state);
}

fn crear_carpetas() {
    // Definir las rutas de las carpetas que se van a crear
    let rutas = ["file/tandasdeprocesos", "file/simulaciones"];

    // Iterar sobre las rutas y crear cada carpeta si no existe
    for ruta in rutas.iter() {
        let path = Path::new(ruta);
        if !path.exists() {
            fs::create_dir_all(path).expect("No se pudo crear la carpeta");
            println!("Carpeta creada: {}", ruta);
        }
    }
}
