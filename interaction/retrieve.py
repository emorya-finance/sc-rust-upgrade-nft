import subprocess
import requests
import time


retrieve_url=f"https://api.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqdtpdu6m78t2umrgay3s37np3ntw2zzkamp3snnl370/transactions?size=1200&function=retrieveNft&after=1746403200"

response = requests.get(retrieve_url)

if response.status_code != 200:
    print("Error not fetched well.")
    exit(1)

transactions = response.json()
senders = []
unclaimed_senders = []


for body in transactions:
    status = body.get("status", "")
    if status == "success":
        senders.append(body.get("sender",''))

print(len(senders))

for sender in senders: 
    claimUrl = f"https://api.elrond.com/accounts/{sender}/transactions?size=1200&status=success&function=claimNft&after=1746403200"
    time.sleep(3)
    newResponse = requests.get(claimUrl)
    
    if newResponse.status_code != 200:
        print(f"Error fetching this user {sender} tx")
        exit(1)
    
    if len(newResponse.json()) == 0:
        print(f"Found one sender: {sender} . Adding him now")
        unclaimed_senders.append(sender)
        
decoded_addresses = []

for address in unclaimed_senders:
    try:
        result = subprocess.run(
            ["mxpy", "wallet", "bech32", "--decode", address],
            capture_output=True,
            text=True,
            check=True
        )
        print(f"Sender address decode from: {address} into {result.stdout.strip()}")
        hex_address = result.stdout.strip()
        decoded_addresses.append(hex_address)
    except subprocess.CalledProcessError as e:
        print(f"Error decoding {address}: {e.stderr.strip()}")

output_string = "@".join(decoded_addresses)

with open("decoded_unclaimed.txt", "w") as f:
    f.write(output_string)

print("Done. Decoded addresses written to 'decoded_unclaimed.txt'")

