import requests
import base64
from multiversx_sdk.core import Address

# === Config ===
TARGET_ACCOUNT = "erd1qqqqqqqqqqqqqpgqdtpdu6m78t2umrgay3s37np3ntw2zzkamp3snnl370"
FILTER_ADDRESS = "erd1l8q3dg7tqqp2dq0gufzlcj70u2303wsaaclvgh9rqttq6udx6qmqtw059r"
AFTER_TIMESTAMP = "1744289952"
API_URL = f"https://api.elrond.com/accounts/{TARGET_ACCOUNT}/transactions?size=8000"

from bech32 import bech32_encode, convertbits

def hex_to_bech32(hex_addr):
    if len(hex_addr) % 2 != 0:
        # If length is odd, pad with a leading 0
        hex_addr = "0" + hex_addr
    try:
        raw_bytes = bytes.fromhex(hex_addr)
        five_bit = convertbits(raw_bytes, 8, 5)
        return bech32_encode("erd", five_bit)
    except Exception as e:
        return None
    
response = requests.get(API_URL)
if response.status_code != 200:
    print(f"Failed to fetch data: {response.status_code}")
    exit(1)

transactions = response.json()
matching_tx_hashes = []

# === Filter logic ===
for tx in transactions:
    action = tx.get("action", {})
    function_name = action.get("name", "")

    if function_name in ["increaseLevel", "decreaseLevel"]:
        data_b64 = tx.get("data")
        if not data_b64:
            continue

        try:
            decoded_data = base64.b64decode(data_b64).decode("utf-8", errors="ignore")
            print("decoded data : "+ decoded_data)
        except Exception as e:
            print(f"Error decoding transaction {tx['txHash']}: {e}")
            continue

        parts = decoded_data.split("@")
        if len(parts) >= 2:
            hex_addr = parts[1]
            try:
                bech32= hex_to_bech32(hex_addr)
                print(bech32)
                if bech32 == FILTER_ADDRESS:
                    matching_tx_hashes.append(tx["txHash"])
            except Exception as e:
                print(f"Invalid hex address in tx {tx['txHash']}: {hex_addr}")
                continue

# === Print results ===
if matching_tx_hashes:
    print("Transactions matching filter:")
    for tx_hash in matching_tx_hashes:
        print(tx_hash)
else:
    print("No matching transactions found.")
