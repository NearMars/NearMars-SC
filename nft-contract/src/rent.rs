use crate::*;

pub trait NonFungibleTokenCore{
    // update message link
    fn nft_rent(&mut self, token_id: TokenId, account_id: AccountId, message: String, account_replaced: Option<AccountId>);
}

#[near_bindgen]
impl Contract {
    #[payable]
    fn nft_rent(
        &mut self, 
        token_id: TokenId, 
        account_id: AccountId,
        message: String,
    ) {

    }
}