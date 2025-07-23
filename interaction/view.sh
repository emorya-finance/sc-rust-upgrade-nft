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
# view getUserInfo "0x$(mxpy wallet bech32 --decode erd1cdxt9m8wnnwdufq3gk722tn8wa0lsxlwzfjjqynxgu0dehecyqdsk33nle)"

# view getNftAttributesLevelAfterUpgrade "0x$NFT_HEX 0x01"

view getUserInfo addr:erd1xs97t73adfnxaql8sss236tum72cm2pd0laps9m746mv80gqn2gsfmk4h7

# EMRNFT-041abf-010b
view getCustomNftInfo "0x$(python to_hex.py EMRNFT-041abf) 0x03d6"
view getCustomNftInfo "0x$(python to_hex.py EMRNFT-041abf) 0x010b"