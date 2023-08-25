use std::error::Error;
use std::fs::File;
use std::time::Instant;
use csv::Writer;
use reqwest;
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let get_url = "https://jsonplaceholder.typicode.com/posts/1";
    let post_url = "https://jsonplaceholder.typicode.com/posts";

    let file = File::create("resultados.csv")?;
    let mut writer = Writer::from_writer(file);

    writer.write_record(&["Tipo", "Tiempo"])?;

    for _ in 0..10 {
        // Medir tiempo para GET
        let get_start_time = Instant::now();
        reqwest::get(get_url).await?;
        let get_end_time = Instant::now();

        let get_duration = get_end_time.duration_since(get_start_time);
        writer.write_record(&["GET", &format!("{:?}", get_duration)])?;

        // Medir tiempo para POST
        let post_start_time = Instant::now();
        let post_data = json!({
            "title": "foo",
            "body": "bar",
            "userId": 1
        });
        let client = reqwest::Client::new();
        let post_request = client.post(post_url).body(post_data.to_string()).header(reqwest::header::CONTENT_TYPE, "application/json");
        post_request.send().await?;
        let post_end_time = Instant::now();

        let post_duration = post_end_time.duration_since(post_start_time);
        writer.write_record(&["POST", &format!("{:?}", post_duration)])?;
    }

    println!("Pruebas completadas. Resultados almacenados en resultados.csv");
    Ok(())
}
