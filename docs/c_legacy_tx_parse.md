# Legacy Tx Parse

## Data Field Overview

1. **Transaction Version** (4 bytes, `int32`)
   - Specifies the version of the transaction format.
   - Allows for protocol upgrades.

2. **Input Count** (variable length, `VarInt`)
   - Number of transaction inputs.

3. **Inputs** (variable length, list)
   - Each input contains:
     - **Previous Transaction Hash** (32 bytes, `hex`)
       - The hash of the previous transaction.
       - Referenced as the source of funds.
     - **Output Index** (4 bytes, `uint32`)
       - The index of the output in the referenced transaction.
     - **ScriptSig Length** (variable length, `VarInt`)
       - Length of the unlocking script (ScriptSig).
     - **ScriptSig** (variable length, `bytes`)
       - Unlocking script that satisfies the referenced output's locking script.
     - **Sequence Number** (4 bytes, `uint32`)
       - Used for advanced features like Replace-by-Fee (RBF).

4. **Output Count** (variable length, `VarInt`)
   - Number of transaction outputs.

5. **Outputs** (variable length, list)
   - Each output contains:
     - **Value** (8 bytes, `int64`)
       - Amount to transfer in satoshis.
     - **ScriptPubKey Length** (variable length, `VarInt`)
       - Length of the locking script (ScriptPubKey).
     - **ScriptPubKey** (variable length, `bytes`)
       - Locking script that defines conditions for spending the output.

6. **Locktime** (4 bytes, `uint32`)
   - Specifies the earliest time or block height when the transaction can be included in a block.

## Encode Techniques
1. **VarInt: Variable Integer in Bitcoin**

    A `VarInt` (Variable Integer) is a compact, space-efficient format used in Bitcoin to encode integers of variable size. It minimizes the space needed for smaller values while accommodating larger ones when necessary.

   - Encoding Rules

        The length of the `VarInt` depends on the value being encoded:

     - **1 byte** (`0x00` to `0xFC`): For values less than or equal to `252`.
       - Encoded as a single byte.
     - **3 bytes** (`0xFD` + 2 bytes): For values `253` to `65,535` (inclusive).
       - First byte is `0xFD`, followed by the value encoded as a 2-byte little-endian integer.
     - **5 bytes** (`0xFE` + 4 bytes): For values `65,536` to `4,294,967,295` (inclusive).
       - First byte is `0xFE`, followed by the value encoded as a 4-byte little-endian integer.
     - **9 bytes** (`0xFF` + 8 bytes): For values greater than `4,294,967,295`.
       - First byte is `0xFF`, followed by the value encoded as an 8-byte little-endian integer.

   - Example Table

        | Value         | Encoding           | Bytes Used |
        |---------------|--------------------|------------|
        | `1`           | `0x01`             | 1 byte     |
        | `252`         | `0xFC`             | 1 byte     |
        | `253`         | `0xFD 0xFD 0x00`   | 3 bytes    |
        | `500`         | `0xFD 0xF4 0x01`   | 3 bytes    |
        | `70,000`      | `0xFE 0x10 0x11 0x01 0x00` | 5 bytes |
        | `5,000,000,000` | `0xFF 0x00 0xCA 0x9A 0x3B 0x00 0x00 0x00 0x00` | 9 bytes |

   - Advantages of VarInt
     - Reduces storage size for small integers, which are common in Bitcoin transactions.
     - Avoids the need for fixed-width fields, saving space in transaction data.

2. **Little-Endian**
   
    **Little-endian** is a method of storing or transmitting data where the **least significant byte (LSB)** comes first, followed by the more significant bytes. It's one of the two main ways to order bytes in multi-byte data, the other being **big-endian**.


    In Bitcoin tx, little-endian encoding is used for field: txid, vout, previous tx hash, btcoin value, sequence number, locktime


   - Characteristics of Little-Endian

     - The byte representing the smallest part of the number is stored at the **lowest memory address**.
     - Higher memory addresses hold the more significant bytes.

   - Why Little-Endian?

     1. **Historical reasons**: Early computer architectures (e.g., Intel's x86) adopted little-endian for compatibility with existing systems.

     2. **Incremental arithmetic**: Operations like addition can start with the least significant byte and proceed to higher bytes, aligning with how processors execute arithmetic.


## Example in JSON-Like Format
```json
{
  "version": 1,
  "inputs": [
    {
      "txid": "prev_tx_hash",
      "vout": 0,
      "scriptSig": "script_sig_bytes",
      "sequence": 4294967295
    }
  ],
  "outputs": [
    {
      "value": 5000000000,
      "scriptPubKey": "script_pub_key_bytes"
    }
  ],
  "locktime": 0
}
