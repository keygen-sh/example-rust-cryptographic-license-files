use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH, PublicKey, Verifier};
use aes_gcm::{Aes256Gcm, Key, Nonce, Tag};
use aes_gcm::aead::{Aead, NewAead};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::error::{Error};
use clap::{Parser};
use hex::{FromHex};
use std::str;
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
struct LicenseFile<'a> {
  enc: &'a str,
  sig: &'a str,
  alg: &'a str,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short = 'l', long, value_parser)]
  license_key: String,

  #[clap(short = 'k', long, value_parser)]
  public_key: String,

  #[clap(short = 'p', long, value_parser)]
  path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
  let args = Args::parse();
  let license_key: &str = &args.license_key;
  let public_key: &str = &args.public_key;
  let lic_path: &str = &args.path;

  // Parse the hex-encoded public key.
  let public_key: PublicKey = match <[u8; PUBLIC_KEY_LENGTH]>::from_hex(public_key) {
    Ok(bytes) => PublicKey::from_bytes(&bytes)?,
    Err(_) => return Err("failed to parse public key".into()),
  };

  let cert = match fs::read_to_string(lic_path) {
    Ok(content) => content,
    Err(_) => return Err("failed to import license file".into()),
  };

  // Extract the encoded payload from the license file.
  let enc = cert
    .replace("-----BEGIN LICENSE FILE-----", "")
    .replace("-----END LICENSE FILE-----", "")
    .replace('\n', "");

  // Decode the payload.
  let payload = match base64::decode(enc) {
    Ok(bytes) => String::from_utf8(bytes)?,
    Err(_) => return Err("failed to decode license file".into()),
  };

  // Parse the payload.
  let lic: LicenseFile = match serde_json::from_str(payload.as_str()) {
    Ok(json) => json,
    Err(_) => return Err("failed to parse license file".into()),
  };

  // Assert algorithm is supported.
  match lic.alg {
    "aes-256-gcm+ed25519" => (),
    _ => return Err("algorithm is not supported".into()),
  }

  // Verify the license file's signature.
  let msg = format!("license/{}", lic.enc);
  let sig: [u8; SIGNATURE_LENGTH] = match base64::decode(lic.sig)?.try_into() {
    Ok(sig) => sig,
    Err(_) => return Err("signature format is invalid".into()),
  };

  match public_key.verify(msg.as_bytes(), &sig.into()) {
    Ok(_) => (),
    Err(_) => return Err("license file is invalid".into()),
  }

  // Print license file.
  println!("license file was successfully verified!");
  println!("  > {}", serde_json::to_string_pretty(&lic).unwrap());

  // Hash the license key to obtain decryption key.
  let mut sha = Sha256::new();

  sha.update(license_key.as_bytes());

  let digest = sha.finalize();

  // Parse the encrypted data.
  let data: Vec<_> = lic.enc
    .trim()
    .split(".")
    .map(|v| base64::decode(v).expect("failed to parse encrypted data"))
    .collect();

  // Set up data and AES-GCM.
  let mut ciphertext = Vec::from(data[0].as_slice());
  let nonce = Nonce::from_slice(data[1].as_slice());
  let tag = Tag::from_slice(data[2].as_slice());
  let key = Key::from_slice(&digest);
  let aes = Aes256Gcm::new(key);

  // Concat authentication tag with ciphertext.
  ciphertext.extend_from_slice(tag);

  // Decrypt the license file.
  let plaintext = match aes.decrypt(nonce, ciphertext.as_ref()) {
    Ok(plaintext) => String::from_utf8(plaintext)?,
    Err(_) => return Err("failed to decrypt license file".into()),
  };

  // Print decrypted data.
  let obj: serde_json::Value = serde_json::from_str(&plaintext).unwrap();

  println!("license file was successfully decrypted!");
  println!("  > {}", serde_json::to_string_pretty(&obj).unwrap());

  Ok(())
}
