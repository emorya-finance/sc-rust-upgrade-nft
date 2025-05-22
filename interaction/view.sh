mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

# view getRemainingUnbondingTime "0x$(mxpy wallet bech32 --decode erd14337hy0hddq6pyz8a07v4u9y0enh5s7cy7wqg0lwz3z4euw89tzqnx8y6c)"
# view getUnbondingPeriod ""
# view getUserRetrieveEpoch "0x$(mxpy wallet bech32 --decode erd14337hy0hddq6pyz8a07v4u9y0enh5s7cy7wqg0lwz3z4euw89tzqnx8y6c)"

# view getNftAttributes "0x$OWNER_ADDRESS_HEX 0x$NFT_HEX 0x$NONCE_HEX"
# view getNftUriJson "0x$OWNER_ADDRESS_HEX 0x$NFT_HEX 0x$NONCE_HEX"
view getNftLevelByAddress "0x$(mxpy wallet bech32 --decode erd1e9yx3e0gylqqeczsw7f8y345hc04tx2pxjn0rska849n9hc8lans8k4ncd)"


