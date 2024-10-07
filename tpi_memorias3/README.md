

### RECORDA QUE ESTA INCOMPLETO ESTE DOCUMENTO!

# Simulador de Procesos en Rust

Este proyecto es un simulador de procesos desarrollado en Rust, que permite la ejecución de distintas estrategias de planificación y simulación de procesos en un entorno configurado mediante archivos de entrada y salida. 

## Características

- **Simulación de procesos:** El sistema simula múltiples procesos con diferentes estrategias de planificación.
- **Estrategias de planificación:** Puedes configurar diferentes algoritmos de planificación como FIFO, Round-Robin, entre otros.
- **Interfaz de usuario sencilla:** Menú interactivo para cargar configuraciones y ejecutar simulaciones.
- **Modularidad:** El código está organizado en múltiples módulos que permiten la separación lógica de funcionalidades.

## Estructura del Proyecto

El proyecto está organizado de la siguiente manera:

- `main.rs`: Archivo principal que inicia la ejecución del simulador.
- `usuario.rs`: Módulo que maneja los datos y la configuración del usuario.
- `confsim.rs`: Módulo que maneja la configuración de la simulación.
- `estrategias.rs`: Contiene las diferentes estrategias de planificación de procesos.
- `generador.rs`: Módulo encargado de generar procesos simulados.
- `io_utils.rs`: Utilidades para la entrada y salida de archivos.
- `menu.rs`: Maneja el menú interactivo para la ejecución del simulador.
- `particion.rs`: Módulo encargado de la gestión de particiones de memoria.
- `proceso.rs`: Define las estructuras de los procesos que serán simulados.
- `simulacion.rs`: Módulo principal donde se ejecuta la lógica de la simulación.

## Instalación

Para utilizar este simulador, primero necesitas tener instalado [Rust](https://www.rust-lang.org/).

1. Clona este repositorio:

   ```bash
   git clone https://github.com/tu_usuario/simulador_procesos_rust.git
   ```

2. Accede al directorio del proyecto:

   ```bash
   cd simulador_procesos_rust
   ```

3. Compila el proyecto:

   ```bash
   cargo build
   ```

4. Ejecuta el simulador:

   ```bash
   cargo run
   ```

## Uso

Una vez que el simulador está corriendo, se presenta un menú interactivo donde puedes:

1. Cargar archivos de configuración para la simulación.
2. Seleccionar la estrategia de planificación.
3. Ejecutar la simulación y visualizar los resultados.

## Archivos de Configuración

Los archivos de configuración permiten definir los parámetros de los procesos y las estrategias de planificación. Asegúrate de que los archivos de entrada sigan el formato adecuado para ser procesados correctamente por el simulador.

## Contribuciones

Las contribuciones son bienvenidas. Si encuentras algún problema o tienes ideas para mejorar el simulador, no dudes en abrir un issue o enviar un pull request.

## Licencia

Este proyecto está licenciado bajo la MIT License - ver el archivo [LICENSE](LICENSE) para más detalles.

---