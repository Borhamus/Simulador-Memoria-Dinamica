// simulacion.rs
//
// Este módulo contiene la lógica principal de la simulación. Ejecuta la asignación
// de procesos a particiones según la estrategia seleccionada, maneja los eventos
// de llegada y finalización de procesos, y genera los resultados de la simulación.
// Además, utiliza el módulo `generador.rs` para manejar la creación y guardado de archivos.


use std::collections::BinaryHeap;
use std::cmp::Ordering;
use crate::generador::guardar_simulacion;
use crate::proceso::Proceso;
use crate::particion::Particion;
use crate::confsim::ConfiguracionSimulacion;
use crate::estrategias::EstrategiaAsignacion;

/// Representa un evento en la simulación.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Evento {
    LlegadaProceso(Proceso),
    FinProceso {
        nombre_proceso: String,
        indice_particion: usize,
    },
}

/// Estructura para manejar los eventos en el tiempo.
#[derive(Debug, Eq, PartialEq)]
struct EventoSimulacion {
    tiempo: u32,
    evento: Evento,
}

impl Ord for EventoSimulacion {
    fn cmp(&self, other: &Self) -> Ordering {
        other.tiempo.cmp(&self.tiempo)
    }
}

impl PartialOrd for EventoSimulacion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Estructura principal para la simulación.
pub struct Simulacion {
    configuracion: ConfiguracionSimulacion,
    procesos: Vec<Proceso>,
    eventos: BinaryHeap<EventoSimulacion>,
    particiones: Vec<Particion>,  // Lista de particiones
    resultados: Vec<String>,      // Guarda los resultados de la simulación.
    tiempos_retorno: Vec<u32>,    // Almacena los tiempos de retorno de los procesos.
}

impl Simulacion {
    /// Crea una nueva simulación con la configuración y los procesos proporcionados.
    pub fn new(configuracion: ConfiguracionSimulacion, procesos: Vec<Proceso>, particiones: Vec<Particion>) -> Self {
        Simulacion {
            configuracion,
            procesos,
            eventos: BinaryHeap::new(),
            particiones,
            resultados: Vec::new(),
            tiempos_retorno: Vec::new(),
        }
    }

    /// Ejecuta la simulación, procesando los eventos en el orden adecuado.
    pub fn ejecutar(&mut self, estrategia: &mut dyn EstrategiaAsignacion) {
        // Inicializar la simulación (por ejemplo, agregar los eventos de llegada de procesos)
        for proceso in &self.procesos {
            let evento_llegada = EventoSimulacion {
                tiempo: proceso.get_arribo(),
                evento: Evento::LlegadaProceso(proceso.clone()),
            };
            self.eventos.push(evento_llegada);
        }

        // Procesar los eventos mientras existan en la cola
        while let Some(evento_simulacion) = self.eventos.pop() {
            match evento_simulacion.evento {
                Evento::LlegadaProceso(proceso) => {
                    self.procesar_llegada_proceso(proceso, evento_simulacion.tiempo, estrategia);
                }
                Evento::FinProceso { nombre_proceso, indice_particion } => {
                    self.procesar_fin_proceso(nombre_proceso, indice_particion, evento_simulacion.tiempo);
                }
            }
        }

        // Al finalizar, generar los informes y guardar los resultados
        self.generar_informes();
    }

    /// Procesa la llegada de un proceso a la simulación.
    fn procesar_llegada_proceso(&mut self, proceso: Proceso, tiempo_actual: u32, estrategia: &mut dyn EstrategiaAsignacion) {
        // Intentar asignar la partición usando la estrategia seleccionada
        if let Some(indice_particion) = estrategia.asignar(&proceso, &mut self.particiones) {
            // Lógica cuando el proceso es asignado a una partición
            self.resultados.push(format!(
                "Tiempo {}: Proceso {} asignado a partición {}.",
                tiempo_actual, proceso.get_nombre(), indice_particion
            ));

            // Programar el evento de finalización
            let evento_fin = EventoSimulacion {
                tiempo: tiempo_actual + proceso.get_duracion(),
                evento: Evento::FinProceso {
                    nombre_proceso: proceso.get_nombre().to_string(),
                    indice_particion,
                },
            };
            self.eventos.push(evento_fin);
        } else {
            // Si no hay particiones disponibles, no se puede asignar el proceso
            self.resultados.push(format!(
                "Tiempo {}: No se pudo asignar partición para el proceso {}.",
                tiempo_actual, proceso.get_nombre()
            ));
        }
    }

    /// Procesa la finalización de un proceso en la simulación.
    fn procesar_fin_proceso(&mut self, nombre_proceso: String, indice_particion: usize, tiempo_actual: u32) {
        // Lógica para manejar la finalización del proceso (e.g., liberar partición)
        self.particiones[indice_particion].liberar();

        self.resultados.push(format!(
            "Tiempo {}: Proceso {} ha finalizado y la partición {} se libera.",
            tiempo_actual, nombre_proceso, indice_particion
        ));

        // Calcular el tiempo de retorno para el proceso
        let tiempo_llegada = self.procesos.iter().find(|p| p.get_nombre() == nombre_proceso).unwrap().get_arribo();
        let tiempo_retorno = tiempo_actual - tiempo_llegada;
        self.tiempos_retorno.push(tiempo_retorno);

        // Calcular la fragmentación externa
        let fragmentacion_externa = self.calcular_fragmentacion_externa();
        self.resultados.push(format!(
            "Fragmentación externa actual: {} unidades de memoria no utilizable.",
            fragmentacion_externa
        ));
    }

    /// Calcula la fragmentación externa.
    fn calcular_fragmentacion_externa(&self) -> u32 {
        let mut fragmentacion = 0;
        for particion in &self.particiones {
            if particion.esta_libre() {
                fragmentacion += particion.get_tamano();
            }
        }
        fragmentacion
    }

    /// Genera los informes y guarda los resultados en archivos.
    fn generar_informes(&self) {
        // Nombre del archivo de simulación (puedes cambiar la convención de nombres si lo deseas)
        let nombre_simulacion = "resultado_simulacion";

        // Guardar los resultados de la simulación utilizando `generador.rs`
        if let Err(e) = guardar_simulacion(nombre_simulacion, self.resultados.clone()) {
            println!("Error al guardar la simulación: {}", e);
        }

        // Calcular y mostrar el tiempo promedio de retorno
        let tiempo_total_retorno: u32 = self.tiempos_retorno.iter().sum();
        let tiempo_promedio_retorno = tiempo_total_retorno as f64 / self.tiempos_retorno.len() as f64;

        println!("Tiempo medio de retorno: {:.2} unidades de tiempo.", tiempo_promedio_retorno);
    }
}

/// Función principal para ejecutar la simulación.
///
/// # Parámetros
/// - `config`: Configuración de la simulación.
/// - `procesos`: Vector de procesos a simular.
pub fn ejecutar_simulacion(config: ConfiguracionSimulacion, procesos: Vec<Proceso>, particiones: Vec<Particion>, estrategia: &mut dyn EstrategiaAsignacion) {
    let mut simulacion = Simulacion::new(config, procesos, particiones);
    simulacion.ejecutar(estrategia);
}
