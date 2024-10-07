/* main.rs
   Este es el archivo principal que inicia la ejecución del simulador de
   asignación de particiones dinámicas. Maneja el flujo del programa.
*/


mod generador;
mod io_utils;
mod proceso;
mod particion;
mod estrategias;
mod simulacion;
mod confsim;
mod menu;


fn main() {
    // Muestra la pantalla de bienvenida antes de cualquier acción.
    menu::mostrar_bienvenida();
    
    // Inicia el programa mostrando el menú principal.
    menu::mostrar_menu();
}