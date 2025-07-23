mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

# view getUserInfo addr:erd1xs97t73adfnxaql8sss236tum72cm2pd0laps9m746mv80gqn2gsfmk4h7
# view getCustomNftInfo "0x$(python to_hex.py EMRNFT-041abf) 0x03d6"
# view getCustomNftInfo "0x$(python to_hex.py EMRNFT-041abf) 0x010b"