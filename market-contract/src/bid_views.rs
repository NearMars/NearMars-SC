use crate::*;

#[near_bindgen]
impl Contract {
    //get 
    pub fn get_bid_token_by_token_id(&self, token_id: TokenId) -> Vec<BidToken>{
        let bid_token_by_token_id = self.bid_token_by_token_id.get(&token_id);

        //if there was some set, we set the sales variable equal to that set. If there wasn't, sales is set to an empty vector
        if let Some(bid_token_by_token_id) = bid_token_by_token_id{
            return bid_token_by_token_id;
        } else {
            return vec![];
        };
    }

    pub fn get_bid_token_by_account_id(&self, account_id: AccountId) -> Vec<BidToken>{
        let bid_token_by_account = self.bid_token_by_account.get(&account_id);

        //if there was some set, we set the sales variable equal to that set. If there wasn't, sales is set to an empty vector
        if let Some(bid_token_by_account) = bid_token_by_account{
            return bid_token_by_account;
        } else {
            return vec![];
        };
    }

    pub fn get_bid_token_on_nft_by_account_id(&self, account_id: AccountId, token_id: TokenId) -> Vec<BidToken>{
        let bid_token_by_account = self.bid_token_by_account.get(&account_id);

        //if there was some set, we set the sales variable equal to that set. If there wasn't, sales is set to an empty vector
        if let Some(bid_token_by_account) = bid_token_by_account{
            return bid_token_by_account.into_iter().filter(|a| a.token_id == token_id).collect();
        } else {
            return vec![];
        };
    }

    pub fn get_bid_rent_by_token_id(&self, token_id: TokenId) -> Vec<BidRent>{
        let bid_rent_by_token_id = self.bid_rent_by_token_id.get(&token_id);

        //if there was some set, we set the sales variable equal to that set. If there wasn't, sales is set to an empty vector
        if let Some(bid_rent_by_token_id) = bid_rent_by_token_id{
            return bid_rent_by_token_id;
        } else {
            return vec![];
        };
    }

    pub fn get_bid_rent_by_account_id(&self, account_id: AccountId) -> Vec<BidRent>{
        let bid_rent_by_account = self.bid_rent_by_account.get(&account_id);

        //if there was some set, we set the sales variable equal to that set. If there wasn't, sales is set to an empty vector
        if let Some(bid_rent_by_account) = bid_rent_by_account{
            return bid_rent_by_account;
        } else {
            return vec![];
        };
    }

    pub fn get_bid_rent_on_nft_by_account_id(&self, account_id: AccountId, token_id: TokenId) -> Vec<BidRent>{
        let bid_rent_by_account = self.bid_rent_by_account.get(&account_id);

        //if there was some set, we set the sales variable equal to that set. If there wasn't, sales is set to an empty vector
        if let Some(bid_rent_by_account) = bid_rent_by_account{
            return bid_rent_by_account.into_iter().filter(|a| a.token_id == token_id).collect();
        } else {
            return vec![];
        };
    }

    //return how much bids token an account has paid for
    pub fn bid_token_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.bid_token_deposits.get(&account_id).unwrap_or(0))
    }

    //return how much bids rent an account has paid for
    pub fn bid_rent_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.bid_rent_deposits.get(&account_id).unwrap_or(0))
    }
}