// main.rs
//
// Este es el archivo principal que inicia la ejecución del simulador de
// asignación de particiones dinámicas. Maneja el flujo del programa.

mod confsim;
mod estrategias;
mod generador;
mod io_utils;
mod menu;
mod particion;
mod proceso;
mod simulacion;

fn main() {

    // Inicia el programa mostrando el menú principal.
    menu::mostrar_menu();
}