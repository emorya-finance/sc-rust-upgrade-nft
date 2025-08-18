<sub>*This file has been auto-generated using the [abi-markdowner](https://github.com/0xk0stas/abi-markdowner).*</sub>

# Smart Contract: NftUpgrade

<details>
<summary>Build info</summary>

- **Rustc Version**: 1.86.0
- **Commit Hash**: 05f9846f893b09a1be1fc8560e33fc3c815cfecb
- **Commit Date**: 2025-03-31
- **Channel**: Stable

- **Framework**: multiversx-sc
- **Version**: 0.60.0
</details>

<details>
<summary>Links</summary>

- **Mainnet Deployments**:
  - **[SC](https://explorer.elrond.com/address/erd1qqqqqqqqqqqqqpgqdtpdu6m78t2umrgay3s37np3ntw2zzkamp3snnl370)**: erd1qqqqqqqqqqqqqpgqdtpdu6m78t2umrgay3s37np3ntw2zzkamp3snnl370
- **Devnet Deployments**:
  - **[SC](https://devnet-explorer.elrond.com/address/erd1qqqqqqqqqqqqqpgqwarwdrnq5gnf7jnjcth5l73s0h6p7adeyqdsc8mjle)**: erd1qqqqqqqqqqqqqpgqwarwdrnq5gnf7jnjcth5l73s0h6p7adeyqdsc8mjle
</details>

## Table of Contents

- [Types](#types)
- [Endpoints](#endpoints)
- [Views](#views)

## Types

<details>
<summary>UserNft</summary>

#### Struct Fields:
| Name | Type |
| - | - |
| identifier | TokenIdentifier |
| nonce | u64 |

</details>

<details>
<summary>UserRetrieve</summary>

#### Struct Fields:
| Name | Type |
| - | - |
| counter | u64 |
| unlocking | bool |

</details>

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
<summary>addAllowedAddresses</summary>

#### Inputs:
| Name | Type | MultiValue |
| - | - | - |
| addresses | Address | ✔ |


</details>

<details>
<summary>removeAllowedAddresses</summary>

#### Inputs:
| Name | Type | MultiValue |
| - | - | - |
| addresses | Address | ✔ |


</details>

<details>
<summary>setUnbondingPeriod</summary>

#### Inputs:
| Name | Type |
| - | - |
| period | u64 |


</details>

<details>
<summary>forceNftClaim</summary>

#### Inputs:
| Name | Type |
| - | - |
| token | TokenIdentifier |
| nonce | u64 |
| address | Address |


</details>

### Other

<details>
<summary>initialize</summary>

Only for testing purposes in Devnet.

Initialize a Test NFT with level 1 in attributes, plus some more info to match current EMR NFTs.

This will make an NFT similar to the current EMR NFTs.
#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>depositNft</summary>

Allows a user to deposit an NFT into the contract to be able to "lock" it and enjoy the benefits.

Only 1 NFT per user can be active at a time.
#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>retrieveNft</summary>

Allows a user to iniate the retrieval of an NFT.

This will start the unbonding period.

After the unbonding period, the user can claim their NFT.

While the NFT is in retrieval, it cannot be used for any benefits but the user can deposit a new NFT as active.

</details>

<details>
<summary>claimNft</summary>

Allows a user to claim their NFT after the unbonding period.

This will transfer the NFT back to the user.

The user must have initiated the retrieval process first.

If the unbonding period is not over, the user cannot claim the NFT.

</details>

<details>
<summary>upgradeNft</summary>

Upgrade an NFT to the same level but with the new attributes.

This will update the NFT attributes to match the current EMR NFTs.
#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>increaseLevel</summary>

Increase the level of an NFT by 1.

This can only be done by the owner or an allowed address.
#### Inputs:
| Name | Type |
| - | - |
| user | Address |


</details>

<details>
<summary>decreaseLevel</summary>

Decrease the level of an NFT by 1.

This can only be done by the owner or an allowed address.
#### Inputs:
| Name | Type |
| - | - |
| user | Address |


</details>

<details>
<summary>setLevel</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |
| new_level | u64 |
| category | u64 |


</details>

<details>
<summary>blockUser</summary>

#### Inputs:
| Name | Type | MultiValue |
| - | - | - |
| addresses | Address | ✔ |


</details>

<details>
<summary>unBlockUser</summary>

#### Inputs:
| Name | Type | MultiValue |
| - | - | - |
| addresses | Address | ✔ |


</details>

## Views

<details>
<summary>getIsScPaused</summary>

#### Outputs:
| Type |
| - |
| bool |


</details>

<details>
<summary>getAllowedAddresses</summary>

#### Outputs:
| Type | MultiValue |
| - | - |
| Address | ✔ |


</details>

<details>
<summary>getNftOwnerAddress</summary>

#### Inputs:
| Name | Type |
| - | - |
| nft_token | TokenIdentifier |
| nft_nonce | u64 |

#### Outputs:
| Type |
| - |
| Address |


</details>

<details>
<summary>getNftFromAddress</summary>

#### Inputs:
| Name | Type |
| - | - |
| user | Address |

#### Outputs:
| Type |
| - |
| UserNft |


</details>

<details>
<summary>getNftRetrieveFromAddress</summary>

#### Inputs:
| Name | Type |
| - | - |
| user | Address |

#### Outputs:
| Type |
| - |
| UserNft |


</details>

<details>
<summary>getUserRetrieveEpoch</summary>

#### Inputs:
| Name | Type |
| - | - |
| user | Address |

#### Outputs:
| Type |
| - |
| u64 |


</details>

<details>
<summary>getUnbondingPeriod</summary>

#### Outputs:
| Type |
| - |
| u64 |


</details>

<details>
<summary>getBlockedUsers</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |

#### Outputs:
| Type |
| - |
| bool |


</details>

<details>
<summary>getNftIdentifier</summary>

#### Outputs:
| Type |
| - |
| TokenIdentifier |


</details>

<details>
<summary>getNftIdentifierInvestors</summary>

#### Outputs:
| Type |
| - |
| TokenIdentifier |


</details>

<details>
<summary>getNftAttributes</summary>

#### Inputs:
| Name | Type |
| - | - |
| token_identifier | TokenIdentifier |
| token_nonce | u64 |

#### Outputs:
| Type |
| - |
| bytes |


</details>

<details>
<summary>getNftUriJson</summary>

#### Inputs:
| Name | Type |
| - | - |
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
| token_identifier | TokenIdentifier |
| token_nonce | u64 |

#### Outputs:
| Type |
| - |
| bytes |


</details>

<details>
<summary>getNftLevel</summary>

#### Inputs:
| Name | Type |
| - | - |
| token_identifier | TokenIdentifier |
| token_nonce | u64 |

#### Outputs:
| Type |
| - |
| bytes |


</details>

<details>
<summary>getNftInfoFromAddressBefore</summary>

#### Inputs:
| Name | Type |
| - | - |
| user | Address |

#### Outputs:
| Type |
| - |
| TokenIdentifier |
| u64 |
| u64 |


</details>

<details>
<summary>getNftInfoFromAddress</summary>

#### Inputs:
| Name | Type |
| - | - |
| user | Address |

#### Outputs:
| Type |
| - |
| TokenIdentifier |
| u64 |
| u64 |


</details>

<details>
<summary>getNftInRetrieveByAddress</summary>

#### Inputs:
| Name | Type |
| - | - |
| user | Address |

#### Outputs:
| Type |
| - |
| TokenIdentifier |
| u64 |
| u64 |


</details>

<details>
<summary>getNftLevelByAddress</summary>

#### Inputs:
| Name | Type |
| - | - |
| user | Address |

#### Outputs:
| Type |
| - |
| u64 |


</details>

<details>
<summary>getRemainingUnbondingTime</summary>

#### Inputs:
| Name | Type |
| - | - |
| user | Address |

#### Outputs:
| Type |
| - |
| UserRetrieve |


</details>

<details>
<summary>getUserInfo</summary>

Returns:

- User Active NFT (Identifier, Nonce , Level)

- User in Retrieve NFT (Identifier, Nonce, Level)

- Unbonding Time

- Can Claim



Takes as input the user address.
#### Inputs:
| Name | Type |
| - | - |
| user | Address |

#### Outputs:
| Type |
| - |
| TokenIdentifier |
| u64 |
| u64 |
| TokenIdentifier |
| u64 |
| u64 |
| u64 |
| bool |


</details>

<details>
<summary>getCustomNftInfo</summary>

Returns:

- User Address

- Is in Retrieve

- Unbonding Time



Takes as input the token identifier and nonce of the NFT.
#### Inputs:
| Name | Type |
| - | - |
| token_identifier | TokenIdentifier |
| token_nonce | u64 |

#### Outputs:
| Type |
| - |
| Address |
| bool |
| u64 |


</details>

<details>
<summary>getAllNfts</summary>

Returns all NFTs that are currently deposited in the smart contract.

This includes both the investors' NFTs and the regular NFTs.

It returns a list of tuples containing:

- Token Identifier

- Token Nonce

- Owner Address
#### Outputs:
| Type | MultiValue |
| - | - |
| multi&lt;TokenIdentifier,u64,Address | ✔ |


</details>

