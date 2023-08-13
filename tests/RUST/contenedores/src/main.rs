use std::fs::File;
use std::io::{Write, Result};
use std::process::Command;
use std::time::Instant;

fn create_and_delete_container() -> Result<()> {
    let mut log_file = File::create("testContRust.log")?;

    for i in 0..10 {
        let start_time = Instant::now();

        let output = Command::new("docker")
            .args(&["run", "-d", "nginx:latest"])
            .output();

        match output {
            Ok(output) => {
                let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let elapsed = start_time.elapsed();
                let duration_ns = elapsed.as_nanos();
                println!("Iteración {} - Tiempo de creación: {}ns", i + 1, duration_ns);
                writeln!(log_file, "Iteración {} - Tiempo de creación: {}ns", i + 1, duration_ns)?;

                Command::new("docker")
                    .args(&["container", "stop", &container_id])
                    .spawn()?
                    .wait()?;
            }
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error al crear el contenedor: {}", err),
                ));
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = create_and_delete_container() {
        eprintln!("Error: {}", err);
    }
}
