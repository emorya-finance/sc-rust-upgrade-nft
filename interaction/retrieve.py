import subprocess
import requests

contract_address = "erd1qqqqqqqqqqqqqpgqdtpdu6m78t2umrgay3s37np3ntw2zzkamp3snnl370"
retrieve_url = f"https://api.elrond.com/accounts/{contract_address}/transactions?size=1200&function=retrieveNft"

response = requests.get(retrieve_url)
if response.status_code != 200:
    print("Error fetching retrieveNft transactions")
    exit(1)

transactions = response.json()
senders = {tx.get("sender", "") for tx in transactions if tx.get("status", "") == "success"}
print(f"Found {len(senders)} unique senders that called retrieveNft")

unclaimed_senders = []

for sender in senders:
    try:
        result = subprocess.run(
            [
                "mxpy", "contract", "query", contract_address,
                "--function", "getUserRetrieveEpoch",
                "--arguments", f"addr:{sender}",
                "--proxy", "https://gateway.multiversx.com"
            ],
            capture_output=True,
            text=True,
            check=True
        )

        raw_output = result.stdout.strip()

        if raw_output.startswith("[") and raw_output.endswith("]"):
            inner = raw_output[1:-1].strip().strip('"').strip()
            if inner != "" and inner != "0":
                epoch_int = int(inner, 16)
                if epoch_int != 0:
                    print(f"Found unclaimed sender: {sender} → epoch {epoch_int}")
                    unclaimed_senders.append(sender)
            else:
                print(f"No epoch (empty) for sender: {sender}")
        else:
            print(f"Unexpected format for {sender}: {raw_output}")

    except subprocess.CalledProcessError as e:
        print(f"Error querying contract for {sender}: {e.stderr.strip()}")
    except Exception as e:
        print(f"Unexpected error for {sender}: {e}")

decoded_addresses = []

for address in unclaimed_senders:
    try:
        result = subprocess.run(
            ["mxpy", "wallet", "bech32", "--decode", address],
            capture_output=True,
            text=True,
            check=True
        )
        hex_address = result.stdout.strip()
        print(f"Decoded {address} → {hex_address}")
        decoded_addresses.append(hex_address)
    except subprocess.CalledProcessError as e:
        print(f"Error decoding {address}: {e.stderr.strip()}")

output_string = "@".join(decoded_addresses)
with open("decoded_unclaimed.txt", "w") as f:
    f.write(output_string)
print(len(unclaimed_senders))
print("Done. Decoded addresses written to 'decoded_unclaimed.txt'")
