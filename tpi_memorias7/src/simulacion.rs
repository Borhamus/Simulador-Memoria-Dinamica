// simulacion.rs
//
// Este módulo contiene la lógica principal de la simulación. Ejecuta la asignación
// de procesos a particiones según la estrategia seleccionada, maneja los eventos
// de llegada y finalización de procesos, y genera los resultados de la simulación.
// Además, utiliza el módulo `generador.rs` para manejar la creación y guardado de archivos.

use crate::generador::guardar_simulacion;
use crate::proceso::Proceso;
use crate::particion::{Particion, mostrar_estado_memoria, coalescer_particiones, calcular_fragmentacion_externa, recompactar_memoria};
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.tiempo.cmp(&self.tiempo) // Ordenamos en base al tiempo (menor primero)
    }
}

impl PartialOrd for EventoSimulacion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Estructura que maneja el flujo de la simulación.
pub struct Simulacion {
    config: ConfiguracionSimulacion,
    procesos: Vec<Proceso>,
    particiones: Vec<Particion>,
    eventos: Vec<EventoSimulacion>,
    tiempos_retorno: Vec<u32>,
    resultados: Vec<String>,
}

impl Simulacion {
    /// Crea una nueva simulación con la configuración y procesos dados.
    pub fn new(config: ConfiguracionSimulacion, procesos: Vec<Proceso>, particiones: Vec<Particion>) -> Self {
        let eventos = Vec::new(); // Aquí se llenarán los eventos de llegada de procesos
        Simulacion {
            config,
            procesos,
            particiones,
            eventos,
            tiempos_retorno: Vec::new(),
            resultados: Vec::new(),
        }
    }

    /// Ejecuta la simulación utilizando la estrategia de asignación seleccionada.
    ///
    /// # Parámetros
    /// - `estrategia`: La estrategia de asignación de memoria a usar (First-Fit, Best-Fit, etc.).
    pub fn ejecutar(&mut self, estrategia: &mut dyn EstrategiaAsignacion) {
        // Registrar la llegada de cada proceso en la lista de eventos
        for proceso in &self.procesos {
            self.eventos.push(EventoSimulacion {
                tiempo: proceso.arribo,
                evento: Evento::LlegadaProceso(proceso.clone()),
            });
        }

        // Ordenar los eventos por tiempo de llegada
        self.eventos.sort();

        // Procesar los eventos de la simulación
        while let Some(evento_simulacion) = self.eventos.pop() {
            match evento_simulacion.evento {
                Evento::LlegadaProceso(proceso) => {
                    self.procesar_llegada_proceso(proceso, estrategia);
                }
                Evento::FinProceso { nombre_proceso, indice_particion } => {
                    self.procesar_fin_proceso(nombre_proceso, indice_particion);
                }
            }

            // Mostrar el estado de la memoria después de cada evento
            mostrar_estado_memoria(&self.particiones);

            // Coalescing: combinar particiones libres adyacentes
            coalescer_particiones(&mut self.particiones);

            // Verificar si se requiere recompactación
            let fragmentacion = calcular_fragmentacion_externa(&self.particiones, self.config.tamano_minimo_particion);
            if fragmentacion > self.config.umbral_fragmentacion {
                println!("Fragmentación externa detectada: {} unidades. Aplicando recompactación...", fragmentacion);
                recompactar_memoria(&mut self.particiones);
            }
        }

        // Al final de la simulación, generar los informes y resultados
        self.generar_informes();
    }

    /// Procesa la llegada de un proceso a la memoria.
    ///
    /// # Parámetros
    /// - `proceso`: El proceso que llega.
    /// - `estrategia`: La estrategia de asignación de memoria seleccionada.
    fn procesar_llegada_proceso(&mut self, proceso: Proceso, estrategia: &mut dyn EstrategiaAsignacion) {
        // Intentar asignar el proceso a una partición según la estrategia
        if let Some(indice_particion) = estrategia.asignar(&proceso, &mut self.particiones) {
            // Registrar el evento
            self.resultados.push(format!("Proceso {} asignado a la partición {}.", proceso.nombre, indice_particion));

            // Registrar evento de finalización del proceso
            self.eventos.push(EventoSimulacion {
                tiempo: proceso.arribo + proceso.duracion,
                evento: Evento::FinProceso {
                    nombre_proceso: proceso.nombre.clone(),
                    indice_particion,
                },
            });
        } else {
            // No se encontró partición libre para el proceso
            self.resultados.push(format!("No se pudo asignar el proceso {} por falta de memoria.", proceso.nombre));
        }
    }

    /// Procesa el final de un proceso, liberando su partición.
    ///
    /// # Parámetros
    /// - `nombre_proceso`: Nombre del proceso que finaliza.
    /// - `indice_particion`: Índice de la partición que ocupaba el proceso.
    fn procesar_fin_proceso(&mut self, nombre_proceso: String, indice_particion: usize) {
        // Liberar la partición que ocupaba el proceso
        self.particiones[indice_particion].liberar();
        self.resultados.push(format!("Proceso {} terminó y liberó la partición {}.", nombre_proceso, indice_particion));

        // Calcular el tiempo de retorno del proceso (tiempo actual - tiempo de llegada)
        let tiempo_actual = self.eventos.last().map_or(0, |e| e.tiempo); // Tiempo actual
        let tiempo_retorno = tiempo_actual - self.config.tiempo_inicio_simulacion;
        self.tiempos_retorno.push(tiempo_retorno);

        // Registrar la fragmentación externa
        let fragmentacion_externa = calcular_fragmentacion_externa(&self.particiones, self.config.tamano_minimo_particion);
        self.resultados.push(format!(
            "Fragmentación externa actual: {} unidades de memoria no utilizable.",
            fragmentacion_externa
        ));
    }

    /// Genera los informes y guarda los resultados de la simulación.
    fn generar_informes(&self) {
        // Nombre del archivo de simulación
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
/// - `particiones`: Vector de particiones disponibles.
/// - `estrategia`: Referencia mutable a la estrategia de asignación de memoria seleccionada.
pub fn ejecutar_simulacion(
    config: ConfiguracionSimulacion,
    procesos: Vec<Proceso>,
    particiones: Vec<Particion>,
    estrategia: &mut dyn EstrategiaAsignacion,
) {
    let mut simulacion = Simulacion::new(config, procesos, particiones);
    simulacion.ejecutar(estrategia);
}
