import requests
import time


retrieve_url=f"https://api.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqdtpdu6m78t2umrgay3s37np3ntw2zzkamp3snnl370/transactions?size=1200&function=retrieveNft&after=1740490339"

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
    claimUrl = f"https://api.elrond.com/accounts/{sender}/transactions?size=1200&status=success&function=claimNft&after=1740490339"
    time.sleep(3)
    newResponse = requests.get(claimUrl)
    
    if newResponse.status_code != 200:
        print(f"Error fetching this user {sender} tx")
        exit(1)
    
    if len(newResponse.json()) == 0:
        print(f"Found one sender: {sender} . Adding him now")
        unclaimed_senders.append(sender)
    

print(len(unclaimed_senders))