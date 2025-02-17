mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

# deploy
# upgrade

# assignRole

# runTx $OWNER_ADDRESS "" ESDTNFTTransfer@$NFT_HEX@$NONCE@01@$SC_ADDRESS_HEX@$(python3 to_hex.py initialize)

# runTx erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u "" controlChanges@$(python3 to_hex.py EMR-d10ed9)@$(python3 to_hex.py canFreeze)@$(python3 to_hex.py false)@$(python3 to_hex.py canPause)@$(python3 to_hex.py false)@$(python3 to_hex.py canWipe)@$(python3 to_hex.py false)
runTx erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u "" setSpecialRole@$(python3 to_hex.py EMR-d10ed9)@$(mxpy wallet bech32 --decode $OWNER_ADDRESS)@$(python3 to_hex.py ESDTRoleLocalBurn)