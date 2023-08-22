# NearMars-Way-to-The-Future

## Setup

Once you've created your near wallet go ahead and login to your wallet with your cli and follow the on-screen prompts

```=bash
near login
```

Once your logged in you have to deploy the contract. Make a subaccount with the name of your choosing 

```=bash 
near create-account nft-example.your-account.testnet --masterAccount your-account.testnet --initialBalance 10
```

After you've created your sub account deploy the contract to that sub account, set this variable to your sub account name

```=bash
NFT_CONTRACT_ID=nft-example.your-account.testnet

MAIN_ACCOUNT=your-account.testnet
```

Verify your new variable has the correct value
```=bash
echo $NFT_CONTRACT_ID

echo $MAIN_ACCOUNT
```


### Deploy Your Contract
```=bash
near deploy --accountId $NFT_CONTRACT_ID --wasmFile out/main.wasm
```

### Initialize Your NFT-Token Contract 

```=bash
near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID
```

### View Contracts Meta Data

```=bash
near view $NFT_CONTRACT_ID nft_metadata
```
### Minting Token

```=bash
near call $NFT_CONTRACT_ID nft_mint '{"token_id": "20230823", "metadata": {"title": "NearMars-Way-to-The-Future Demo day 2"}, "receiver_id": "'$MAIN_ACCOUNT'", "message_url": "https://bafybeiaeolnfjsai6bvf56slqwvtpmsse2f5gljuv4b4tmm6vql2er6t54.ipfs.dweb.link/20220816_test.json"}' --accountId $MAIN_ACCOUNT --amount 0.1
```

After you've minted the token go to wallet.testnet.near.org to `your-account.testnet` and look in the collections tab and check out your new sample NFT! 



## View NFT Information

After you've minted your NFT you can make a view call to get a response containing the `token_id` `owner_id` and the `metadata`

```=bash
near view $NFT_CONTRACT_ID nft_token '{"token_id": "20230823"}'
```

List all NFT minted with month and day
```=bash
near view $NFT_CONTRACT_ID nft_tokens_by_date '{"date" : "0816"}'
```

List all NFT owned by accountID
```=bash
near view $NFT_CONTRACT_ID nft_tokens_for_owner '{"account_id" : "'$MAIN_ACCOUNT'"}'
```

Get random n NFT or get default 10 NFT (without number parameter)
```=bash
near view $NFT_CONTRACT_ID get_random_nfts '{"number" : n}'
```

## Update NFT message

Update message of NFT with the new link url
```=bash
near call $NFT_CONTRACT_ID   '{"token_id" : "20210816", "message_url" : ""}' --accountId $MAIN_ACCOUNT --depositYocto 1
```

## Transfering NFTs

To transfer an NFT go ahead and make another [testnet wallet account](https://wallet.testnet.near.org).

Then run the following
```=bash
MAIN_ACCOUNT_2=your-second-wallet-account.testnet
```

Verify the correct variable names with this

```=bash
echo $NFT_CONTRACT_ID

echo $MAIN_ACCOUNT

echo $MAIN_ACCOUNT_2
```

To initiate the transfer..

```bash=
near call $NFT_CONTRACT_ID nft_transfer '{"receiver_id": "$MAIN_ACCOUNT_2", "token_id": "20210816", "memo": ""}' --accountId $MAIN_ACCOUNT --depositYocto 1
```

In this call you are depositing 1 yoctoNEAR for security and so that the user will be redirected to the NEAR wallet.

Note

NearMars-Way-to-The-Future

Mod Offer 
+ Buyer : Offer with lower sale price : lock token bidded to market account.
+ Seller  : Accept offer.
+ Condition : Seller accept or Buyer cancel.

Add Rent slot
+ add rented_account_id: UnorderedSet<AccountId> to Token, JsonToken. 

Save reference data(text) to centralized db or on-chain


#### NFT CONTRACT


export NFT_CONTRACT_ID=nft-contract-test1.hdtung.testnet
export NFT_MARKETPLACE_ID=nft-market-test.hdtung.testnet
export MAIN_ACCOUNT=hdtung.testnet
export MAIN_ACCOUNT_2=stack2829.testnet
export BUYER_1=buyer1.hdtung.testnet
export BUYER_2=buyer2.hdtung.testnet

## delete
near delete $NFT_CONTRACT_ID hdtung.testnet	

near create-account $NFT_CONTRACT_ID —masterAccount hdtung.testnet --initialBalance 10
near create-account $NFT_MARKETPLACE_ID —masterAccount hdtung.testnet --initialBalance 10

###deploy nft contract 
near deploy --accountId $NFT_CONTRACT_ID --wasmFile out/main.wasm

###init nft contract 
near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID

## view
near view $NFT_CONTRACT_ID nft_metadata

### mint nft
near call $NFT_CONTRACT_ID nft_mint '{"token_id": "20230030”, "metadata": {"title": "Dollar Day", "description": "history", "media": ""}, "receiver_id": "'$MAIN_ACCOUNT'", "message_url": "https://gateway.pinata.cloud/ipfs/QmTfk6u3W7PGhY4Y1Zt32TuJghmi7jfGTSxX95fFQJDFp2/19330605.txt"}' --accountId $MAIN_ACCOUNT --amount 0.1

### view nft
near view $NFT_CONTRACT_ID nft_token '{"token_id":"token-2"}'

### test transfer nft 
near call $NFT_CONTRACT_ID nft_transfer '{"receiver_id" : "'$MAIN_ACCOUNT_2'", "token_id": "token-1", "memo" : "NearMars-Way-to-The-Future NFT"}' --accountId $MAIN_ACCOUNT --depositYocto 1


####￼ MARKETPLACE CONTRACT

export NFT_MARKETPLACE_ID=nft-marketplace.hdtung.testnet

#￼# # deploy nft marketplace contract 
near deploy --accountId $NFT_MARKETPLACE_ID --wasmFile out/market.wasm

###init -> set owner
near call $NFT_MARKETPLACE_ID new '{"owner_id" : "'$NFT_MARKETPLACE_ID'"}'  --accountId $NFT_MARKETPLACE_ID

###before list nft to market, account need to deposit for storage
near call $NFT_CONTRACT_ID storage_deposit '{"account_id":"'$MAIN_ACCOUNT'"}' --accountId $MAIN_ACCOUNT  --deposit 0.1

### list nft to market by call approval to nft contract with price
near call $NFT_CONTRACT_ID nft_approve '{"token_id": "token-2", "account_id" : "'$NFT_MARKETPLACE_ID'", "msg" : "{ \"sale_conditions\" : \"300000000000000000000000\" }"}' --accountId $MAIN_ACCOUNT --deposit 0.01

### view sale object from token-id
near view $NFT_MARKETPLACE_ID get_sale '{"nft_contract_token":"nft-contract.hdtung.testnet.token-2"}'

### view sales object from accountId 
near view $NFT_MARKETPLACE_ID get_sales_by_owner_id '{"account_id":"'$MAIN_ACCOUNT'", "from_index":"0", "limit" : '3'}'

###  buy
near call $NFT_MARKETPLACE_ID offer '{"nft_contract_id":"'$NFT_CONTRACT_ID'", "token_id" : "token-1”}’ --accountId buyer1.hdtung.testnet --deposit 0.3 --gas 300000000000000


### update-sell-price
near call nft-marketplace.hdtung.testnet update_price '{"nft_contract_id" : "nft-contract.hdtung.testnet", "token_id" : "token-1", "price" : "300000000000000000000000" }' --accountId $MAIN_ACCOUNT --depositYocto 1


### bid token
near call $NFT_MARKETPLACE_ID bid_token '{"token_id": "19330605"}' --accountId $MAIN_ACCOUNT  --deposit 0.4

### view bid token by account id
near view $NFT_MARKETPLACE_ID get_bid_token_by_account_id '{"account_id" : "'$MAIN_ACCOUNT'"}' --accountId $MAIN_ACCOUNT

### view bids token by tokenid
near view $NFT_MARKETPLACE_ID get_bid_token_by_token_id '{"token_id" : "19330605"}' --accountId $MAIN_ACCOUNT

### cancel
near call $NFT_MARKETPLACE_ID bid_token_cancel_and_widthdraw '{"bid_id" : 0}' --a
ccountId $MAIN_ACCOUNT

### bid rent
near call $NFT_MARKETPLACE_ID bid_rent '{"token_id": "19330605", "message" : "http:://google.com", "start_at": 1660463599, "expire_at" : 1660467199}' --accountId $MAIN_ACCOUNT  --deposit 0.4

### view bid  rent by tokenid 
near view $NFT_MARKETPLACE_ID get_bid_rent_by_token_id '{"token_id" : "19330605"}' --accountId $MAIN_ACCOUNT

### view bid rent by account id
near view $NFT_MARKETPLACE_ID get_bid_rent_by_account_id '{"account_id" : "'$MAIN_ACCOUNT'"}' --accountId $MAIN_ACCOUNT

### accept bid rent
near call $NFT_MARKETPLACE_ID accept_bid_rent '{"token_id" : "19330605", "bid_id" :3}’ --accountId $MAIN_ACCOUNT_2 --depositYocto 1

### view rented by token id 
near view $NFT_MARKETPLACE_ID get_rent_by_token_id '{"token_id" : "19330605"}' --accountId $MAIN_ACCOUNT
