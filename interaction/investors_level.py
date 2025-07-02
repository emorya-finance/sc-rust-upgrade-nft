import requests
import base64
import matplotlib.pyplot as plt
from collections import Counter
import re

url = "https://api.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqa7hy7fe8khlf5lhg7huk3nlqusu2xa6kmp3suhgj4h/nfts?size=150"

response = requests.get(url)
response.raise_for_status()  
nfts = response.json()

target_collection = "EMRNFT-50e862"
filtered_nfts = [nft for nft in nfts if nft.get("collection") == target_collection]

print(len(filtered_nfts))

levels = []

for nft in filtered_nfts:
    attributes_encoded = nft.get("attributes")
    if not attributes_encoded:
        continue
    try:
        decoded = base64.b64decode(attributes_encoded).decode('utf-8')
        match = re.search(r"level:(\d+)", decoded)
        if match:
            level = int(match.group(1))
            levels.append(level)
    except Exception as e:
        print(f"Eroare la decodarea unui NFT: {e}")

level_counts = Counter(levels)

plt.bar(level_counts.keys(), level_counts.values())
plt.xlabel('Nivel')
plt.ylabel('Număr NFT-uri')
plt.title('Distribuția NFT-urilor pe niveluri (EMRNFT-50e862)')
plt.xticks(sorted(level_counts.keys()))
plt.grid(axis='y')
plt.show()