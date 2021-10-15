use crate::{
    solana::{get_all_account, get_rent_exemption},
    token_data_model::TokenData,
};
use borsh::{BorshDeserialize, BorshSerialize};
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
    let _rent_exempt = get_rent_exemption();
    let _accounts = get_all_account();
    // let mut receiving: Vec<PaymentStreamResponse> = Vec::new();
    // let mut sending: Vec<PaymentStreamResponse> = Vec::new();
    // for acc in accounts {
    //     let program_account = acc.1;
    // let deserialized_data: PaymentStreams =
    //     match PaymentStreams::try_from_slice(&program_account.data) {
    //         Ok(p) => p,
    //         Err(_e) => {
    //             print!("{:?}", _e);
    //             continue;
    //         }
    //     };
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

#[get("/allcards")]
pub fn get_all_cards() -> Json<Value> {
    let _accounts = get_all_account();
    let mut all_account: Vec<TokenData> = Vec::new();

    for acc in _accounts {
        let program_account = acc.1;
        let deserialized_data = match TokenData::try_from_slice(&program_account.data) {
            Ok(p) => p,
            Err(_e) => {
                print!("{:?}", _e);
                continue;
            }
        };
        all_account.push(deserialized_data);
    }

    Json(json!({"code": 200,"result":all_account}))
}

#[get("/marketplace")]
pub fn get_marketplace() -> Json<Value> {
    let _accounts = get_all_account();
    let mut all_account: Vec<TokenData> = Vec::new();

    for acc in _accounts {
        let program_account = acc.1;
        let deserialized_data = match TokenData::try_from_slice(&program_account.data) {
            Ok(p) => p,
            Err(_e) => {
                print!("{:?}", _e);
                continue;
            }
        };
        if deserialized_data.is_for_sale {
            all_account.push(deserialized_data);
        }
    }

    Json(json!({"code": 200,"result":all_account}))
}

#[get("/owned/<public_key>")]
pub fn get_owned(public_key: &str) -> Json<Value> {
    let _accounts = get_all_account();
    let mut all_account: Vec<TokenData> = Vec::new();

    for acc in _accounts {
        let program_account = acc.1;
        let deserialized_data = match TokenData::try_from_slice(&program_account.data) {
            Ok(p) => p,
            Err(_e) => {
                print!("{:?}", _e);
                continue;
            }
        };
        if deserialized_data.owner.to_string().eq(public_key) {
            all_account.push(deserialized_data);
        }
    }

    Json(json!({"code": 200,"result":all_account}))
}
