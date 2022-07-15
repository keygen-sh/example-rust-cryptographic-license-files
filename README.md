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
  > LicenseFile { enc: "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.3WmLxP1dOFDvipnQ.+ALxsp9z4RZZGnee39wY1g==", sig: "Mm3rDMgOR1DXQW8KvIbr8ljqLaFO1ezWj9GwNp+6cnLLd1rHscx7lierR8UbR6X3ErP44nZqjbdyG0/xk8uDAA==", alg: "aes-256-gcm+ed25519" }
license file was successfully decrypted!
  > {"data":{"id":"a4797354-b53a-4641-b14d-89c4e87f9412","type":"licenses","attributes":{"name":"Java Example","key":"33362C-D254BA-F54C3C-DAAE48-C71975-V3","expiry":null,"status":"ACTIVE","uses":0,"suspended":false,"scheme":null,"encrypted":false,"strict":false,"floating":false,"concurrent":false,"protected":true,"maxMachines":1,"maxCores":null,"maxUses":null,"requireHeartbeat":false,"requireCheckIn":false,"lastValidated":null,"lastCheckIn":null,"nextCheckIn":null,"metadata":{},"created":"2022-04-01T13:17:43.813Z","updated":"2022-04-01T13:17:43.813Z"},"relationships":{"account":{"links":{"related":"/v1/accounts/1fddcec8-8dd3-4d8d-9b16-215cac0f9b52"},"data":{"type":"accounts","id":"1fddcec8-8dd3-4d8d-9b16-215cac0f9b52"}},"product":{"links":{"related":"/v1/accounts/1fddcec8-8dd3-4d8d-9b16-215cac0f9b52/licenses/a4797354-b53a-4641-b14d-89c4e87f9412/product"},"data":{"type":"products","id":"6db9ac6e-ea9e-4943-8462-a0315dda0f2e"}},"policy":{"links":{"related":"/v1/accounts/1fddcec8-8dd3-4d8d-9b16-215cac0f9b52/licenses/a4797354-b53a-4641-b14d-89c4e87f9412/policy"},"data":{"type":"policies","id":"70bda6e4-2b9e-4100-946a-103164a2abc6"}},"group":{"links":{"related":"/v1/accounts/1fddcec8-8dd3-4d8d-9b16-215cac0f9b52/licenses/a4797354-b53a-4641-b14d-89c4e87f9412/group"},"data":null},"user":{"links":{"related":"/v1/accounts/1fddcec8-8dd3-4d8d-9b16-215cac0f9b52/licenses/a4797354-b53a-4641-b14d-89c4e87f9412/user"},"data":null},"machines":{"links":{"related":"/v1/accounts/1fddcec8-8dd3-4d8d-9b16-215cac0f9b52/licenses/a4797354-b53a-4641-b14d-89c4e87f9412/machines"},"meta":{"cores":0,"count":0}},"tokens":{"links":{"related":"/v1/accounts/1fddcec8-8dd3-4d8d-9b16-215cac0f9b52/licenses/a4797354-b53a-4641-b14d-89c4e87f9412/tokens"}},"entitlements":{"links":{"related":"/v1/accounts/1fddcec8-8dd3-4d8d-9b16-215cac0f9b52/licenses/a4797354-b53a-4641-b14d-89c4e87f9412/entitlements"}}},"links":{"self":"/v1/accounts/1fddcec8-8dd3-4d8d-9b16-215cac0f9b52/licenses/a4797354-b53a-4641-b14d-89c4e87f9412"}},"meta":{"issued":"2022-04-01T13:18:09.930Z","expiry":"2022-05-01T13:18:09.930Z","ttl":2629746}}
```

## Questions?

Reach out at [support@keygen.sh](mailto:support@keygen.sh) if you have any
questions or concerns!
