// menu.rs
//
// Este módulo muestra el menú principal al usuario y maneja la lógica de selección.
// Permite generar archivos de procesos, seleccionar archivos de procesos, configurar
// la simulación y ejecutarla, tanto en modo automático como en modo paso a paso.

use std::io::{self, Write};
use crate::generador::generar_archivo_procesos;
use crate::simulacion::{ejecutar_simulacion, simulacion_paso_a_paso}; // Asegúrate de que simulacion_paso_a_paso esté definida en simulacion.rs
use crate::confsim::ConfiguracionSimulacion;
use crate::particion::Particion;
use crate::estrategias::{EstrategiaAsignacion, FirstFit, BestFit, WorstFit, NextFit};
use crate::io_utils::{leer_archivo_procesos, limpiar_pantalla};

// Estructura que encapsula el estado de la simulación
struct EstadoSimulacion {
    archivo_procesos: Option<String>, // Nombre del archivo de procesos cargado
}

impl EstadoSimulacion {
    /// Crea un nuevo estado de simulación
    fn nuevo() -> Self {
        EstadoSimulacion {
            archivo_procesos: None,
        }
    }

    /// Asigna un archivo de procesos al estado de simulación
    fn set_archivo(&mut self, archivo: String) {
        self.archivo_procesos = Some(archivo);
    }

    /// Verifica si hay un archivo de procesos cargado
    fn archivo_cargado(&self) -> bool {
        self.archivo_procesos.is_some()
    }
}

/// Muestra el menú principal y maneja las opciones seleccionadas por el usuario.
pub fn mostrar_menu() {
    let mut estado_simulacion = EstadoSimulacion::nuevo();

    loop {
        limpiar_pantalla();

        println!("=================================================");
        println!("       MemSim3000 - Sistema de Simulación de Memoria");
        println!("=================================================");
        println!("Bienvenido a MemSim3000, un simulador de gestión de memoria.");
        println!("\nSeleccione una de las siguientes opciones:\n");

        println!("1. Generar un nuevo archivo de procesos.");
        println!("2. Seleccionar un archivo de procesos.");
        if estado_simulacion.archivo_cargado() {
            println!("3. Configurar y comenzar una simulación.");
            println!("4. Simulación paso a paso.");
        }
        println!("5. Salir.");

        let opcion = obtener_opcion_usuario();
        match opcion.as_str() {
            "1" => {
                generar_archivo_procesos();
                esperar_usuario();
            }
            "2" => {
                seleccionar_archivo_procesos(&mut estado_simulacion);
                esperar_usuario();
            }
            "3" => {
                if estado_simulacion.archivo_cargado() {
                    ejecutar_simulacion_normal(&estado_simulacion);
                } else {
                    println!("Por favor, seleccione un archivo de procesos antes de iniciar la simulación.");
                }
                esperar_usuario();
            }
            "4" => {
                if estado_simulacion.archivo_cargado() {
                    ejecutar_simulacion_paso_a_paso(&estado_simulacion);
                } else {
                    println!("Por favor, seleccione un archivo de procesos antes de iniciar la simulación.");
                }
                esperar_usuario();
            }
            "5" => {
                println!("Gracias por usar MemSim3000. ¡Hasta la próxima!");
                break;
            }
            _ => {
                println!("Opción no válida. Intente de nuevo.");
                esperar_usuario();
            }
        }
    }
}

/// Lee la opción seleccionada por el usuario.
fn obtener_opcion_usuario() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada.");
    input.trim().to_string()
}

/// Espera a que el usuario presione ENTER para continuar.
fn esperar_usuario() {
    println!("Presione ENTER para continuar...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

/// Permite al usuario seleccionar un archivo de procesos.
fn seleccionar_archivo_procesos(estado_simulacion: &mut EstadoSimulacion) {
    println!("Ingrese el nombre del archivo de procesos que desea cargar:");
    let mut archivo = String::new();
    io::stdin().read_line(&mut archivo).unwrap();
    let archivo = archivo.trim().to_string();

    if Path::new(&archivo).exists() {
        println!("Archivo {} cargado correctamente.", archivo);
        estado_simulacion.set_archivo(archivo);
    } else {
        println!("Archivo no encontrado. Intente nuevamente.");
    }
}

/// Configura y ejecuta la simulación en modo normal (automático).
fn ejecutar_simulacion_normal(estado_simulacion: &EstadoSimulacion) {
    let archivo_procesos = estado_simulacion.archivo_procesos.as_ref().unwrap();
    
    // Leer los procesos del archivo
    let procesos = leer_archivo_procesos(archivo_procesos).unwrap_or_else(|_| {
        println!("Error al leer el archivo de procesos.");
        vec![]
    });

    // Configurar la simulación
    let configuracion = configurar_simulacion();
    let particiones = vec![Particion::new(0, configuracion.memoria_total)];

    // Seleccionar la estrategia de asignación
    let mut estrategia: Box<dyn EstrategiaAsignacion> = seleccionar_estrategia(&configuracion);

    // Ejecutar la simulación
    ejecutar_simulacion(configuracion, procesos, particiones, &mut *estrategia);
}

/// Configura y ejecuta la simulación en modo paso a paso.
fn ejecutar_simulacion_paso_a_paso(estado_simulacion: &EstadoSimulacion) {
    let archivo_procesos = estado_simulacion.archivo_procesos.as_ref().unwrap();

    // Leer los procesos del archivo
    let procesos = leer_archivo_procesos(archivo_procesos).unwrap_or_else(|_| {
        println!("Error al leer el archivo de procesos.");
        vec![]
    });

    // Configurar la simulación
    let configuracion = configurar_simulacion();
    let particiones = vec![Particion::new(0, configuracion.memoria_total)];

    // Seleccionar la estrategia de asignación
    let mut estrategia: Box<dyn EstrategiaAsignacion> = seleccionar_estrategia(&configuracion);

    // Ejecutar la simulación en modo paso a paso
    simulacion_paso_a_paso(configuracion, procesos, particiones, &mut *estrategia);
}

/// Configura los parámetros de la simulación.
fn configurar_simulacion() -> ConfiguracionSimulacion {
    println!("Ingrese el tamaño total de la memoria física disponible:");
    let memoria_total = obtener_numero_usuario();
    
    println!("Ingrese el umbral de fragmentación externa (en unidades):");
    let umbral_fragmentacion = obtener_numero_usuario();

    println!("Ingrese el tamaño mínimo de partición para considerar como fragmentación:");
    let tamano_minimo_particion = obtener_numero_usuario();

    ConfiguracionSimulacion {
        memoria_total,
        umbral_fragmentacion,
        tamano_minimo_particion,
        tiempo_inicio_simulacion: 0, // Ajusta si lo necesitas
    }
}

/// Permite al usuario seleccionar la estrategia de asignación de memoria.
fn seleccionar_estrategia(config: &ConfiguracionSimulacion) -> Box<dyn EstrategiaAsignacion> {
    println!("Seleccione una estrategia de asignación de memoria:");
    println!("1. First-Fit");
    println!("2. Best-Fit");
    println!("3. Worst-Fit");
    println!("4. Next-Fit");

    let opcion = obtener_opcion_usuario();
    match opcion.as_str() {
        "1" => Box::new(FirstFit::new()),
        "2" => Box::new(BestFit::new()),
        "3" => Box::new(WorstFit::new()),
        "4" => Box::new(NextFit::new()),
        _ => {
            println!("Opción no válida. Seleccionando First-Fit por defecto.");
            Box::new(FirstFit::new())
        }
    }
}

/// Obtiene un número ingresado por el usuario.
fn obtener_numero_usuario() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or_else(|_| {
        println!("Entrada no válida, usando valor por defecto de 0.");
        0
    })
}
