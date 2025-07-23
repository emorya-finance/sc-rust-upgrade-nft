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

# view getNftFromAddress "0x$(mxpy wallet bech32 --decode erd1xs97t73adfnxaql8sss236tum72cm2pd0laps9m746mv80gqn2gsfmk4h7)"
# view getNftFromAddress "0x$(mxpy wallet bech32 --decode erd1emtp9wqzfg7p5gnqgu3qz4awa9ak9nfrkd6e8fehp597k54fse2sht0qyv)"
# view getNftFromAddress "0x$(mxpy wallet bech32 --decode erd1v5tjaqwed09wshkp7gxp56k66futyvgterr3vthfeqzxvzhzgcrsxe9krp)"
# view getNftFromAddress "0x$(mxpy wallet bech32 --decode erd170d985s433uwwvn8ewnh8lsvaldw2wae6exp2xyapr2sr57rxv6szaa4qw)"
# view getNftFromAddress "0x$(mxpy wallet bech32 --decode erd1s4uumkrz8gjevv64sne7cje6hq9ft2mkm8wwta6dvs379vy2mr0q88x8l6)"


# view getRemainingUnbondingTime addr:erd1xyphe7caprqgm9t05dewgrzvte3a32a9s8ml75wlzuqzht5cyg6skulk3y
# view getRemainingUnbondingTime addr:erd15c4ptcs8szl90l6lq0tjz30a0u4qtptmu74lyp0a6l36qnl8aj8q9l2luc
# view getRemainingUnbondingTime addr:erd14966040u3ha7xynqg7cdulvyzyegc8s6lrt8mtf57mtce7egq8psqj4ws6
# view getRemainingUnbondingTime addr:erd1at6mzc3s9dmxj40kajwrw5zaazxpj6huvmsluxk38cezugc0n63srud6lw
# view getRemainingUnbondingTime addr:erd1qqqqqqqqqqqqqpgqdtpdu6m78t2umrgay3s37np3ntw2zzkamp3snnl370

# view getUserInfo addr:erd1ksv20qd9uu5tzhumvq3dxjgzdg3aw3t05fyghz5mcrsxyeexe0csy805qv

view getCustomNftInfo "0x454d524e46542d303431616266 0x03d6"