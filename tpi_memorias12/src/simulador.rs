use crate::particion::{Particion, EstadoParticion};
use crate::proceso::Proceso;
use crate::config::Config;

pub struct Simulador;

impl Simulador {
    pub fn ejecutar_simulacion(
        procesos: &[Proceso],
        configuracion: &Config,
        eventos: &mut Vec<String>,
        resultados: &mut Vec<String>,
    ) {
        let mut tiempo_global: u64 = 0;
        let mut indice_proceso_actual: usize = 0;
        let mut vector_memoria = crear_vector_memoria(configuracion.tamanio_memoria);
        let mut ultima_asignada: usize = 0;
        let mut procesos_liberados: Vec<String> = Vec::new();
        let mut tiempos_de_retorno: Vec<u64> = vec![0; procesos.len()];
        let mut particiones_libres_totales: u64 = 0;

        // Continuar hasta que todos los procesos hayan sido asignados y liberados
        while !fin_de_la_tanda(indice_proceso_actual, procesos.len(), &vector_memoria) {
            let mut hubo_cambio = false;

            // ===========================
            // Parte 1: Liberación de memoria
            // ===========================
            let memoria_liberada = liberar_memoria(
                &mut vector_memoria,
                configuracion,
                &mut procesos_liberados,
            );
            if memoria_liberada {
                if !procesos_liberados.is_empty() {
                    let memoria_ocupada = calcular_memoria_ocupada(&vector_memoria);
                    eventos.push(format!(
                        "En el tiempo global {}, se liberó memoria de los procesos finalizados: {}. (Memoria Ocupada: {} KB de {} total)",
                        tiempo_global,
                        procesos_liberados.join(", "),
                        memoria_ocupada,
                        configuracion.tamanio_memoria
                    ));
                    procesos_liberados.clear();
                }
                hubo_cambio = true;
            }

            // ===========================
            // Parte 2: Asignación de procesos
            // ===========================
            // Intentar asignar procesos que arriban en el tiempo actual
            while indice_proceso_actual < procesos.len() && procesos[indice_proceso_actual].arribo as u64 <= tiempo_global {
                let proceso_actual = &procesos[indice_proceso_actual];
            
                let proceso_asignado = configuracion.estrategia.asignar_proceso(
                    &mut vector_memoria,
                    proceso_actual,
                    &mut ultima_asignada,
                );
            
                // En la asignación exitosa del proceso
                if let Some(_) = proceso_asignado {
                    let memoria_ocupada = calcular_memoria_ocupada(&vector_memoria);
                    eventos.push(format!(
                        "En el tiempo global {}, el proceso {} fue asignado correctamente. (Memoria Ocupada: {} KB de {} total)",
                        tiempo_global,
                        proceso_actual.nombre,
                        memoria_ocupada,
                        configuracion.tamanio_memoria
                    ));
                    tiempos_de_retorno[indice_proceso_actual] = tiempo_global - proceso_actual.arribo as u64; // Tiempo de retorno inicial
                    indice_proceso_actual += 1;
                    hubo_cambio = true;
                } else {
                    // Si no se pudo asignar, registramos el evento y salimos del bucle
                    eventos.push(format!(
                        "En el tiempo global {}, la memoria estaba llena o era insuficiente para asignar el proceso {}. El proceso quedó esperando.",
                        tiempo_global,
                        proceso_actual.nombre
                    ));
                    hubo_cambio = true; // Registramos que hubo un intento de asignación
                    break; // Salimos del bucle para reintentar en el siguiente ciclo
                }
            }

            // ===========================
            // Parte 3: Avance del tiempo
            // ===========================
            if !hubo_cambio {
                eventos.push(format!(
                    "En el tiempo global {}, no se asignó ni liberó nada. (Memoria Ocupada: {} KB de {} total)",
                    tiempo_global,
                    calcular_memoria_ocupada(&vector_memoria),
                    configuracion.tamanio_memoria
                ));
            }

            // Decrementar tiempo de vida de los procesos en memoria
            decrementar_tiempo_vida(&mut vector_memoria);

            // Calcular particiones libres para el índice de fragmentación externa
            let particiones_libres_este_ciclo = vector_memoria.iter().filter(|p| p.esta_libre()).count() as u64;
            particiones_libres_totales += particiones_libres_este_ciclo;

            // Avanzar el tiempo global en una unidad
            tiempo_global += 1;
        }

        // ===========================
        // Registro de resultados finales
        // ===========================
        // Calcular tiempos de retorno
        let mut tiempos_totales_retorno = 0;
        for (i, proceso) in procesos.iter().enumerate() {
            let tiempo_retorno = tiempos_de_retorno[i] + proceso.duracion as u64;
            resultados.push(format!(
                "Tiempo de retorno del proceso {}: {} unidades de tiempo.",
                proceso.nombre,
                tiempo_retorno
            ));
            tiempos_totales_retorno += tiempo_retorno;
        }

        // Calcular tiempo medio de retorno
        let tiempo_medio_retorno = tiempos_totales_retorno as f64 / procesos.len() as f64;
        resultados.push(format!(
            "Tiempo medio de retorno: {:.2} unidades de tiempo.",
            tiempo_medio_retorno
        ));

        // Calcular índice de fragmentación externa
        let indice_fragmentacion_externa = particiones_libres_totales as f64 / tiempo_global as f64;
        resultados.push(format!(
            "Índice de fragmentación externa: {:.2}.",
            indice_fragmentacion_externa
        ));

        // Registrar tiempo total de simulación
        resultados.push(format!(
            "Tiempo total de la simulación: {} unidades de tiempo.",
            tiempo_global
        ));
    }
}

// ===========================
// Definición de funciones
// ===========================

/// Función para determinar si la tanda ha finalizado
/// La tanda ha finalizado si todos los procesos han sido asignados y la memoria está completamente libre.
fn fin_de_la_tanda(indice_proceso_actual: usize, total_procesos: usize, vector_memoria: &[Particion]) -> bool {
    indice_proceso_actual == total_procesos && vector_memoria.iter().all(|p| p.esta_libre())
}

/// Funcion para crear el vector del simulador con la configuracion seteada
/// Crea un vector de particiones, todas inicialmente libres, con el tamaño especificado.
fn crear_vector_memoria(tamanio_memoria: u32) -> Vec<Particion> {
    let mut vector_memoria = Vec::new();
    for _ in 0..tamanio_memoria {
        let particion = Particion::nueva(); // Crear una nueva partición de tamaño 1KB
        vector_memoria.push(particion); // Añadir la partición al vector de memoria
    }
    vector_memoria
}

/// Funcion para liberar memoria de los procesos que han finalizado
/// Libera la memoria de los procesos cuyo tiempo de vida ha llegado a cero.
fn liberar_memoria(
    vector_memoria: &mut Vec<Particion>,
    _configuracion: &Config,
    procesos_liberados: &mut Vec<String>,
) -> bool {
    let mut memoria_liberada = false;
    let mut procesos_a_liberar = Vec::new();

    // Identificar procesos que deben ser liberados
    for particion in vector_memoria.iter() {
        if let Some(tiempo_vida) = particion.tiempo_de_vida {
            if tiempo_vida == 0 {
                if let Some(ref nombre_proceso) = particion.id_proceso {
                    if !procesos_a_liberar.contains(nombre_proceso) {
                        procesos_a_liberar.push(nombre_proceso.clone());
                    }
                }
            }
        }
    }

    // Liberar particiones de los procesos identificados
    for particion in vector_memoria.iter_mut() {
        if let Some(ref nombre_proceso) = particion.id_proceso {
            if procesos_a_liberar.contains(nombre_proceso) {
                particion.liberar();
                memoria_liberada = true;
            }
        }
    }

    procesos_liberados.extend(procesos_a_liberar);
    memoria_liberada
}

/// Función para calcular la memoria ocupada
/// Devuelve la cantidad de memoria ocupada en KB
fn calcular_memoria_ocupada(vector_memoria: &[Particion]) -> u32 {
    vector_memoria.iter().filter(|p| !p.esta_libre()).count() as u32
}

//Esta función se encarga de reducir el tiempo de vida restante de los procesos en memoria en cada ciclo.
fn decrementar_tiempo_vida(vector_memoria: &mut Vec<Particion>) {
    for particion in vector_memoria.iter_mut() {
        if let Some(tiempo_vida) = particion.tiempo_de_vida {
            if tiempo_vida > 0 {
                particion.tiempo_de_vida = Some(tiempo_vida - 1);
            }
        }
    }
}