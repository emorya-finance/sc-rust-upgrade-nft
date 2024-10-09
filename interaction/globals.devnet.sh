#=============================================================================
#                                  DEVNET 
#=============================================================================

PROXY="https://devnet-gateway.multiversx.com"
CHAIN_ID="D"

#=================================== SC ===================================

SC_PATH="../"
SC_NAME=$(grep -oP 'name = "\K[^"]+' $SC_PATH"Cargo.toml")
SC_BYTECODE=$SC_PATH"output/$SC_NAME.wasm"

SC_ADDRESS="erd1qqqqqqqqqqqqqpgqlvnsp42pd08pqu99lyjt3nuu7aqz8vkju7zsk7x4dh"
if [ ! -z $SC_ADDRESS ]; then
    SC_ADDRESS_HEX=$(mxpy wallet bech32 --decode $SC_ADDRESS)
else
    SC_ADDRESS_HEX=""
fi

#=============================== WALLETS ===============================

source $SC_PATH".devnet.env"

OWNER_PEM=$MY_PEM
OWNER_ADDRESS=$(mxpy wallet convert --infile $OWNER_PEM --in-format pem --out-format address-bech32 | sed -n '3p')
OWNER_ADDRESS_HEX=$(mxpy wallet bech32 --decode $OWNER_ADDRESS)

#=============================== TOKENS ===============================

# EGLD="EGLD"
# EGLD_HEX=$(python to_hex.py $EGLD)

# WEGLD="WEGLD-d7c6bb"
# WEGLD_HEX=$(python to_hex.py $WEGLD)

# USDC="USDC-8d4068"
# USDC_HEX=$(python to_hex.py $USDC)

NFT="TESTK-e49c59"
NFT_HEX=$(python to_hex.py $NFT)

NONCE="99"
NONCE_HEX=$(python to_hex.py $NONCE)