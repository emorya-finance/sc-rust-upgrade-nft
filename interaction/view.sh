mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

# ADDR=erd1tknagty689mt7qzx727anc4re58wwse6yv93pz3znrja8xsgjlqquh06md

# view getUserInfo addr:$ADDR

# view getAllNfts >> allNfts.txt