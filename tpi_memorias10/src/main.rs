//main.rs

mod config;
mod estrategias;
mod generador;
mod manipular_archivo;
mod memoria;
mod nueva_simulacion;
mod particion;
mod proceso;
mod simulador;
mod visualizador;


fn limpiar_consola() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}

fn crear_carpetas() {
    // Crear carpetas si no existen
    let path = std::path::Path::new("files");
    if !path.exists() {
        std::fs::create_dir_all(path).expect("No se pudo crear la carpeta 'files'");
    }
}

fn menu() {
    loop {
        println!("1) Nueva Simulación");
        println!("2) Ver simulaciones anteriores");
        println!("3) Salir");

        let mut opcion = String::new();
        std::io::stdin().read_line(&mut opcion).expect("Error al leer opción");

        match opcion.trim() {
            "1" => nueva_simulacion::NuevaSimulacion::nueva_tanda(),
            "2" => visualizador::mostrar_simulaciones(),
            "3" => break,
            _ => println!("Opción no válida"),
        }
    }
}

fn main() {
    limpiar_consola();
    crear_carpetas();
    menu();
}
