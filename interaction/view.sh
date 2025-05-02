mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

view getRemainingUnbondingTime "0x$(mxpy wallet bech32 --decode erd1rdtdleyq6qmwkc9c2g85fdk9e4jf55kg6s7ly6atygpzxj5mtltsgg45tc)"
# view getNftAttributes "0x$OWNER_ADDRESS_HEX 0x$NFT_HEX 0x$NONCE_HEX"
# view getNftUriJson "0x$OWNER_ADDRESS_HEX 0x$NFT_HEX 0x$NONCE_HEX"


