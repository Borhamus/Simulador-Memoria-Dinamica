### README.md

# **MemSim3000** - Simulador de Memoria

## **Descripción General**

**MemSim3000** es un simulador educativo de administración de memoria que implementa diversas estrategias de asignación de particiones dinámicas. Este proyecto simula un sistema multiprogramado y monoprocesador, donde se gestionan los trabajos (procesos) que llegan y se asignan a la memoria principal mediante diferentes estrategias. El simulador permite configurar la cantidad de procesos, la estrategia de asignación y la configuración de la memoria física.

El simulador es interactivo y está diseñado para ser utilizado desde la terminal. Se puede usar para realizar experimentos y entender cómo funcionan las estrategias de asignación de memoria.

### **Objetivo del Proyecto**

El objetivo principal es permitir la simulación de diversas estrategias de administración de memoria contigua en particiones dinámicas, proporcionando métricas sobre el rendimiento de dichas estrategias. Esto será útil para estudiar el comportamiento de la memoria física en sistemas operativos multiprogramados.

---

## **Características del Sistema**

### **Entrada de Procesos**

El programa simula la llegada de trabajos mediante archivos de texto que describen una lista de procesos. Cada proceso tiene la siguiente estructura:

- **Nombre**: Identificación del proceso.
- **Instante de Arribo**: Tiempo en que el proceso llega al sistema.
- **Duración**: Tiempo total que el proceso debe permanecer en memoria.
- **Memoria Requerida**: Tamaño de memoria que el proceso necesita para ejecutarse.

El usuario puede seleccionar o crear un archivo de procesos donde se define la lista de trabajos a simular.

### **Configuración del Sistema**

Al iniciar una simulación, el usuario puede configurar varios parámetros del sistema:

- **Tamaño de la Memoria Física**: Memoria total disponible (excluyendo la memoria del sistema operativo).
- **Estrategia de Asignación**: El simulador soporta varias estrategias:
  - **First Fit**
  - **Best Fit**
  - **Next Fit**
  - **Worst Fit**
- **Tiempo de Selección de Partición**: Tiempo que toma seleccionar la partición donde se asignará el proceso.
- **Tiempo de Carga Promedio**: Tiempo que tarda en cargar el proceso desde la memoria secundaria a la memoria principal.
- **Tiempo de Liberación de Partición**: Tiempo que tarda en liberar la partición cuando un proceso termina.

### **Simulación y Salidas**

Durante la simulación, el sistema realiza las siguientes acciones:

1. Selección de particiones según la estrategia configurada.
2. Carga y liberación de procesos en la memoria.
3. Registro de eventos en un archivo de texto con detalles de las acciones realizadas, como:
   - Selección de una partición para un proceso.
   - Carga del proceso en memoria.
   - Liberación de la partición al finalizar el proceso.

### **Indicadores**

Al finalizar la simulación, se generan varios indicadores, tanto por proceso como para toda la tanda de procesos:

- **Tiempo de Retorno** (Por proceso): Tiempo total desde el arribo del proceso hasta su finalización.
- **Tiempo de Retorno Medio** (Para la tanda completa): Promedio de los tiempos de retorno de todos los procesos.
- **Índice de Fragmentación Externa**: Se mide el porcentaje de memoria fragmentada que no puede ser utilizada por procesos activos.

---

## **Ejecución del Simulador**

### **Requisitos**

- Rust 1.56+ instalado. Si no lo tienes, puedes instalarlo desde [Rust Official](https://www.rust-lang.org/tools/install).
- Windows, Linux o macOS con terminal disponible.

### **Cómo Ejecutar**

1. Clona el repositorio del proyecto:
    ```bash
    git clone https://github.com/tu-repositorio/memsim3000.git
    cd memsim3000
    ```

2. Compila el proyecto:
    ```bash
    cargo build
    ```

3. Ejecuta el simulador:
    ```bash
    cargo run
    ```

Al iniciar el programa, aparecerá una pantalla de bienvenida en estilo retro con un menú que te permitirá:

1. **Configurar y comenzar una simulación**.
2. **Ver simulaciones anteriores** (funcionalidad pendiente).
3. **Salir** del simulador.

### **Pasos para Simular**

1. **Configura la Simulación**: Ingresa la cantidad de memoria disponible, la estrategia de asignación y otros parámetros.
2. **Selecciona un archivo de procesos**: Puedes seleccionar un archivo existente o crear uno nuevo.
3. **Corre la simulación**: El simulador cargará los procesos y gestionará su asignación de memoria de acuerdo con la estrategia seleccionada.
4. **Revisa los resultados**: Al finalizar, se mostrará un archivo de texto con los resultados y eventos de la simulación.

---

## **Estrategias de Asignación**

El simulador soporta las siguientes estrategias de asignación de particiones:

1. **First Fit**: Asigna la primera partición que sea lo suficientemente grande.
2. **Best Fit**: Asigna la partición más pequeña que pueda contener el proceso.
3. **Next Fit**: Similar a First Fit, pero comienza la búsqueda desde la última posición de asignación.
4. **Worst Fit**: Asigna la partición más grande disponible.

---

## **Estructura del Proyecto**

- **`main.rs`**: Archivo principal que inicia el simulador.
- **`menu.rs`**: Gestiona la interacción del usuario a través del menú y llama a las funciones correspondientes.
- **`confsim.rs`**: Captura la configuración inicial de la simulación, como la estrategia y los parámetros de memoria.
- **`estrategias.rs`**: Define las estrategias de asignación de memoria.
- **`proceso.rs`**: Define la estructura del proceso y su comportamiento.
- **`particion.rs`**: Define la estructura y comportamiento de las particiones de memoria.
- **`generador.rs`**: Funciones para generar archivos de procesos y trabajar con los mismos.
- **`io_utils.rs`**: Utilidades para la lectura y escritura de archivos de texto.

---

## **Por Hacer / Funcionalidades Pendientes**

- **Implementar la función de ver simulaciones anteriores.**
- **Ajustar el sistema de informes y hacer más robusto el análisis de resultados.**
- **Optimizar la visualización de métricas y estadísticas al finalizar la simulación.**
- **Añadir más opciones de configuración para personalizar aún más las simulaciones.**
- **Probar con diferentes tamaños de procesos y memorias para optimizar el rendimiento.**

---

## **Créditos**

- **Autor**: Franco Joaquín Gómez - Alias: Borhamus Fynolt
- **Colaboradores**: (Espacio para añadir colaboradores si hay otros que han trabajado en el proyecto)

---

## **Licencia**

Este proyecto está licenciado bajo los términos de la **MIT License**. Puedes consultar más detalles en el archivo [LICENSE](./LICENSE).

---

¡Gracias por explorar el Simulador de Memoria MemSim3000! Si tienes preguntas o sugerencias, no dudes en contactarme.
