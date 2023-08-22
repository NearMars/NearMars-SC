use crate::*;
use near_sdk::promise_result_as_success;

//struct that holds important information about each sale on the market

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RentData {
    pub renting_account_id: AccountId, // AccountId rent this slot
    pub token_id: TokenId,
    pub starts_at: u64, // When rent NFT slot starts being valid, Unix epoch in milliseconds
    pub expires_at: u64, // When rent NFT slot expires, Unix epoch in milliseconds
    pub rent_message: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Rent {
    //owner of the sale
    pub owner_id: AccountId,
    //actual token ID for sale
    pub token_id: String,
    //message link
    pub message_url: String,
    //keep track rented slots
    pub rented_slots: Vec<RentData>,
}
