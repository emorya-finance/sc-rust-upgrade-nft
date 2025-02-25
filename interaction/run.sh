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

# assignRole

# runTx $OWNER_ADDRESS "" ESDTNFTTransfer@$NFT_HEX@$NONCE@01@$SC_ADDRESS_HEX@$(python3 to_hex.py initialize)

