use std::fs::File;
use std::io::Write;
use std::time::{Instant};


fn main() {
    let mut archivo_log = File::create("testMathRust.log").expect("Error al crear el archivo de registro");
    
    archivo_log.write_all(b"Ejecutando pruebas de rendimiento...\n").expect("Error al escribir en el archivo de registro");
    
    prueba_rendimiento("Suma", &mut archivo_log, sumar);
    prueba_rendimiento("Resta", &mut archivo_log, restar);
    prueba_rendimiento("RaízCuadrada", &mut archivo_log, raiz_cuadrada);
    prueba_rendimiento("Multiplicación", &mut archivo_log, multiplicar);
    prueba_rendimiento("División", &mut archivo_log, dividir);
    prueba_rendimiento("Seno", &mut archivo_log, seno);
    prueba_rendimiento("Coseno", &mut archivo_log, coseno);
}

fn prueba_rendimiento<F>(operacion: &str, archivo_log: &mut File, funcion_operacion: F)
where
    F: Fn() -> (),
{
    let num_repeticiones = 100;
    let mut duraciones = Vec::with_capacity(num_repeticiones);

    for i in 0..num_repeticiones {
        let tiempo_inicio = Instant::now();
        funcion_operacion();
        let transcurrido = tiempo_inicio.elapsed();
        duraciones.push(transcurrido);

        let duracion_ns = transcurrido.as_nanos();
        writeln!(archivo_log, "{} - Repetición {}: {}ns", operacion, i + 1, duracion_ns).expect("Error al escribir en el archivo de registro");
    }
}

fn sumar() {
    let _resultado = 2 + 3;
}

fn restar() {
    let _resultado = 5 - 3;
}

fn raiz_cuadrada() {
    let _resultado = 16f64.sqrt();
}

fn multiplicar() {
    let _resultado = 4 * 6;
}

fn dividir() {
    let _resultado = 10.0 / 2.0;
}

fn seno() {
    let _resultado = 0.5_f64.sin();
}

fn coseno() {
    let _resultado = 0.5_f64.cos();
}
