use std::fs::File;
use std::io::self;
use std::time::Instant;
use csv::Writer;

fn main() -> io::Result<()> {
    println!("Ejecutando pruebas de rendimiento...");

    let nombre_archivo_csv = "testOperacionesRust.csv";
    let archivo_csv = File::create(nombre_archivo_csv)?;

    let mut writer = Writer::from_writer(archivo_csv);

    writer.write_record(&["Operación", "Repetición", "Tiempo (0.0001s)"])?;

    prueba_rendimiento(&mut writer, "Suma", sumar)?;
    prueba_rendimiento(&mut writer, "Resta", restar)?;
    prueba_rendimiento(&mut writer, "RaizCuadrada", raiz_cuadrada)?;
    prueba_rendimiento(&mut writer, "Multiplicacion", multiplicar)?;
    prueba_rendimiento(&mut writer, "Division", dividir)?;
    prueba_rendimiento(&mut writer, "Seno", seno)?;
    prueba_rendimiento(&mut writer, "Coseno", coseno)?;

    println!("Pruebas terminadas");
    
    Ok(())
}

fn prueba_rendimiento<F>(writer: &mut Writer<File>, operacion: &str, funcion_operacion: F) -> io::Result<()>
where
    F: Fn(),
{
    let num_repeticiones = 1000;
    let num_iteraciones = 100000000;
    let mut duraciones = Vec::with_capacity(num_repeticiones);

    for i in 0..num_repeticiones {
        let tiempo_inicio = Instant::now();
        for _ in 0..num_iteraciones {
            funcion_operacion();
        }
        let elapsed = tiempo_inicio.elapsed().as_secs_f64() * 10000.0; // Convertir a décimas de segundo con cuatro decimales
        duraciones.push(elapsed);

        println!("{} - Repetición {}: {:.4} (0.0001s)", operacion, i + 1, elapsed);
        writer.write_record(&[operacion, &format!("{}", i + 1), &format!("{:.4}", elapsed)])?;
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
    let _ = f64::sqrt(16.0);
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
    let _ = f64::cos(0.5);
}
