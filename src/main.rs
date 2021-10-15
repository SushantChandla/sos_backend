mod routes;
mod solana;
mod token_data_model;
use rocket::routes;
use routes::get_all_cards;
use routes::get_marketplace;
use routes::get_owned;
use routes::index;
use routes::serialize_stream;
use routes::serialize_sell;


#[rocket::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                serialize_stream,
                get_all_cards,
                get_marketplace,
                get_owned,
                serialize_sell
            ],
        )
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
