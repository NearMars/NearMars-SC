# NFT Marketplace Implementation

## Setup environment
```=bash
NFT_MARKETPLACE_ID=nft-market.tommythuy2023.testnet
```

### Deploy your contract
```=bash
near deploy --accountId $NFT_MARKETPLACE_ID --wasmFile out/market.wasm
```

### Initialize Your Marketplace Contract 
```=bash
near call $NFT_MARKETPLACE_ID new '{"owner_id" : "'$NFT_MARKETPLACE_ID'"}'  --accountId $NFT_MARKETPLACE_ID
```

### Deposit for storage on-chain
```=bash
near call $NFT_CONTRACT_ID storage_deposit '{"account_id":"'$MAIN_ACCOUNT'"}' --accountId $MAIN_ACCOUNT  --deposit 0.1
```

### Approval and list NFT to marketplace
```=bash
near call $NFT_CONTRACT_ID nft_approve '{"token_id": "20220816", "account_id" : "'$NFT_MARKETPLACE_ID'", "msg" : "{ \"sale_conditions\" : \"300000000000000000000000\" }"}' --accountId $MAIN_ACCOUNT --deposit 0.01
```

### Update sale price
```=bash
near call $NFT_MARKETPLACE_ID update_price ‘{"nft_contract_id" : "’$NFT_CONTRACT_ID'", "token_id" : "20220816", "price" : "500000000000000000000000" }' --accountId $MAIN_ACCOUNT --depositYocto 1
```

### View sale object by token-id of NFT
```=bash
near view $NFT_MARKETPLACE_ID get_sale '{"nft_contract_token":"'$NFT_CONTRACT_ID'.'20220816'"}'
```

### View sale object by account id owner of NFT
```=bash
near view $NFT_MARKETPLACE_ID get_sales_by_owner_id '{"account_id":"'$MAIN_ACCOUNT'", "from_index":"0", "limit" : '3'}'
```

### View all sales on the market
```=bash
near view $NFT_MARKETPLACE_ID get_sales_by_nft_contract_id '{"nft_contract_id" : "'$NFT_CONTRACT_ID'", "from_index" : "0", "limit": 100}'
```

### Buy selling NFT
```=bash
near call $NFT_MARKETPLACE_ID offer '{"nft_contract_id":"'$NFT_CONTRACT_ID'", "20220816" : "20230101”}’ --accountId buyer1.tommythuy2023.testnet --deposit 0.3 --gas 300000000000000
```
### Cancel selling NFT
```=bash
near call $NFT_MARKETPLACE_ID remove_sale '{"nft_contract_id":"'$NFT_CONTRACT_ID'", "token_id":"20220816"}' --accountId $MAIN_ACCOUNT_2 --depositYocto 1
```

### Bid amount of token (Near) offer to NFT
```=bash
near call $NFT_MARKETPLACE_ID bid_token '{"20220816": "19330605"}' --accountId $MAIN_ACCOUNT  --deposit 0.4
```

### Cancel your bidded items
```=bash
near call $NFT_MARKETPLACE_ID bid_token_cancel_and_widthdraw '{"bid_id": 2}' --accountId $MAIN_ACCOUNT --depositYocto 1
```

### Owner Accept offer bid
```=bash
near call $NFT_MARKETPLACE_ID accept_bid_token '{"nft_contract_id" : "'$NFT_CONTRACT_ID'", "20220816" : "19330605",  "bid_id": 13}' --accountId $BUYER_1 --gas 300000000000000 --depositYocto 1
```

### View bids for your NFT
```=bash
near view $NFT_MARKETPLACE_ID get_bid_token_by_token_id '{"token_id" : "20220816"}'
```

### View your bidded items on specific NFT
```=bash
near view $NFT_MARKETPLACE_ID get_bid_token_on_nft_by_account_id '{"account_id" : "'$BUYER_1'", "token_id": "20220816"}'
```

### Bid amount of token (Near) to rent slot on specific NFT to show your message
```=bash
near call $NFT_MARKETPLACE_ID bid_rent '{"token_id": "20220816", "message" : "https://bafybeih3jqddykasore27van7q7gta6swybu465hgkr4slboauefeoewte.ipfs.dweb.link/20220817_slot_1660716419895.json", "start_at": 1660463599, "expire_at" : 1660467199}' --accountId $MAIN_ACCOUNT  --deposit 0.4
```

### Cancel your bidded rent 
```=bash
near call $NFT_MARKETPLACE_ID bid_rent_cancel_and_widthdraw '{"bid_id" : 0}' --accountId $MAIN_ACCOUNT --depositYocto 1
```

### Accept bidded rent slot of your NFT
```=bash
near call $NFT_MARKETPLACE_ID accept_bid_rent '{"token_id" : "20220816", "bid_id" : 0}' --accountId $MAIN_ACCOUNT --depositYocto 1
```

### List slots rented on specific NFT
```=bash
near view $NFT_MARKETPLACE_ID get_rent_by_token_id '{"token_id" : "20220816"}'
```
 
### List your bidding slots on specific NFT
```=bash
near view $NFT_MARKETPLACE_ID get_bid_rent_by_account_id '{"account_id" : "'$MAIN_ACCOUNT'"}'
```

### List NFTs that you'r renting on
```=bash
near view $NFT_MARKETPLACE_ID get_rent_by_account_id '{"account_id": "'$MAIN_ACCOUNT_2'"}'
```

### List bidding rent slot of specific NFT
```=bash
near view $NFT_MARKETPLACE_ID get_bid_rent_by_token_id '{"token_id" : "20220816"}'
```
