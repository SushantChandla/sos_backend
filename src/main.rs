mod routes;
mod solana;
mod token_data_model;
use routes::index;
use rocket::routes;
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount(
            "/",
            routes![
                index,
            ],
        )
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
