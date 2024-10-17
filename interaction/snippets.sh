view() {
    # view func_name  OR  view func_name "0xArg1_hex 0xArg2_hex ..."
    
    if [ -n "$2" ]; then
        mxpy contract query $SC_ADDRESS --proxy $PROXY --function $1 --arguments $2
    else
        mxpy contract query $SC_ADDRESS --proxy $PROXY --function $1
    fi
}

changeOwnerAddress() {
    # changeOwnerAddress new_owner_address_hex

    mxpy tx new \
    --receiver $SC_ADDRESS --recall-nonce --pem $OWNER_PEM \
    --gas-limit 10000000 --outfile ./reports/changeOwnerAddress.report.json \
    --send --value 0 --proxy $PROXY --chain $CHAIN_ID \
    --data ChangeOwnerAddress@$1
}


claimDeveloperRewards() {
    # claimDeveloperRewards

    mxpy tx new \
    --receiver $SC_ADDRESS --recall-nonce --pem $OWNER_PEM \
    --gas-limit 10000000 --outfile ./reports/claimDeveloperRewards.report.json \
    --send --value 0 --proxy $PROXY --chain $CHAIN_ID \
    --data ClaimDeveloperRewards
}

deploy() {
    # deploy OR  deploy "0xArg1_hex 0xArg2_hex ..." 50000000

    local GAS_LIMIT=${2:-40000000}  # Default gas limit is 20 million

    if [ -n "$1" ]; then
        ARGS="--arguments $1"
    else
        ARGS=""
    fi

    mxpy contract deploy --bytecode $SC_BYTECODE \
    --recall-nonce --pem $OWNER_PEM \
    --gas-limit $GAS_LIMIT \
    --send --outfile ./reports/deploy.report.json \
    --proxy $PROXY --chain $CHAIN_ID \
    $ARGS
}

upgrade() {
    # upgrade OR upgrade "0xArg1_hex 0xArg2_hex ..." 50000000

    local GAS_LIMIT=${2:-60000000}  # Default gas limit is 20 million

    if [ -n "$1" ]; then
        ARGS="--arguments $1"
    else
        ARGS=""
    fi
    mxpy contract upgrade $SC_ADDRESS --bytecode $SC_BYTECODE \
    --recall-nonce --pem $OWNER_PEM \
    --gas-limit $GAS_LIMIT \
    --send --outfile ./reports/upgrade.report.json \
    --proxy $PROXY --chain $CHAIN_ID \
    $ARGS
}

# ======================================================================

runTx() {
    # runTx receiver_hex EGLD_value endpoint_name "@Arg1_hex@Arg2_hex@...@ArgN_hex" 20000000

    local RECEIVER=${1:-$SC_ADDRESS} # Default receiver is the SC address
    local EGLD_VALUE=${2:-0}  # Default EGLD value is 0
    local ENDPOINT_NAME=${3:-""}  # Default endpoint name is empty
    local ARGUMENTS=${4:-""}  # Default arguments are empty
    local GAS_LIMIT=${5:-60000000}  # Default gas limit is 20 million
    local REPORT_FILE=${ENDPOINT_NAME:-"tx"} # Default report file is tx.report.json
    local OUTFILE="./reports/$REPORT_FILE.report.json" # Default outfile is ./reports/tx.report.json

    mxpy tx new \
    --receiver $RECEIVER --recall-nonce --pem $OWNER_PEM \
    --gas-limit $GAS_LIMIT --outfile $OUTFILE \
    --send --value $EGLD_VALUE --wait-result \
    --proxy $PROXY --chain $CHAIN_ID \
    --data $ENDPOINT_NAME$ARGUMENTS
}

assignRole () {
    runTx erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u "" setSpecialRole @$NFT_HEX@$SC_ADDRESS_HEX@$(python3 to_hex.py ESDTRoleNFTUpdateAttributes) 60000000
}