# Address Generation Process


## 1. HD wallet structure

- How does BIP32 define the HD wallet structure?

    BIP32 organizes keys into a tree structure where each node (key) can generate child nodes. This allows:

    - Unlimited key generation without needing multiple seeds.
    - Parent-child relationships between keys.

    The structure is hierarchical with several levels:

    - ```m``` - The master node (root of the tree).
    - ```m/0``` - The first child key.
    - ```m/0/1``` - The first child of the first child.
    - ```m/0'/0``` - A hardened child key (explained below).

    Path Notation

    - Each level represents a path segment separated by ```/```.
  
    A derivation path specifies a key’s location in the tree:

    - Example: ```m/44'/0'/0'/0/0```
        - ```m``` is the master node.
        - Hardened keys are denoted with an apostrophe (```'```).


## 2. From Master Key to Address

- How does BIP44 establish a standard derivation path for legacy Bitcoin addresses?
    
    BIP44 (Bitcoin Improvement Proposal 44) defines a standard derivation path for HD wallets to generate Bitcoin addresses based on BIP32. It specifies a hierarchical structure that organizes keys by purpose, coin type, account, change, and index, ensuring consistent address generation for legacy Bitcoin (P2PKH) addresses. This standard allows multiple wallets to be derived from a single seed phrase, making key management and recovery easier across platforms.
    
    **Breakdown of the Path Components:**

    - **Purpose (`purpose'`)**
      - Always set to `44'` for BIP44, indicating the wallet adheres to this standard.
      - The apostrophe (`'`) signifies **hardened derivation**.

    - **Coin Type (`coin_type'`)**
      - Specifies the cryptocurrency type, allowing different currencies to coexist in the same wallet.
      - Hardened derivation is used to ensure separation of coins.
      - Common values:
        - `0'` = Bitcoin
        - `1'` = Testnet Bitcoin
        - Other cryptocurrencies have assigned coin types defined in [SLIP-44](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).

    - **Account (`account'`)**
      - Separates accounts within a wallet for organizational purposes.
      - Useful for managing multiple independent accounts.
      - Hardened derivation ensures accounts are isolated from each other.

    - **Change (`change`)**
      - `0` = External addresses: Used for receiving payments.
      - `1` = Internal addresses: Used for change (when sending transactions).
      - Non-hardened derivation is used here.

    - **Address Index (`address_index`)**
      - Non-hardened derivation is used to generate individual addresses within the selected account and change type.
      - Sequential numbers are used (e.g., `0, 1, 2, ...`).
    
- How is a key derived?

    Each key derives a child key using the parent key and chain code:
    - For **non-hardened keys**:
    - Input: Parent **public key**, child index, and parent chain code.
    - In non-hardened derivation, if a child private key and the corresponding extended public key (xpub) are exposed, then the parent private key can be computed.
    - Formula:  
    $$Privkey_{index} = Privkey_{parent} + HMAC_{SHA512}(chaincode, pubkey || index)[:32]$$

    - For **hardened keys**:
    - Input: Parent **private key**, child index (≥ 2³¹), and parent chain code.
    - Hardened derivation prevents access to parent keys if child keys are exposed.
    - Formula:
    $$Privkey_{index} = Privkey_{parent} + HMAC_{SHA512}(chaincode, privkey || index)[:32]$$

    The derived key and chain code(last 32 bytes of hmac_sha512) allow further derivations.
