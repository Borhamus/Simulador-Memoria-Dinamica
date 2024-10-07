use crate::configuracion::ConfigurarSimulador;
use crate::manipular_archivo::{guardar_procesos_y_configuracion, agregar_eventos_y_resultados, mostrar_archivo_completo};
use crate::generador::Generador;
use crate::simulacion::Simulador;

pub struct NuevaSimulacion {
    pub archivo_cargado: Option<String>,
    pub contador_simulaciones: usize,
}

impl NuevaSimulacion {
    pub fn new() -> NuevaSimulacion {
        NuevaSimulacion {
            archivo_cargado: None,
            contador_simulaciones: 0,
        }
    }

    // Función principal para iniciar la simulación
    pub fn iniciar_nueva_tanda(&mut self) {
        // Generar los procesos utilizando el nuevo archivo generador.rs
        let procesos = Generador::generar_procesos();

        // Llamamos a la función en configuracion.rs para obtener la configuración
        let configuracion = ConfigurarSimulador::nueva_configuracion();

        let nombre_archivo = self.generar_nombre_archivo(procesos.len(), &configuracion.estrategia);
        self.archivo_cargado = Some(nombre_archivo.clone());

        // Guardamos los procesos y la configuración en el archivo
        guardar_procesos_y_configuracion(&nombre_archivo, &procesos, &configuracion);

        // Ejecutar la simulación
        let memoria = Memoria::new(vec![]); // Crear particiones
        let mut simulador = Simulador::new(memoria, procesos, configuracion.estrategia);

        let (eventos, resultados) = simulador.ejecutar_simulacion();

        // Guardar los eventos y resultados en el archivo
        agregar_eventos_y_resultados(&nombre_archivo, eventos, resultados);

        // Mostrar el archivo completo por pantalla
        mostrar_archivo_completo(&nombre_archivo);
    }

    fn generar_nombre_archivo(&mut self, num_procesos: usize, estrategia: &str) -> String {
        // Lógica de generar nombre de archivo...
    }
}
