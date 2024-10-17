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

assignRole

# runTx $OWNER_ADDRESS "" ESDTNFTTransfer@$NFT_HEX@$NONCE_HEX@01@$SC_ADDRESS_HEX@$(python3 to_hex.py upgradeNft)@$(mxpy wallet bech32 --decode erd16arrrc0daw46y52prfyz0httsdy4dr0a34hngdedz2fuzvsfmp3sld67zc)

