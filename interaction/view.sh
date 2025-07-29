mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############


view getCustomNftInfo "0x$(python to_hex.py EMRNFT-041abf) 0x58"