use crate::{
    solana::{get_all_account, get_rent_exemption},
    token_data_model::TokenData,
};
use borsh::BorshSerialize;
use rocket::{
    get, post,
    serde::json::{serde_json::json, Json, Value},
};

#[get("/")]
pub fn index() -> &'static str {
    ": )"
}

#[get("/getallstream/<public_key>")]
pub fn get_streams(public_key: &str) -> Json<Value> {
    let rent_exempt = get_rent_exemption();
    let accounts = get_all_account();
    // let mut receiving: Vec<PaymentStreamResponse> = Vec::new();
    // let mut sending: Vec<PaymentStreamResponse> = Vec::new();
    // for acc in accounts {
    //     let program_account = acc.1;
    //     let deserialized_data: PaymentStreams =
    //         match PaymentStreams::try_from_slice(&program_account.data) {
    //             Ok(p) => p,
    //             Err(_e) => {
    //                 print!("{:?}", _e);
    //                 continue;
    //             }
    //         };
    //     if deserialized_data.to.to_string().eq(public_key) {
    //         receiving.push(PaymentStreamResponse::new(
    //             deserialized_data,
    //             &acc.0,
    //             (program_account.lamports - rent_exempt) as i64,
    //         ));
    //     } else if deserialized_data.from.to_string().eq(public_key) {
    //         sending.push(PaymentStreamResponse::new(
    //             deserialized_data,
    //             &acc.0,
    //             (program_account.lamports - rent_exempt) as i64,
    //         ))
    //     }
    // }
    // "public_key":public_key,"receiving":receiving,"sending":sending,
    Json(json!({"code": 200,}))
}

#[post("/serializetokendata", data = "<token_data>")]
pub fn serialize_stream(token_data: Json<TokenData>) -> Json<Value> {
    let temp = token_data.0;
    Json(json!({"code": 200,"result":temp.try_to_vec().unwrap()}))
}
