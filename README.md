# Example Rust Cryptographic License Files

This is an example of how to verify and decrypt [cryptographic license files](https://keygen.sh/docs/api/cryptography/#cryptographic-lic)
in Rust, using Ed25519 signature verification and AES-256-GCM encryption.

This example verifies the `aes-256-gcm+ed25519` algorithm.

## Running the example

Install dependencies with [`cargo`](https://doc.rust-lang.org/cargo/):

```bash
cargo build
```

Then run the example program, where `-p` is the path to a license file,
`-k` is your Ed25519 public key, and `-l` is a license key. Feel free
to use these example values:

```bash
cargo run -- -p examples/license.lic \
  -k "e8601e48b69383ba520245fd07971e983d06d22c4257cfd82304601479cee788" \
  -l "988214-879010-F1185E-B37E91-E53AF5-V3"
```

You should see output indicating that the license file is valid, with its
decrypted dataset:

```
license file was successfully verified!
  > {
      "enc": "WJduvnfelNL8XxZPo6CJ4RACcIEpGAasx...uY317MosowW5zAbD4w==.hDyId1HHbc5XxF9U.gkMKTqOZDz35+ehHOkO+wQ==",
      "sig": "oSePC/4ADN6KV0FWs0qMEn7THgrAai6011SQqwXMSQsu8OA5eEyAaz0xjes14nJL6gv4zig5VYTfYYTCbRhWBQ==",
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
        "issued": "2022-08-08T16:37:55.036Z",
        "expiry": "2023-08-08T16:37:55.036Z",
        "ttl": 31556952
      }
    }
```

## Questions?

Reach out at [support@keygen.sh](mailto:support@keygen.sh) if you have any
questions or concerns!
