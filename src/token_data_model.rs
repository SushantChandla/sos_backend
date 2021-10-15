use borsh::{BorshDeserialize, BorshSerialize};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
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

impl Serialize for TokenData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("token_data", 18)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("rarity", &self.rarity)?;
        s.serialize_field("health", &self.health)?;
        s.serialize_field("mana", &self.mana)?;
        s.serialize_field("is_for_sale", &self.is_for_sale)?;
        s.serialize_field("price", &self.price)?;
        s.serialize_field("level", &self.level)?;
        s.serialize_field("category", &self.category)?;
        s.serialize_field("attackpoints", &self.attackpoints)?;
        s.serialize_field("land_required_to_stand", &self.land_required_to_stand)?;
        s.serialize_field("uri", &self.uri)?;
        s.serialize_field("image_uri", &self.image_uri)?;
        s.serialize_field("abilities", &self.abilities)?;
        s.serialize_field("mint_id", &self.mint_id.to_owned())?;
        s.serialize_field("owner", &self.owner.to_string())?;
        s.serialize_field("metadata_at", &self.metadata_at.to_string())?;
        s.serialize_field("seed", &self.seed)?;
        s.serialize_field("owned_account", &self.owned_account.to_string())?;

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
