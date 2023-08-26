use std::fs::File;
use std::io::Write;
use std::time::Instant;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let get_url = "https://jsonplaceholder.typicode.com/posts/1";
    let post_url = "https://jsonplaceholder.typicode.com/posts";

    let mut file = File::create("resultados.csv")?;
    writeln!(file, "Iteración,Tipo,Tiempo")?;

    let client = reqwest::Client::new();

    for i in 0..10 {
        //GET
        let get_start_time = Instant::now();
        let _ = client.get(get_url).send().await?;
        let get_end_time = Instant::now();

        let get_duration = get_end_time.duration_since(get_start_time);
        writeln!(file, "{},{},{}", i + 1, "GET", format!("{:?}", get_duration))?;
        println!("Iteración {}: Operación GET registrada - Tiempo = {:?}", i + 1, get_duration);

        //POST
        let post_start_time = Instant::now();
        let json_data = serde_json::json!({
            "title": "foo",
            "body": "bar",
            "userId": 1
        });
        let _ = client.post(post_url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&json_data)?)
            .send()
            .await?;
        let post_end_time = Instant::now();

        let post_duration = post_end_time.duration_since(post_start_time);
        writeln!(file, "{},{},{}", i + 1, "POST", format!("{:?}", post_duration))?;
        println!("Iteración {}: Operación POST registrada - Tiempo = {:?}", i + 1, post_duration);
    }

    println!("Pruebas completadas. Resultados almacenados en resultados.csv");

    Ok(())
}
