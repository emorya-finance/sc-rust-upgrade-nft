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


# view getRemainingUnbondingTime addr:erd1xyphe7caprqgm9t05dewgrzvte3a32a9s8ml75wlzuqzht5cyg6skulk3y
# view getRemainingUnbondingTime addr:erd15c4ptcs8szl90l6lq0tjz30a0u4qtptmu74lyp0a6l36qnl8aj8q9l2luc
# view getRemainingUnbondingTime addr:erd14966040u3ha7xynqg7cdulvyzyegc8s6lrt8mtf57mtce7egq8psqj4ws6
# view getRemainingUnbondingTime addr:erd1at6mzc3s9dmxj40kajwrw5zaazxpj6huvmsluxk38cezugc0n63srud6lw
# view getRemainingUnbondingTime addr:erd1qqqqqqqqqqqqqpgqdtpdu6m78t2umrgay3s37np3ntw2zzkamp3snnl370
