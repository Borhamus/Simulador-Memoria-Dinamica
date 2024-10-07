use crate::particion::{Particion, EstadoParticion};
use crate::proceso::Proceso;
use crate::config::Config;

pub struct Simulador;

impl Simulador {
    pub fn ejecutar_simulacion(
        _procesos: &[Proceso],
        _configuracion: &Config,
        eventos: &mut Vec<String>,   // Usa directamente las referencias mutables
        resultados: &mut Vec<String> // Usa directamente las referencias mutables
    ) {
        
        //Defino cosas necesarias:
        let mut tiempo_global: u32 = 0;
        let mut mem_ocupada: bool = false;
        let mut indice_proceso_actual: usize = 0;
        let mut proceso_actual: &Proceso = &_procesos[indice_proceso_actual];
        let mut fin_de_la_tanda: bool = indice_proceso_actual==_procesos.len();

        

        //Funciones disparadas:
        //Funcion para asignar proceso a particiones:
        fn asignar(proceso: &Proceso,vector_memoria: &mut Vec<Particion>, tiempo_global: &mut u32, configuracion: &Config ) -> bool {
            // Tu lógica aquí
            
            
            
            
            
            
            true
        }
        
        //Funcion para "esperar" generica:
        fn delay_tiempo(tiempo_global: &mut u32, valor: u32) {
            *tiempo_global += valor;
        }

        //Funcion para crear el vector del simulador con la configuracion seteada:
        fn crear_vector_memoria(tamanio_memoria: u32) -> Vec<Particion> {
            // Inicializamos el vector de particiones
            let mut vector_memoria = Vec::new();
            
            // Creamos cada partición de tamaño 1KB y la insertamos en el vector
            for _ in 0..tamanio_memoria {
                let particion = Particion::nueva();
                vector_memoria.push(particion);
            }
            
            vector_memoria
        }

        fn liberar_memoria(vector_memoria: &mut Vec<Particion>, tiempo_global: u32, configuracion: &Config) -> bool {
            let mut i = 0;
        
            // Iterar sobre las particiones de la memoria
            while i < vector_memoria.len() {
                if (EstadoParticion::Ocupada == vector_memoria[i].estado) {
                    // Verificar si el tiempo de vida ha llegado a 0
                    if vector_memoria[i].tiempo_de_vida <= 0 {
                        // Obtener el nombre del proceso que se va a eliminar
                        let nombre_del_proceso_a_eliminar = vector_memoria[i].id_proceso;
                        
                        // Liberar todas las particiones ocupadas por el mismo proceso
                        for particion in vector_memoria.iter_mut() {
                            if particion.id_proceso == nombre_del_proceso_a_eliminar {
                                particion.estado = EstadoParticion::Libre;
                            } else {
                                break;
                            }
                        }
                        //Si el tiempo de vida no ha llegado a 0:
                    } else {
                        // Decrementar el tiempo de vida si el proceso sigue activo
                        *tiempo_vida = tiempo_vida.saturating_sub(configuracion.tiempo_liberacion);
                    }
                } else {
                    i += 1;
                }
            }
        
            true
        }
        

        // Crear el vector de memoria basado en la configuración
        let mut vector_memoria = crear_vector_memoria(_configuracion.tamanio_memoria);


        //Logica de la simulacion:

        while (!fin_de_la_tanda) || (mem_ocupada) {

            if (indice_proceso_actual < _procesos.len()){

                //cargo el proceso del vector en esta variable.
                let proceso_actual = &_procesos[indice_proceso_actual];

                // Intentar asignar el proceso actual
                let mut proceso_asignado: bool = asignar(proceso_actual, &mut vector_memoria, &mut tiempo_global, _configuracion);

                if (proceso_asignado){
                    indice_proceso_actual += 1;
                } else {
                    eventos.push(format!(
                        "En el tiempo global {}, la memoria estaba llena o era insuficiente para asignar el proceso {}. El proceso quedó esperando.",
                        tiempo_global, proceso_actual.nombre
                    ));
                }

                let mut mem_ocupada: bool = liberar_memoria(&mut vector_memoria, &mut tiempo_global, _configuracion);

                if (mem_ocupada){

                }


            }         
        }
    }
}

