use crate::memoria::Memoria;
use crate::proceso::Proceso;
use crate::particion::Particion;
use crate::estrategias::EstrategiaAsignacion;
use std::io::{self, Write};

pub struct Simulador {
    pub memoria: Memoria,
    pub procesos: Vec<Proceso>,
    pub estrategia: EstrategiaAsignacion,
}

impl Simulador {
    pub fn new(memoria: Memoria, procesos: Vec<Proceso>, estrategia: EstrategiaAsignacion) -> Simulador {
        Simulador {
            memoria,
            procesos,
            estrategia,
        }
    }

    // Función principal para ejecutar la simulación y devolver el contenido en formato tabla
    pub fn ejecutar_simulacion(&mut self) -> (String, String) {
        let mut tiempo_actual = 0;
        let mut procesos_pendientes = self.procesos.clone();
        let mut eventos = String::new(); // Para almacenar los eventos
        let mut resultados = String::new(); // Para almacenar los resultados finales

        eventos.push_str("Registro de eventos:\n");
        eventos.push_str(format!("{:<10} {:<15} {:<15} {:<10}\n", "Tiempo", "Proceso", "Partición", "Evento").as_str());
        eventos.push_str("-----------------------------------------------\n");

        while !procesos_pendientes.is_empty() {
            // Ordenamos los procesos por su instante de arribo
            procesos_pendientes.sort_by_key(|p| p.arribo);

            // Revisamos cada proceso para intentar asignarlo a la memoria
            for proceso in procesos_pendientes.iter_mut() {
                if proceso.arribo <= tiempo_actual {
                    match self.memoria.asignar_proceso(proceso, self.estrategia.clone()) {
                        Some(particion) => {
                            eventos.push_str(
                                format!("{:<10} {:<15} {:<15} {:<10}\n", tiempo_actual, proceso.nombre, particion.id, "Asignado").as_str()
                            );

                            proceso.set_tiempo_inicio(tiempo_actual);
                            tiempo_actual += proceso.duracion;

                            eventos.push_str(
                                format!("{:<10} {:<15} {:<15} {:<10}\n", tiempo_actual, proceso.nombre, particion.id, "Finalizado").as_str()
                            );

                            self.memoria.liberar_particion(particion.id);
                            proceso.set_tiempo_fin(tiempo_actual);
                        }
                        None => {
                            eventos.push_str(
                                format!("{:<10} {:<15} {:<15} {:<10}\n", tiempo_actual, proceso.nombre, "-", "No Asignado").as_str()
                            );
                        }
                    }
                }
            }

            procesos_pendientes.retain(|p| p.tiempo_fin.is_none());
            tiempo_actual += 1;
        }

        // Finaliza la simulación, calcular e imprimir indicadores
        resultados.push_str(self.calcular_indicadores().as_str());

        (eventos, resultados)
    }

    // Calcular indicadores y devolver como string en formato tabla
    fn calcular_indicadores(&self) -> String {
        let mut resultados = String::new();
        let mut tiempo_total_retorno = 0;
        let mut procesos_finalizados = 0;

        resultados.push_str("Resultados:\n");
        resultados.push_str(format!("{:<15} {:<20}\n", "Proceso", "Tiempo de Retorno").as_str());
        resultados.push_str("---------------------------------\n");

        for proceso in &self.procesos {
            if let (Some(tiempo_inicio), Some(tiempo_fin)) = (proceso.get_tiempo_inicio(), proceso.get_tiempo_fin()) {
                let tiempo_retorno = tiempo_fin - proceso.arribo;
                tiempo_total_retorno += tiempo_retorno;
                procesos_finalizados += 1;

                resultados.push_str(
                    format!("{:<15} {:<20}\n", proceso.nombre, tiempo_retorno).as_str()
                );
            }
        }

        if procesos_finalizados > 0 {
            let tiempo_medio_retorno = tiempo_total_retorno as f64 / procesos_finalizados as f64;
            resultados.push_str(format!("Tiempo Medio de Retorno: {:.2}\n", tiempo_medio_retorno).as_str());
        }

        let fragmentacion_externa = self.memoria.calcular_fragmentacion_externa();
        resultados.push_str(format!("Índice de Fragmentación Externa: {:.2}%\n", fragmentacion_externa * 100.0).as_str());

        resultados
    }
}
