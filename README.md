<sub>*This file has been auto-generated using the [abi-markdowner](https://github.com/0xk0stas/abi-markdowner).*</sub>

# Smart Contract: NftUpgrade

<details>
<summary>Build info</summary>

- **Rustc Version**: 1.79.0
- **Commit Hash**: 129f3b9964af4d4a709d1383930ade12dfe7c081
- **Commit Date**: 2024-06-10
- **Channel**: Stable

- **Framework**: multiversx-sc
- **Version**: 0.53.2
</details>

## Table of Contents

- [Endpoints](#endpoints)
- [Views](#views)

## Endpoints

### Deploy

<details>
<summary>init</summary>


</details>

### Upgrade

<details>
<summary>upgrade</summary>


</details>

### Owner Only

<details>
<summary>pauseSc</summary>


</details>

<details>
<summary>resumeSc</summary>


</details>

<details>
<summary>setEmrNftIdentifier</summary>

#### Inputs:
| Name | Type |
| - | - |
| identifier | TokenIdentifier |


</details>

### Other

<details>
<summary>initialize</summary>

Initialize a Test NFT with level 1 in attributes, plus some more info to match current EMR NFTs.

This will make an NFT similar to the current EMR NFTs.
#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>upgradeNft</summary>

Upgrade an NFT to the same level but with more data in attributes.
#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>increaseLevel</summary>

Increase the level of an NFT by 1.
#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>decreaseLevel</summary>

Decrease the level of an NFT by 1.
#### Note: This endpoint is payable by any token.


</details>

## Views

<details>
<summary>getNftAttributes</summary>

#### Inputs:
| Name | Type |
| - | - |
| owner | Address |
| token_identifier | TokenIdentifier |
| token_nonce | u64 |

#### Outputs:
| Type |
| - |
| bytes |


</details>

<details>
<summary>getNftAttributesLevelBeforeUpgrade</summary>

#### Inputs:
| Name | Type |
| - | - |
| owner | Address |
| token_identifier | TokenIdentifier |
| token_nonce | u64 |

#### Outputs:
| Type |
| - |
| bytes |


</details>

<details>
<summary>getNftAttributesLevelAfterUpgrade</summary>

#### Inputs:
| Name | Type |
| - | - |
| owner | Address |
| token_identifier | TokenIdentifier |
| token_nonce | u64 |

#### Outputs:
| Type |
| - |
| bytes |


</details>

<details>
<summary>getEmrNftIdentifier</summary>

#### Outputs:
| Type |
| - |
| TokenIdentifier |


</details>

<details>
<summary>getIsScPaused</summary>

#### Outputs:
| Type |
| - |
| bool |


</details>

