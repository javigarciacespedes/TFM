use std::error::Error;
use std::process::Command;
use std::time::{Duration, Instant};
use std::fs::File;
use csv::WriterBuilder;

fn crear_y_eliminar_contenedor() -> Result<Duration, Box<dyn Error>> {
    // Crear el contenedor
    let cmd_crear = Command::new("docker")
        .args(&["run", "-d", "nginx:latest"])
        .output()?;
    let tiempo_inicio = Instant::now(); // Registrar el tiempo de inicio
    let id_contenedor = String::from_utf8_lossy(&cmd_crear.stdout).trim().to_string();

    // Calcular el tiempo de creación del contenedor
    let transcurrido = Instant::now().duration_since(tiempo_inicio);

    // Eliminar el contenedor
    let _ = Command::new("docker")
        .args(&["container", "stop", &id_contenedor])
        .output()?;
    
    Ok(transcurrido)
}

fn main() -> Result<(), Box<dyn Error>> {
    let num_iteraciones = 100;
    let tiempo_actual = chrono::Local::now();
    let nombre_archivo_csv = format!("testContRust_{}.csv", tiempo_actual.format("%Y%m%d%H%M%S"));

    let archivo_csv = File::create(&nombre_archivo_csv)?;
    let mut writer = WriterBuilder::new().from_writer(archivo_csv);

    println!("Ejecutando pruebas de creación de contenedor...");

    // Escribir encabezado en el CSV
    writer.write_record(&["Iteración", "Tiempo Transcurrido (ns)"])?;

    for i in 0..num_iteraciones {
        let transcurrido = crear_y_eliminar_contenedor()?;
        println!("Iteración {}: {:?}", i + 1, transcurrido);

        writer.write_record(&[format!("{}", i + 1), format!("{}", transcurrido.as_nanos())])?;
    }

    println!("Pruebas completadas. Resultados guardados en {}", nombre_archivo_csv);

    Ok(())
}
