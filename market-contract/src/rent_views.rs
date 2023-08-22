use crate::*;

#[near_bindgen]
impl Contract {
    /// views
    //get rent info of NFT
    pub fn get_rent_by_token_id(
        &self,
        token_id: TokenId,
    ) -> Option<Rent> {
        return self.rent_by_token.get(&token_id);
    }

    //
    pub fn get_rent_by_account_id(
        &self,
        account_id: AccountId,
    ) ->Vec<RentData> {
        let rent_by_account_id = self.rent_by_account.get(&account_id);

        if let Some (rent_by_account_id) = rent_by_account_id{
            return rent_by_account_id;
        }else{
            return vec![];
        };
    }

}