mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

view getNftAttributesLevel "0x$(mxpy wallet bech32 --decode erd1cdxt9m8wnnwdufq3gk722tn8wa0lsxlwzfjjqynxgu0dehecyqdsk33nle) 0x$(python3 to_hex.py TESTDICK-d4c12a) 0x01"