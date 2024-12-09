#=============================================================================
#                                  MAINNET 
#=============================================================================

PROXY="https://gateway.multiversx.com"
CHAIN_ID="1"

#=================================== SC ===================================

SC_PATH="../"
SC_NAME=$(grep -oP 'name = "\K[^"]+' $SC_PATH"Cargo.toml")
SC_BYTECODE=$SC_PATH"output/$SC_NAME.wasm"

SC_ADDRESS="erd1qqqqqqqqqqqqqpgqdtpdu6m78t2umrgay3s37np3ntw2zzkamp3snnl370"
if [ ! -z $SC_ADDRESS ]; then
    SC_ADDRESS_HEX=$(mxpy wallet bech32 --decode $SC_ADDRESS)
else
    SC_ADDRESS_HEX=""
fi

#=============================== WALLETS ===============================

source $SC_PATH".mainnet.env"

OWNER_PEM=$MY_PEM
OWNER_ADDRESS=$(mxpy wallet convert --infile $OWNER_PEM --in-format pem --out-format address-bech32 | sed -n '3p')
OWNER_ADDRESS_HEX=$(mxpy wallet bech32 --decode $OWNER_ADDRESS)

#=============================== ADDRESSES ===============================

WRAPPEDEGLD_ADDRESS="erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3" # shard 1
WRAPPEDEGLD_ADDRESS_HEX=$(mxpy wallet bech32 --decode $WRAPPEDEGLD_ADDRESS)

#=============================== TOKENS ===============================

EGLD="EGLD"
EGLD_HEX=$(python3 to_hex.py $EGLD)

WEGLD="WEGLD-bd4d79"
WEGLD_HEX=$(python3 to_hex.py $WEGLD)

USDC="USDC-c76f1f"
USDC_HEX=$(python3 to_hex.py $USDC)

NFT="EMRNFT-041abf"
NFT_HEX=$(python3 to_hex.py $NFT)

NFT_INVESTORS="EMRNFT-50e862"
NFT_INVESTORS_HEX=$(python3 to_hex.py $NFT_INVESTORS)

NONCE="2500"
NONCE_HEX=$(python3 to_hex.py $NONCE)