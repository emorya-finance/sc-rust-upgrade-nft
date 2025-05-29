mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

# deploy
upgrade

# runTx $SC_ADDRESS_HEX "" setUnbondingPeriod@$(python3 to_hex.py 30)

# runTx $SC_ADDRESS "" forceNftClaim@$(mxpy wallet bech32 --decode erd1g5r7uk942pdawx40retuw80g06j3e05z4d3sxh5leffp870j480svw5l8w)

# assignRole

# runTx $OWNER_ADDRESS "" ESDTNFTTransfer@$NFT_HEX@$NONCE@01@$SC_ADDRESS_HEX@$(python3 to_hex.py initialize)

