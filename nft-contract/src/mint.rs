use crate::*;

pub(crate) fn is_valid_date(dateString: TokenId) -> bool //yyyymmdd
{
    let day = &dateString[6..8].parse::<i32>().unwrap();
    let month = &dateString[4..6].parse::<i32>().unwrap();
    let year = &dateString[0..4].parse::<i32>().unwrap();

    // Check the ranges of month and year
    if(*year < 1000 || *year > 3000 || *month == 0 || *month > 12){
        return false;
    }

    let mut monthLength = vec![ 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 ];

    // Adjust for leap years
    if(*year % 400 == 0 || (*year % 100 != 0 && *year % 4 == 0)){
        monthLength[1] = 29;
    }

    // Check the range of the day
    if( *day <= 0 || *day > monthLength[(*month-1) as usize])
    {
        return false;
    }

    // Check limit NFT can be mint
    //19330605 -> 21400101
    if(*year < 1933 || *year > 2140)
    {
        return false;
    }else if(*year == 1933 && (*month < 6 || (*month == 6 && *day < 5)) )
    {
        return false;
    }else if(*year == 2140 && (*month > 1 || *day > 1) )
    {
        return false;
    }
    return true;
}

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        message_url: String,
        //we add an optional parameter for perpetual royalties
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
        price: Option<U128>,
    ) {
        let d =  env::block_timestamp_ms();// - self.last_mint_moment;
        assert!(d >= DEFAULT_MINT_TIME + self.last_mint_moment, "{} ms to next mint", (DEFAULT_MINT_TIME + self.last_mint_moment - d));
        //TODO assert() format token_id
        let is_valid = is_valid_date(token_id.clone());
        assert!(is_valid, "not a validate date");

        let account_id = env::predecessor_account_id();

        let mut mint_fee :U128 = U128(0);

        let first_mint = self.first_mint_address.iter().filter(|x| **x == account_id).collect::<Vec<_>>();
        let first_mint = first_mint.is_empty();
        if account_id == self.owner_id {
            mint_fee = U128(0);
        }
        else if first_mint  //first time mint 
        {
            mint_fee = U128(0);
        }else{
            mint_fee = DEFAULT_MINT_PRICE; 
        }
        
        // check 
        let attached_deposit = env::attached_deposit();

        assert!(attached_deposit > mint_fee.0, "Attached deposit must be greater than mint price: {:?}", mint_fee);

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        // if perpetual royalties were passed into the function: 
        if let Some(perpetual_royalties) = perpetual_royalties {
            //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
            assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");

            //iterate through the perpetual royalties and insert the account and amount in the royalty map
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
            }
        }

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
            //message url to 
            message: message_url,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        //set last mint time
        self.last_mint_moment = env::block_timestamp_ms();

        // 
        if first_mint
        {
            self.first_mint_address.push(account_id);
        }

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage ;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit_mint(required_storage_in_bytes , mint_fee.0);

          // Construct the mint log as per the events standard.
          let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // storage used
                storage_used: required_storage_in_bytes,
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

    }
}