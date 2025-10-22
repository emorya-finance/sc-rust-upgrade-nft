mkdir reports > /dev/null 2>&1
if [ -z $1 ]; then
    source globals.devnet.sh
elif [ $1 -eq 1 ]; then
    source globals.mainnet.sh
fi
source snippets.sh
############ START ############

# view getBlockedUsers "0x$(mxpy wallet bech32 --decode erd1c4umt3yhfs2xrldzer360symjg338y54p95hhk5u995vfaxkuy3q6un9ay)"

# view getNftInfoFromAddress "0x$(mxpy wallet bech32 --decode erd10nua873vmd9ykpkm3dhsvgqskmsuqrtsk9srta8mfln37fkhexqqwk02qe)"
# sleep 2
# view getNftInfoFromAddress "0x$(mxpy wallet bech32 --decode erd13j9t32wdam2r0v9f52y5njm7ynr9y3hzzhq75v8uykjg328sgwfszx3hs9)"
# sleep 2
# view getNftInfoFromAddress "0x$(mxpy wallet bech32 --decode erd1anuwmnxxsrfc9tntn2pnnffprt8vakf8ljsn22vll9kca9wles7sa2tt9e)"
# sleep 2
echo erd1rtgwmv5hvneu9z3e0jhnxdxs8d3vgn43uej7vxs49rl0dsu5ylnqnkgau3
view getNftInfoFromAddress "0x$(mxpy wallet bech32 --decode erd1rtgwmv5hvneu9z3e0jhnxdxs8d3vgn43uej7vxs49rl0dsu5ylnqnkgau3)"
sleep 2
echo erd1ssnxq0c4tvfuf2w7n558yclzrk8ql5duxffxt6v6fxk3w5q2t98sjqkmv2
view getNftInfoFromAddress "0x$(mxpy wallet bech32 --decode erd1ssnxq0c4tvfuf2w7n558yclzrk8ql5duxffxt6v6fxk3w5q2t98sjqkmv2)"
sleep 2
echo erd16sr3r3gm7ga2ksjpej3s637lntacpjws5ar6e97ldxu26zse0suqmm8tm9
view getNftInfoFromAddress "0x$(mxpy wallet bech32 --decode erd16sr3r3gm7ga2ksjpej3s637lntacpjws5ar6e97ldxu26zse0suqmm8tm9)"


