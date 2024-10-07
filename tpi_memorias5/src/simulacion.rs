// simulacion.rs
//
// Este módulo contiene la lógica principal de la simulación. Ejecuta la asignación
// de procesos a particiones según la estrategia seleccionada, maneja los eventos
// de llegada y finalización de procesos, y genera los resultados de la simulación.
// Además, utiliza el módulo `generador.rs` para manejar la creación y guardado de archivos.

use crate::proceso::Proceso;
use crate::particion::Particion;
use crate::confsim::{ConfiguracionSimulacion, Estrategia};
use crate::estrategias::{FirstFit, BestFit, NextFit, WorstFit, EstrategiaAsignacion};
use crate::generador::guardar_simulacion;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

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
#[derive(Debug, Clone, Eq, PartialEq)]
struct EventoSimulado {
    tiempo: u32,
    evento: Evento,
}

impl Ord for EventoSimulado {
    fn cmp(&self, other: &Self) -> Ordering {
        // Queremos que el evento con el menor tiempo tenga mayor prioridad
        // Por eso comparamos al revés
        other.tiempo.cmp(&self.tiempo)
    }
}

impl PartialOrd for EventoSimulado {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Estructura principal de la simulación.
pub struct Simulacion {
    tiempo_actual: u32,
    eventos: BinaryHeap<EventoSimulado>,
    particiones: Vec<Particion>,
    estrategia: Box<dyn EstrategiaAsignacion>,
    configuracion: ConfiguracionSimulacion,
    procesos: Vec<Proceso>,
    log_eventos: Vec<String>,
}

impl Simulacion {
    /// Crea una nueva instancia de la simulación.
    pub fn new(configuracion: ConfiguracionSimulacion, procesos: Vec<Proceso>) -> Self {
        // Inicializamos la partición única con el tamaño de la memoria disponible
        let mut particiones = Vec::new();
        particiones.push(Particion::new(0, configuracion.tamanio_memoria));

        // Seleccionamos la estrategia de asignación
        let estrategia = obtener_estrategia(&configuracion.estrategia);

        // Inicializamos la cola de eventos con los procesos de llegada
        let mut eventos = BinaryHeap::new();
        for proceso in &procesos {
            eventos.push(EventoSimulado {
                tiempo: proceso.get_arribo(),
                evento: Evento::LlegadaProceso(proceso.clone()),
            });
        }

        Simulacion {
            tiempo_actual: 0,
            eventos,
            particiones,
            estrategia,
            configuracion,
            procesos,
            log_eventos: Vec::new(),
        }
    }

    /// Ejecuta la simulación completa.
    pub fn ejecutar(&mut self) {
        while let Some(evento_simulado) = self.eventos.pop() {
            self.tiempo_actual = evento_simulado.tiempo;

            match evento_simulado.evento {
                Evento::LlegadaProceso(mut proceso) => {
                    self.log_evento(format!(
                        "Tiempo {}: Llegada del proceso {}",
                        self.tiempo_actual,
                        proceso.get_nombre()
                    ));

                    // Intentar asignar el proceso a una partición
                    let indice_particion =
                        self.estrategia.asignar(&proceso, &mut self.particiones);

                    if let Some(indice) = indice_particion {
                        // Proceso asignado exitosamente
                        self.log_evento(format!(
                            "Tiempo {}: Proceso {} asignado a la partición {}",
                            self.tiempo_actual,
                            proceso.get_nombre(),
                            indice
                        ));

                        // Registrar el tiempo de inicio del proceso
                        proceso.set_tiempo_inicio(self.tiempo_actual);

                        // Agregar evento de fin de proceso
                        let tiempo_finalizacion = self.tiempo_actual
                            + self.configuracion.tiempo_carga
                            + self.configuracion.tiempo_seleccion
                            + proceso.get_duracion();

                        self.eventos.push(EventoSimulado {
                            tiempo: tiempo_finalizacion,
                            evento: Evento::FinProceso {
                                nombre_proceso: proceso.get_nombre().to_string(),
                                indice_particion: indice,
                            },
                        });

                        // Actualizar el proceso en la lista de procesos
                        if let Some(p) = self
                            .procesos
                            .iter_mut()
                            .find(|p| p.get_nombre() == proceso.get_nombre())
                        {
                            p.set_tiempo_inicio(proceso.get_tiempo_inicio().unwrap());
                        }
                    } else {
                        // No se pudo asignar el proceso
                        self.log_evento(format!(
                            "Tiempo {}: Proceso {} no pudo ser asignado (esperando)",
                            self.tiempo_actual,
                            proceso.get_nombre()
                        ));

                        // Reintentar más adelante (aumentamos el tiempo en 1 unidad)
                        self.eventos.push(EventoSimulado {
                            tiempo: self.tiempo_actual + 1,
                            evento: Evento::LlegadaProceso(proceso),
                        });
                    }
                }
                Evento::FinProceso {
                    nombre_proceso,
                    indice_particion,
                } => {
                    self.log_evento(format!(
                        "Tiempo {}: Finalización del proceso {} en la partición {}",
                        self.tiempo_actual,
                        nombre_proceso,
                        indice_particion
                    ));

                    // Liberar la partición
                    self.particiones[indice_particion].liberar();

                    // Combinar particiones libres adyacentes
                    combinar_particiones(&mut self.particiones);

                    // Registrar el tiempo de finalización del proceso
                    if let Some(pos) = self
                        .procesos
                        .iter()
                        .position(|p| p.get_nombre() == nombre_proceso)
                    {
                        {
                            let proceso = &mut self.procesos[pos];
                            proceso.set_tiempo_finalizacion(self.tiempo_actual);
                        }

                        let proceso = &self.procesos[pos];
                        self.log_evento(format!(
                            "Proceso {} completado. Tiempo de retorno: {}",
                            proceso.get_nombre(),
                            proceso.tiempo_retorno().unwrap()
                        ));
                    }
                }
            }
        }

        // Al finalizar, generar los informes y guardar los resultados
        self.generar_informes();
    }

    /// Registra un evento en el log de eventos.
    fn log_evento(&mut self, mensaje: String) {
        println!("{}", mensaje); // También lo mostramos en pantalla
        self.log_eventos.push(mensaje);
    }

    /// Genera los informes y guarda los resultados en archivos.
    fn generar_informes(&self) {
        // Calcular indicadores
        let tiempos_retorno: Vec<u32> = self
            .procesos
            .iter()
            .filter_map(|p| p.tiempo_retorno())
            .collect();

        let tiempo_total_retorno: u32 = tiempos_retorno.iter().sum();
        let tiempo_medio_retorno = tiempo_total_retorno as f32 / tiempos_retorno.len() as f32;

        // Calcular índice de fragmentación externa
        let memoria_total_libre: u32 = self
            .particiones
            .iter()
            .filter(|p| p.esta_libre())
            .map(|p| p.get_tamano())
            .sum();

        let fragmentacion_externa =
            (memoria_total_libre as f32 / self.configuracion.tamanio_memoria as f32) * 100.0;

        // Mostrar resultados en pantalla
        println!("=== Resultados de la Simulación ===");
        for proceso in &self.procesos {
            if let Some(tiempo_retorno) = proceso.tiempo_retorno() {
                println!(
                    "Proceso {}: Tiempo de Retorno = {}",
                    proceso.get_nombre(),
                    tiempo_retorno
                );
            } else {
                println!("Proceso {}: No completado", proceso.get_nombre());
            }
        }
        println!("Tiempo Medio de Retorno: {:.2}", tiempo_medio_retorno);
        println!(
            "Índice de Fragmentación Externa: {:.2}%",
            fragmentacion_externa
        );

        // Guardar los resultados de la simulación utilizando `generador.rs`
        if let Err(e) = guardar_simulacion(self) {
            println!("Error al guardar la simulación: {}", e);
        }
    }

    /// Calcula el tiempo medio de retorno de los procesos.
    pub fn calcular_tiempo_medio_retorno(&self) -> f32 {
        let tiempos_retorno: Vec<u32> = self
            .procesos
            .iter()
            .filter_map(|p| p.tiempo_retorno())
            .collect();

        let tiempo_total_retorno: u32 = tiempos_retorno.iter().sum();
        tiempo_total_retorno as f32 / tiempos_retorno.len() as f32
    }

    /// Calcula el índice de fragmentación externa.
    pub fn calcular_fragmentacion_externa(&self) -> f32 {
        let memoria_total_libre: u32 = self
            .particiones
            .iter()
            .filter(|p| p.esta_libre())
            .map(|p| p.get_tamano())
            .sum();

        (memoria_total_libre as f32 / self.configuracion.tamanio_memoria as f32) * 100.0
    }

    /// Obtiene una referencia a los procesos de la simulación.
    pub fn get_procesos(&self) -> &Vec<Proceso> {
        &self.procesos
    }

    /// Obtiene una referencia a la configuración de la simulación.
    pub fn get_configuracion(&self) -> &ConfiguracionSimulacion {
        &self.configuracion
    }

    /// Obtiene una referencia al log de eventos de la simulación.
    pub fn get_log_eventos(&self) -> &Vec<String> {
        &self.log_eventos
    }
}

/// Combina particiones libres adyacentes para reducir la fragmentación externa.
///
/// # Parámetros
///
/// - `particiones`: Referencia mutable al vector de particiones.
fn combinar_particiones(particiones: &mut Vec<Particion>) {
    let mut i = 0;
    while i < particiones.len() - 1 {
        if particiones[i].esta_libre() && particiones[i + 1].esta_libre() {
            // Combinar las particiones
            let nuevo_tamano =
                particiones[i].get_tamano() + particiones[i + 1].get_tamano();
            particiones[i].set_tamano(nuevo_tamano);
            particiones.remove(i + 1);
            // No incrementamos i para verificar si la siguiente partición también está libre
        } else {
            i += 1;
        }
    }
}

/// Obtiene la estrategia de asignación correspondiente.
///
/// # Parámetros
///
/// - `estrategia`: Referencia a la estrategia seleccionada.
///
/// # Retorna
///
/// - `Box<dyn EstrategiaAsignacion>`: Implementación de la estrategia de asignación.
fn obtener_estrategia(estrategia: &Estrategia) -> Box<dyn EstrategiaAsignacion> {
    match estrategia {
        Estrategia::FirstFit => Box::new(FirstFit),
        Estrategia::BestFit => Box::new(BestFit),
        Estrategia::WorstFit => Box::new(WorstFit),
        Estrategia::NextFit => Box::new(NextFit::new()),
    }
}

/// Ejecuta la simulación con la configuración y procesos dados.
///
/// # Parámetros
///
/// - `config`: Configuración de la simulación.
/// - `procesos`: Vector de procesos a simular.
pub fn ejecutar_simulacion(config: ConfiguracionSimulacion, procesos: Vec<Proceso>) {
    let mut simulacion = Simulacion::new(config, procesos);
    simulacion.ejecutar();
}
