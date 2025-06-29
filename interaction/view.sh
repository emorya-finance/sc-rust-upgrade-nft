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
view getUserRetrieveEpoch "0x$(mxpy wallet bech32 --decode erd1890p8d4vypd5he78zamjmac978d68zzct65q6lx5hxwdf7k7hwqswsnu5l)"

# view getNftAttributes "0x$OWNER_ADDRESS_HEX 0x$NFT_HEX 0x$NONCE_HEX"
# view getNftUriJson "0x$OWNER_ADDRESS_HEX 0x$NFT_HEX 0x$NONCE_HEX"
# view getNftLevelByAddress "0x$(mxpy wallet bech32 --decode erd1tknagty689mt7qzx727anc4re58wwse6yv93pz3znrja8xsgjlqquh06md)"


