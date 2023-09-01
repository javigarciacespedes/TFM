use std::fs::File;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url_get = "https://jsonplaceholder.typicode.com/posts/1";
    let url_post = "https://jsonplaceholder.typicode.com/posts";

    let mut archivo_csv = File::create("resultados.csv").expect("Error al crear el archivo CSV");

    writeln!(archivo_csv, "Iteración,Tipo,Tiempo (s)").expect("Error al escribir encabezado en el archivo");

    for i in 0..5 {
        // GET
        let mut duracion_total_get = Duration::new(0, 0);
        for _ in 0..10 {
            let inicio_get = Instant::now();
            reqwest::get(url_get)
                .await?
                .error_for_status()?
                .bytes()
                .await?;
            let fin_get = Instant::now();
            duracion_total_get += fin_get - inicio_get;
        }
        let duracion_total_get_s = duracion_total_get.as_secs_f32();
        writeln!(archivo_csv, "{},{}, {:.4}", i + 1, "GET", duracion_total_get_s).expect("Error al escribir resultado en el archivo");

        println!("GET - Iteración {}: Tiempo total = {:.4} segundos", i + 1, duracion_total_get_s);

        // POST
        let mut duracion_total_post = Duration::new(0, 0);
        for _ in 0..10 {
            let inicio_post = Instant::now();
            reqwest::Client::new()
                .post(url_post)
                .header("Content-Type", "application/json")
                .body(r#"{"title": "foo", "body": "bar", "userId": 1}"#)
                .send()
                .await?
                .error_for_status()?
                .bytes()
                .await?;
            let fin_post = Instant::now();
            duracion_total_post += fin_post - inicio_post;
        }
        let duracion_total_post_s = duracion_total_post.as_secs_f32();
        writeln!(archivo_csv, "{},{}, {:.4}", i + 1, "POST", duracion_total_post_s).expect("Error al escribir resultado en el archivo");

        println!("POST - Iteración {}: Tiempo total = {:.4} segundos", i + 1, duracion_total_post_s);
    }

    println!("Pruebas completadas. Resultados almacenados en resultados.csv");
    Ok(())
}
