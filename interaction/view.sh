mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

# view getNftAttributesLevelBeforeUpgrade "0x$NFT_HEX 0x$NONCE_HEX"
# view getNftAttributes "0x$OWNER_ADDRESS_HEX 0x$NFT_HEX 0x$NONCE_HEX"
view getNftLevelByAddress "0x$(mxpy wallet bech32 --decode erd16d8t0fm6gp5x2ute024pamg666frmddhzjp5qhz8ady2mfhe6yrsqqdu9n)"
view getNftLevelByAddress "0x$(mxpy wallet bech32 --decode erd18l643ls27yp2l0ky9tth596kws9kppdjm4humdxqw3jxcsky0znq8r0frh)"


