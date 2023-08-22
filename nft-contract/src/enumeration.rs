use crate::*;

#[near_bindgen]
impl Contract {
    //Query for the total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> U128 {
        //return the length of the token metadata by ID
        U128(self.token_metadata_by_id.len() as u128)
    }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        self.token_metadata_by_id.keys()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128 {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            U128(tokens_for_owner_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    pub fn nft_tokens_by_date(
        &self,
        date: String,
    ) ->Vec<JsonToken> {
        let total = self.token_metadata_by_id.len();
        //iterate through each token using an iterator
        self.token_metadata_by_id.keys()
            //filter
            .filter(|token_id| token_id[4..] == date)
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
    
    pub fn get_random_nfts(
        &self,
        number : Option<u64>,
    )->Vec<JsonToken>{
        //iterate through each token using an iterator
        let keys = self.token_metadata_by_id.keys_as_vector();
        assert!(!keys.is_empty(), "does not have any nft");
        let mut ret = vec![];
        let num = u64::from(number.unwrap_or(10));
        let seeds = env::random_seed();

        for n in 0..num
        {
            let ran = *seeds.get(n as usize).unwrap_or(&0);
            let r = (ran as u64 ) % keys.len();
            if let Some(token_id) = keys.get(r){
                ret.push(self.nft_token(token_id).unwrap());
            }
        }
        ret
    }

    // get next moment for mint and percentage success
    pub fn get_mint(&self) -> (u64, u8) {
        let d = self.last_mint_moment + DEFAULT_MINT_TIME;
        let percent = ((MAX_NUMBER_OF_NFT - self.token_metadata_by_id.len())*100/MAX_NUMBER_OF_NFT) as u8;

        (d, percent)
    }

    // get init time
    pub fn get_init_time(&self) -> (u64) {
        self.last_mint_moment
    }

    // 
    pub fn get_first_mint_address(&self, account_id: AccountId) -> bool {
        let first_mint = self.first_mint_address.iter().filter(|x| **x == account_id).collect::<Vec<_>>();
        first_mint.is_empty()
    }
}
