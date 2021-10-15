use borsh::{BorshDeserialize, BorshSerialize};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use solana_program::pubkey::{self, Pubkey};
pub struct TokenDataResponse {
    pub id: Pubkey,
    pub token: TokenData,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Deserialize)]
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
    pub uri: String,
    pub image_uri: String,
    pub abilities: Vec<Ability>,
    pub mint_id: Pubkey,
    pub owner: Pubkey,
    pub metadata_at: Pubkey,
    pub seed: String,
    pub owned_account: Pubkey,
}
impl TokenDataResponse {
    pub fn new(id: Pubkey, data: TokenData) -> Self {
        TokenDataResponse {
            id: id,
            token: data,
        }
    }
}

impl Serialize for TokenDataResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("token_data", 18)?;
        s.serialize_field("name", &self.token.name)?;

        s.serialize_field("rarity", &self.token.rarity)?;
        s.serialize_field("health", &self.token.health)?;
        s.serialize_field("mana", &self.token.mana)?;
        s.serialize_field("is_for_sale", &self.token.is_for_sale)?;
        s.serialize_field("price", &self.token.price)?;
        s.serialize_field("level", &self.token.level)?;
        s.serialize_field("category", &self.token.category)?;
        s.serialize_field("attackpoints", &self.token.attackpoints)?;
        s.serialize_field("land_required_to_stand", &self.token.land_required_to_stand)?;
        s.serialize_field("uri", &self.token.uri)?;
        s.serialize_field("image_uri", &self.token.image_uri)?;
        s.serialize_field("abilities", &self.token.abilities)?;
        s.serialize_field("mint_id", &self.token.mint_id.to_string())?;
        s.serialize_field("owner", &self.token.owner.to_string())?;
        s.serialize_field("metadata_at", &self.token.metadata_at.to_string())?;
        s.serialize_field("seed", &self.token.seed)?;
        s.serialize_field("owned_account", &self.token.owned_account.to_string())?;
        s.serialize_field("id", &self.id.to_string())?;
        s.end()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Deserialize, Serialize)]
pub struct Ability {
    pub name: String,
    pub description: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Deserialize, Serialize)]
struct SellFor(u32);
