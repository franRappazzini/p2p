#!/bin/bash

# first run `solana-test-validator -r`
# then run `anchor test` to deploy the program and create the mint

# Define variables
ADDRESS1="FadYQNKrUxRosbjaQNMeQvZQCHJVyxGk2vJzshLr3zTg"
ADDRESS2="7zrm4JrkUwNYkUu2BdW1CqXnzH9iZw63Hft1GtXtf1sk"
MINT="8xHZCmJc6JTM6mbQoUCqgE13KEwe9iJcf7vQReqLoctv" # get from logs after `anchor test` command
LOCAL_ADDRESS=$(solana address)

# Token operations
# spl-token mint "$MINT" 10000 "$LOCAL_ADDRESS"
spl-token transfer "$MINT" 10000 "$ADDRESS1" --fund-recipient --allow-unfunded-recipient
    
# Airdrops (to cover transaction fees)
solana airdrop 1 "$ADDRESS1"
solana airdrop 1 "$ADDRESS2"