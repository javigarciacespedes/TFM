use std::error::Error;
use std::fs::File;
use std::time::Instant;
use csv::WriterBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Ejecutando pruebas de rendimiento...");

    let nombre_archivo_csv = "testMathRust.csv";
    let archivo_csv = File::create(nombre_archivo_csv)?;
    let mut writer = WriterBuilder::new().from_writer(archivo_csv);

    writer.write_record(&["Operación", "Repetición", "Tiempo (ns)"])?;

    // Iteraciones de calentamiento
    calentar(sumar);
    calentar(restar);
    calentar(raiz_cuadrada);
    calentar(multiplicar);
    calentar(dividir);
    calentar(seno);
    calentar(coseno);

    // Mediciones reales
    prueba_rendimiento(&mut writer, "Suma", sumar)?;
    prueba_rendimiento(&mut writer, "Resta", restar)?;
    prueba_rendimiento(&mut writer, "RaizCuadrada", raiz_cuadrada)?;
    prueba_rendimiento(&mut writer, "Multiplicacion", multiplicar)?;
    prueba_rendimiento(&mut writer, "Division", dividir)?;
    prueba_rendimiento(&mut writer, "Seno", seno)?;
    prueba_rendimiento(&mut writer, "Coseno", coseno)?;

    println!("Pruebas completadas. Resultados guardados en {}", nombre_archivo_csv);

    Ok(())
}

fn calentar(funcion_operacion: fn()) {
    let num_repeticiones = 20;
    for _ in 0..num_repeticiones {
        funcion_operacion();
    }
}

fn prueba_rendimiento(writer: &mut csv::Writer<File>, operacion: &str, funcion_operacion: fn()) -> Result<(), Box<dyn Error>> {
    let num_repeticiones = 100;
    let mut duraciones = Vec::with_capacity(num_repeticiones);

    for _ in 0..num_repeticiones {
        let tiempo_inicio = Instant::now();
        funcion_operacion();
        let elapsed = Instant::now().duration_since(tiempo_inicio);

        println!("{} - Repetición {}: {:?}", operacion, duraciones.len() + 1, elapsed);
        writer.write_record(&[operacion, &(duraciones.len() + 1).to_string(), &elapsed.as_nanos().to_string()])?;

        duraciones.push(elapsed);
    }

    Ok(())
}

fn sumar() {
    let _ = 2 + 3;
}

fn restar() {
    let _ = 5 - 3;
}

fn raiz_cuadrada() {
    let _ = 16.0f64.sqrt();
}

fn multiplicar() {
    let _ = 4 * 6;
}

fn dividir() {
    let _ = 10.0 / 2.0;
}

fn seno() {
    let _ = f64::sin(0.5);
}

fn coseno() {
    let _ = f64::cos(std::f64::consts::FRAC_PI_2);
}
