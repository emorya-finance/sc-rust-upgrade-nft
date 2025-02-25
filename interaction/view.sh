mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

# view getRemainingUnbondingTime "0x$(mxpy wallet bech32 --decode erd1n65t4rt37g5ep44rddp9l8692g3mzza0hpnh605tsdd2cps0qgkqfxrf58)"
# view getNftAttributes "0x$OWNER_ADDRESS_HEX 0x$NFT_HEX 0x$NONCE_HEX"
# view getNftUriJson "0x$OWNER_ADDRESS_HEX 0x$NFT_HEX 0x$NONCE_HEX"


