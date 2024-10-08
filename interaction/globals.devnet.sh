#=============================================================================
#                                  DEVNET 
#=============================================================================

PROXY="https://devnet-gateway.multiversx.com"
CHAIN_ID="D"

#=================================== SC ===================================

SC_PATH="../"
SC_NAME=$(grep -oP 'name = "\K[^"]+' $SC_PATH"Cargo.toml")
SC_BYTECODE=$SC_PATH"output/$SC_NAME.wasm"

SC_ADDRESS="erd1qqqqqqqqqqqqqpgqq9sml0hxc09ytmc9r2242tkkcetwy7vkyqdsqzzuxd"
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

#=============================== ADDRESSES ===============================

WRAPPEDEGLD_ADDRESS="erd1qqqqqqqqqqqqqpgqpv09kfzry5y4sj05udcngesat07umyj70n4sa2c0rp" # shard 1
WRAPPEDEGLD_ADDRESS_HEX=$(mxpy wallet bech32 --decode $WRAPPEDEGLD_ADDRESS)

#=============================== TOKENS ===============================

# EGLD="EGLD"
# EGLD_HEX=$(python to_hex.py $EGLD)

# WEGLD="WEGLD-d7c6bb"
# WEGLD_HEX=$(python to_hex.py $WEGLD)

# USDC="USDC-8d4068"
# USDC_HEX=$(python to_hex.py $USDC)