# LockSwap Contract

A contract that verifies swap maker's order on exchanging sUDT and CKB.

## Transactions

order making:

```yaml
inputs:
    maker's sudt cell
    maker's secp256k1 cell
outputs:
    maker's lockswap cell:
        data:
            sudt number (8 bytes)
            ckb order number (8 bytes)
        lock:
            code_hash: lockswap contract type_id
            hash_type: type
            args: maker's lock script bytes
        type:
            sudt contract script
    maker's secp256k1 change cell
witnesses:
    0: maker's secp256k1 signature
    1: empty
```

order taking:

```yaml
inputs:
    taker's secp256k1 cell
    maker's lockswap cell
outputs:
    taker's sudt cell
    maker's secp256k1 cell
witnesses:
    0: taker's secp256k1 signature
```
