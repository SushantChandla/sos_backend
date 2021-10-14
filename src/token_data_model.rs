use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use serde::{Deserialize, Serialize};
#[derive(BorshSerialize, BorshDeserialize, Debug,Deserialize, Serialize)]
pub struct TokenData {
    pub name: String,
    pub rarity: String,
    pub health: u32,
    pub mana: u32,
    pub is_for_sale: bool,
    pub price: u32,
    pub level: u32,
    pub category: String,
    pub attackpoints: u32,
    pub land_required_to_stand: u32,
    pub uri:String,
    pub abilities: Vec<Ability>,
    pub mint_id: Pubkey,
    pub owner: Pubkey,
    pub metadata_at: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug,Deserialize, Serialize)]
pub struct Ability {
    pub name: String,
    pub description: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug,Deserialize, Serialize)]
struct SellFor(u32);
