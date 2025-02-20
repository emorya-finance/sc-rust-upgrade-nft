<sub>*This file has been auto-generated using the [abi-markdowner](https://github.com/0xk0stas/abi-markdowner).*</sub>

# Smart Contract: NftUpgrade

<details>
<summary>Build info</summary>

- **Rustc Version**: 1.80.1
- **Commit Hash**: 3f5fd8dd41153bc5fdca9427e9e05be2c767ba23
- **Commit Date**: 2024-08-06
- **Channel**: Stable

- **Framework**: multiversx-sc
- **Version**: 0.53.2
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

### Other

<details>
<summary>initialize</summary>

Initialize a Test NFT with level 1 in attributes, plus some more info to match current EMR NFTs.

This will make an NFT similar to the current EMR NFTs.
#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>depositNft</summary>

#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>retrieveNft</summary>


</details>

<details>
<summary>claimNft</summary>


</details>

<details>
<summary>upgradeNft</summary>

Upgrade an NFT to the same level but with more data in attributes.
#### Note: This endpoint is payable by any token.


</details>

<details>
<summary>increaseLevel</summary>

#### Note: This endpoint is payable by any token.

#### Inputs:
| Name | Type |
| - | - |
| user | Address |


</details>

<details>
<summary>decreaseLevel</summary>

#### Note: This endpoint is payable by any token.

#### Inputs:
| Name | Type |
| - | - |
| user | Address |


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
<summary>getTags</summary>

#### Outputs:
| Type |
| - |
| bytes |


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
<summary>getNftUris</summary>

#### Inputs:
| Name | Type |
| - | - |
| token_identifier | TokenIdentifier |
| token_nonce | u64 |

#### Outputs:
| Type | List |
| - | - |
| bytes | ✔ |


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
<summary>getNftInfoBeforeUpgrade</summary>

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
<summary>getNftInfoAfterUpgrade</summary>

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
<summary>getNftNonce</summary>

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
| u64 |


</details>

