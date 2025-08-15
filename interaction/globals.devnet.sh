#=============================================================================
#                                  DEVNET 
#=============================================================================

PROXY="https://devnet-gateway.multiversx.com"
CHAIN_ID="D"

#=================================== SC ===================================

SC_PATH="../"
SC_NAME=$(sed -n 's/^name = "\([^"]*\)".*/\1/p' "$SC_PATH/Cargo.toml")
SC_BYTECODE=$SC_PATH"output/$SC_NAME.wasm"

source $SC_PATH".env.mainnet"

SC_ADDRESS="erd1qqqqqqqqqqqqqpgqwarwdrnq5gnf7jnjcth5l73s0h6p7adeyqdsc8mjle"
if [ ! -z $SC_ADDRESS ]; then
    SC_ADDRESS_HEX=$(mxpy wallet bech32 --decode $SC_ADDRESS)
else
    SC_ADDRESS_HEX=""
fi

#=============================== WALLETS ===============================

source $SC_PATH".env.devnet"

OWNER_PEM=$MY_PEM
OWNER_ADDRESS=$(mxpy wallet convert --infile $OWNER_PEM --in-format pem --out-format address-bech32 | sed -n '3p')
OWNER_ADDRESS_HEX=$(mxpy wallet bech32 --decode $OWNER_ADDRESS)

#=============================== TOKENS ===============================

EGLD="EGLD"
EGLD_HEX=$(python3 to_hex.py $EGLD)

WEGLD="WEGLD-d7c6bb"
WEGLD_HEX=$(python3 to_hex.py $WEGLD)

USDC="USDC-8d4068"
USDC_HEX=$(python3 to_hex.py $USDC)

NFT="EMORYANFT-036f1f"
NFT_HEX=$(python3 to_hex.py $NFT)
NONCE="01"
NONCE_HEX=$(python3 to_hex.py $NONCE)

NFT1="EMORYANFT-036f1f"
NFT_HEX1=$(python3 to_hex.py $NFT1)
NONCE1="02"
NONCE_HEX1=$(python3 to_hex.py $NONCE1)

NFT2="EMORYANFT-036f1f"
NFT_HEX2=$(python3 to_hex.py $NFT2)
NONCE2="03"
NONCE_HEX2=$(python3 to_hex.py $NONCE2)

NFT3="EMORYANFT-036f1f"
NFT_HEX3=$(python3 to_hex.py $NFT3)
NONCE3="04"
NONCE_HEX3=$(python3 to_hex.py $NONCE3)