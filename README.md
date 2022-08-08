# Example Rust Cryptographic License Files

This is an example of how to verify and decrypt [cryptographic license files](https://keygen.sh/docs/api/cryptography/#cryptographic-lic)
in Rust, using Ed25519 signing and AES-256-GCM encryption.

This example verifies the `aes-256-gcm+ed25519` algorithm.

## Running the example

Install dependencies with [`cargo`](https://doc.rust-lang.org/cargo/):

```bash
cargo build
```

Then run the program:

```bash
cargo run
```

You should see output indicating that the license file is valid, with its
decrypted dataset:

```
license file was successfully verified!
  > {
      "enc": "w4qdbuqLaGdO98bdSS2shu4xX+NwR1uRIWy5...zNZ7ZA0D3mo=.wpu1TdcHVno8zgPI./VzxTIA8sFq0Bix0ljdfxw==",
      "sig": "gCe0gS67opMc9x2qiuZF2o742vd/bHLEADBpr4Bq/NwEGY9wHrEKSvT6Oaiv7cW7LfTUmECNHRjX+X8eQMxMCA==",
      "alg": "aes-256-gcm+ed25519"
    }
license file was successfully decrypted!
  > {
      "data": {
        "id": "5224caee-595b-44cc-bac9-d01787642bac",
        "type": "licenses"
        "attributes": {
          ...
        },
        "relationships": {
          ...
        }
      },
      "included": [
        ...
      ],
      "meta": {
        "expiry": null,
        "issued": "2022-08-08T15:51:17.178Z",
        "ttl": null
      }
    }
```

## Questions?

Reach out at [support@keygen.sh](mailto:support@keygen.sh) if you have any
questions or concerns!
