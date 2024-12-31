# Segwit Tx Parse

## Key Differences Between Legacy and SegWit Transactions

1. **Legacy Transactions**

    - Signature data is part of the scriptSig field in each input.<br><br>
    - Stored directly in the main part of the transaction.
2. **SegWit Transactions**

    - Signature data is moved to the witness field, separate from the main transaction structure.<br><br>
    - The witness data is not included in the part of the transaction that is hashed to compute the transaction ID (txid), making it immune to malleability issues.<br><br>
   - Although segregated, witness data is still included in the transaction and stored in the block.

# Advantages of witness

1. **Fixing Transaction Malleability**

    In legacy transactions, modifying the scriptSig changes the transaction ID (txid), making the transaction malleable.<br><br>
    In SegWit, the txid is computed without the witness data, so changing the witness does not affect the transaction ID.
2. **Block Size Optimization**

    SegWit introduced the concept of block weight and a new limit of 4,000,000 weight units.<br><br>
    Witness data is given less weight (1 byte of witness data = 1 weight unit) compared to non-witness data (1 byte = 4 weight units), effectively increasing the capacity for transactions.

3. **Backward Compatibility**

    SegWit transactions are designed to work with non-SegWit nodes, ensuring compatibility.