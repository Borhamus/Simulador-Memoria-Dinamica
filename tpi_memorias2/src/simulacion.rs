// simulacion.rs


use crate::proceso::Proceso;
use crate::particion::Particion;
use crate::estrategias;
use crate::confsim::ConfiguracionSimulacion;
use std::io::{Write, BufWriter};
use std::fs::File;

// Estructura para llevar el control de los eventos simulados.
pub struct Evento {
    pub tiempo: u32,  // El tiempo en el que ocurre el evento.
    pub descripcion: String, // Descripción del evento (ejemplo: "Proceso X asignado a partición Y").
}

/// Función principal para simular la tanda de procesos.
pub fn ejecutar_simulacion(config: &ConfiguracionSimulacion, procesos: Vec<Proceso>, mut particiones: Vec<Particion>) {
    let mut tiempo_actual: u32 = 0; // Control del tiempo en la simulación.
    let mut eventos: Vec<Evento> = Vec::new(); // Lista para almacenar los eventos ocurridos.

    // Creamos el archivo de salida para registrar los eventos en la carpeta "files".
    let archivo_eventos: File = File::create("files/eventos_simulacion.txt").expect("No se pudo crear el archivo de eventos.");
    let mut escritor: BufWriter<File> = BufWriter::new(archivo_eventos);

    // Iteramos sobre cada proceso en la tanda de trabajos.
    for proceso in procesos {
        // Simulamos el tiempo del arribo del proceso.
        if proceso.arribo > tiempo_actual {
            tiempo_actual = proceso.arribo;
        }

        // Dependiendo de la estrategia seleccionada, intentamos asignar una partición.
        let mut indice: usize = 0; // Inicializamos un índice para usarlo en la estrategia.
        let particion_asignada: Option<usize> = match config.estrategia.as_str() {
            "First-fit" => estrategias::first_fit(&proceso, &mut particiones),
            "Best-fit" => estrategias::best_fit(&proceso, &mut particiones),
            "Next-fit" => estrategias::next_fit(&proceso, &mut particiones, &mut indice),
            "Worst-fit" => estrategias::worst_fit(&proceso, &mut particiones),
            _ => None, // En caso de que no se seleccione ninguna estrategia válida.
        };

        // Si se asignó una partición, registramos el evento.
        if let Some(indice) = particion_asignada {
            let evento: Evento = Evento {
                tiempo: tiempo_actual,
                descripcion: format!("Proceso {} asignado a la partición {}", proceso.get_nombre(), indice),
            };
            eventos.push(evento);
            writeln!(escritor, "Tiempo {}: Proceso {} asignado a la partición {}", tiempo_actual, proceso.get_nombre(), indice)
                .expect("No se pudo escribir en el archivo.");
        } else {
            let evento_fallo: Evento = Evento {
                tiempo: tiempo_actual,
                descripcion: format!("No se encontró una partición para el proceso {}", proceso.get_nombre()),
            };
            eventos.push(evento_fallo);
            writeln!(escritor, "Tiempo {}: No se encontró partición para el proceso {}", tiempo_actual, proceso.get_nombre())
                .expect("No se pudo escribir en el archivo.");
        }

        // Simulamos el tiempo que el proceso permanece en la memoria.
        tiempo_actual += proceso.get_duracion();
    }

    // Finalizamos la escritura en el archivo
    drop(escritor);

    // Mostramos los eventos registrados
    mostrar_eventos(&eventos);

    // Calculamos los indicadores de la simulación
    calcular_indicadores(&procesos, &particiones);

    println!("Simulación finalizada y archivo de eventos generado.");
}

/// Función que muestra los eventos que han ocurrido durante la simulación.
pub fn mostrar_eventos(eventos: &Vec<Evento>) {
    println!("--- Eventos de la Simulación ---");
    for evento in eventos {
        println!("Tiempo {}: {}", evento.tiempo, evento.descripcion);
    }
}

/// Cálculo de los indicadores
pub fn calcular_indicadores(procesos: &Vec<Proceso>, particiones: &Vec<Particion>) {
    let mut tiempo_total_retorno: u32 = 0;
    let mut fragmentacion_externa: u32 = 0;

    // Calcular el Tiempo de Retorno para cada proceso
    for proceso in procesos {
        let tiempo_retorno: u32 = proceso.arribo + proceso.duracion;
        tiempo_total_retorno += tiempo_retorno;
        println!("Proceso {}: Tiempo de Retorno {}", proceso.get_nombre(), tiempo_retorno);
    }

    // Calcular el Tiempo Medio de Retorno
    let tiempo_medio_retorno: f64 = tiempo_total_retorno as f64 / procesos.len() as f64;
    println!("Tiempo Medio de Retorno: {:.2}", tiempo_medio_retorno);

    // Calcular la Fragmentación Externa
    for particion in particiones {
        if particion.esta_libre() {
            fragmentacion_externa += particion.get_tamano();
        }
    }

    println!("Fragmentación Externa: {}", fragmentacion_externa);
}
