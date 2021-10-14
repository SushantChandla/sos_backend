mod routes;
mod solana;
mod token_data_model;
use routes::index;
use rocket::routes;
use routes::serialize_stream;
#[rocket::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                serialize_stream
            ],
        )
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
