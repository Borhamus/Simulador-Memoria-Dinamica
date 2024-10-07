//simulacion.rs

use crate::memoria::Memoria;
use crate::estrategias::EstrategiaAsignacion;
use crate::manipulaciondearchivos;
use crate::proceso::Proceso;
use std::fs::File;
use std::io::Write;


#[derive(Debug)]
pub enum TipoEvento {
    LlegadaProceso,
    SeleccionParticion,
    CargaProceso,
    FinalizacionProceso,
    LiberacionParticion,
}

#[derive(Debug)]
pub struct Evento {
    tiempo: u32,
    tipo: TipoEvento,
    descripcion: String,
}


pub struct Simulacion {
    memoria: Memoria,
    estrategia: EstrategiaAsignacion,
    tiempo_seleccion: u32,
    tiempo_carga: u32,
    tiempo_liberacion: u32,
    procesos: Vec<Proceso>,
    tiempo_actual: u32,
    eventos: Vec<Evento>,
}

impl Simulacion {
    pub fn new(
        nombre_archivo: &str,
        tamanio_memoria: u32,
        estrategia: EstrategiaAsignacion,
        tiempo_seleccion: u32,
        tiempo_carga: u32,
        tiempo_liberacion: u32,
    ) -> Simulacion {
        let procesos = manipulaciondearchivos::cargar_procesos_desde_archivo(nombre_archivo);
        Simulacion {
            memoria: Memoria::new(tamanio_memoria),
            estrategia,
            tiempo_seleccion,
            tiempo_carga,
            tiempo_liberacion,
            procesos,
            tiempo_actual: 0,
            eventos: Vec::new(),
        }
    }

    pub fn ejecutar(&mut self) {
        println!("Iniciando la simulación...");

        let mut procesos_pendientes = self.procesos.clone();
        let mut procesos_en_memoria = Vec::new();
        let mut procesos_finalizados = Vec::new();

        let mut archivo_salida = File::create("file/simulaciones/simulacion.txt")
            .expect("No se pudo crear el archivo de simulación");

        while !procesos_pendientes.is_empty() || !procesos_en_memoria.is_empty() {
            self.tiempo_actual += 1;

            // Procesar llegada de procesos
            let procesos_a_llegar: Vec<Proceso> = procesos_pendientes
                .iter()
                .filter(|p| p.instante_arribo == self.tiempo_actual)
                .cloned()
                .collect();

            for proceso in procesos_a_llegar {
                let evento = Evento {
                    tiempo: self.tiempo_actual,
                    tipo: TipoEvento::LlegadaProceso,
                    descripcion: format!("Proceso {} ha llegado", proceso.nombre),
                };
                self.eventos.push(evento);
                writeln!(archivo_salida, "{:?}", self.eventos.last().unwrap()).unwrap();

                let particion_id = self.memoria.asignar_proceso(&proceso, self.estrategia);
                if let Some(id) = particion_id {
                    let evento_seleccion = Evento {
                        tiempo: self.tiempo_actual + self.tiempo_seleccion,
                        tipo: TipoEvento::SeleccionParticion,
                        descripcion: format!("Proceso {} asignado a partición {}", proceso.nombre, id),
                    };
                    self.eventos.push(evento_seleccion);
                    writeln!(archivo_salida, "{:?}", self.eventos.last().unwrap()).unwrap();

                    let evento_carga = Evento {
                        tiempo: self.tiempo_actual + self.tiempo_seleccion + self.tiempo_carga,
                        tipo: TipoEvento::CargaProceso,
                        descripcion: format!("Proceso {} cargado en memoria", proceso.nombre),
                    };
                    self.eventos.push(evento_carga);
                    writeln!(archivo_salida, "{:?}", self.eventos.last().unwrap()).unwrap();

                    let mut proceso_en_memoria = proceso.clone();
                    proceso_en_memoria.tiempo_inicio = self.tiempo_actual + self.tiempo_seleccion + self.tiempo_carga;
                    proceso_en_memoria.tiempo_fin = proceso_en_memoria.tiempo_inicio + proceso_en_memoria.duracion_total;
                    procesos_en_memoria.push(proceso_en_memoria);
                } else {
                    // No se pudo asignar, esperar o manejar cola de espera
                }
                procesos_pendientes.retain(|p| p.nombre != proceso.nombre);
            }

            // Procesar finalización de procesos
            let procesos_a_finalizar: Vec<Proceso> = procesos_en_memoria
                .iter()
                .filter(|p| p.tiempo_fin == self.tiempo_actual)
                .cloned()
                .collect();

            for proceso in procesos_a_finalizar {
                self.memoria.liberar_particion(&proceso);
                let evento_finalizacion = Evento {
                    tiempo: self.tiempo_actual,
                    tipo: TipoEvento::FinalizacionProceso,
                    descripcion: format!("Proceso {} ha finalizado", proceso.nombre),
                };
                self.eventos.push(evento_finalizacion);
                writeln!(archivo_salida, "{:?}", self.eventos.last().unwrap()).unwrap();

                let evento_liberacion = Evento {
                    tiempo: self.tiempo_actual + self.tiempo_liberacion,
                    tipo: TipoEvento::LiberacionParticion,
                    descripcion: format!("Partición liberada para proceso {}", proceso.nombre),
                };
                self.eventos.push(evento_liberacion);
                writeln!(archivo_salida, "{:?}", self.eventos.last().unwrap()).unwrap();

                procesos_finalizados.push(proceso.clone());
                procesos_en_memoria.retain(|p| p.nombre != proceso.nombre);
            }

            // Mostrar eventos ocurridos en este tiempo
            for evento in self.eventos.iter().filter(|e| e.tiempo == self.tiempo_actual) {
                println!("Tiempo {}: {}", evento.tiempo, evento.descripcion);
            }
        }

        self.calcular_indicadores(&procesos_finalizados);
    }

    fn calcular_indicadores(&self, procesos_finalizados: &Vec<Proceso>) {
        let mut tiempos_retorno = Vec::new();
        for proceso in procesos_finalizados {
            let tiempo_retorno = proceso.tiempo_fin - proceso.instante_arribo;
            tiempos_retorno.push(tiempo_retorno);
            println!("Proceso {}: Tiempo de retorno: {}", proceso.nombre, tiempo_retorno);
        }

        let suma_tiempos: u32 = tiempos_retorno.iter().sum();
        let tiempo_retorno_medio = suma_tiempos as f32 / tiempos_retorno.len() as f32;
        println!("Tiempo de retorno medio para la tanda completa: {}", tiempo_retorno_medio);

        let indice_fragmentacion = self.memoria.calcular_fragmentacion_externa();
        println!("Índice de fragmentación externa: {}", indice_fragmentacion);
    }
}