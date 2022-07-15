use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH, PublicKey, Verifier};
use aes_gcm::{Aes256Gcm, Key, Nonce, Tag};
use aes_gcm::aead::{Aead, NewAead};
use sha2::{Sha256, Digest};
use serde::{Deserialize};
use std::error::{Error};
use hex::FromHex;
use std::str;

const KEYGEN_LICENSE_FILE: &str = "-----BEGIN LICENSE FILE-----\neyJlbmMiOiJRbmVpZXZvMDl4clZWcXdMYm1MVCt0RU5wa1A5Z0ZqMlg2cmJZ\nNDFNZ0UzamlQZGVRRE9SZS8wZU5CbU9ZbnRvU3JHQzhUa1dYVzRQQWFoREdT\ncUR0WHhuQ3RTY09hRlJtMzI0M2w1TU8wNDlpaVE3RlZuaHlyeGNrb2loNUUx\neUdOSlMvN1VRcFhFTUNuaWwxTFVTL1BMWVk4dGlKQmV6bThqQlVGUVN4enRW\nVWUrMFVGelZLSW5VZ0VYY1dSQk1INHVlU3g4L1NibWkxY2twSWNES2hIajdo\nVnlrT0xFanNmMXpRb1FOY0pSejZHM0J2SHJmcHpJazZ0blppaVdGY3c4akp2\nR2pGNWpjOEl3ZGpxQjhaLzJmOGlHTi9YQm14RlNKWWZvOHJxSUZHVDZPbFZl\nOUpJMGRRZjVCTE1neStBWXU5K05nbE5mMlpLMFNkZk93aDFnN1ArNDEyRTBn\ncXJqcWUxUXNOOWRGZDFPcytuRjR1MS9sVklwMFp0RVdoUlQvc0U3MldRcGQx\neHdqenB5dEtNT2V0bG1reDVKVE03TldOVk15d1FoanRVbm5Zc0wwNTRLaURU\nTlZ5dmQ4RHBjTmViT2tpR0xzNVhUa1B2TWJWSkZGRlEvdk5Mam5qZVRucVF3\nS1lhNTkrdVdhZnlIYnVuUUt1cmNSRHJQQUZaZjFmSThzWHBXd2lGY0lQY3JJ\nZ3NwY0lQZVFuYTJJWUx0ZExOZnhrYlNzMmRVemphMWFvVGRPWVBYbVJDUVUv\nZGhqb0tCZTRzZERiL1VqR1FBZnFFVzB1V1pqbTNmR0c5SmY3RExROEZ2cTVw\nNnNLNCtpNjFSR3NVeXdxVFgydkdxM0ZyZmpGRlhuNTViUDJNQ2YvV1Awc3RO\nbFpOYWhPR2xFam9WY3dyeGFkOERBY2E4VGtqdXZ0RHdnakhvL2NmZnpsM295\nVjFWMHAvcVFGMTlOL0Ztc0Q3dXBuVWRjM1JMTmZFYkU1bmZLb0pQZTExVU82\nRHZCdThXTFhKS1U0Z2p0dTNFTVViaXpXTThYR1pjeTlOZWsxUVF2dFc3a294\nYXo4aTNBSEx0YWlnS2tQREVPdTJzbTJyN1dvWmpVcWhkUFpZdUNQN2UrZVh5\nb3NaSy9nbWZSZXU1WkJnV0dpcnM1eTdHRStSYnZlOWpSb2dnekEvSHc2aHA4\nQ1JWUG4vS2ZxUkdJOUdxd2gxbll2SXl2bE5OZXpHMzBqSTM0S1hYRU9DQTFs\nU3pxckhSSFZFc2VGWWlwZjd5TjZ3SjJoYk5zUkhmd3h2ZUFLSWZHUmo0Q0Ja\nNlNwVWcvVndQT2swblc5Y3grWWR2TkxUa1FjWm1Yc0xmYjNZUkszNlcyYUJB\nUXFCL3lMbXNRMjRvdHdjSlhLR2wwNzFoa0tVdXdzb0ZyUTdtZVU1dTNKQkpG\nVjdGeVJHSjNFcTZHdzZ1TFNGZDVhbGV6eGZXVmd6TXpsYzhQL1hjb1piZ3Bi\nZDJGRWhpQ0lqblQ1RzAwZEZENURsM0h1bjY0RFZpVGhmbFZmOHY4cURqakVE\naG9ER2ZSRlVqWEd6L3dDbGN2YjFZUnlHUW9iTGhqblcrek96Mlp6aTRtR1V3\nK0t0Y3dnOUNlZ1BRelI4MVp2V1g0WEc0UFBTUzV2V3g2aWVHMWpaTUZzRHBZ\ncUJrazR1NG85YkR3WFdLbFNkeVFYUlJKU2NtRmdUdnF2UUtmMjVHKy9DZFZY\nV2tQbmNMU0s4WTgrOHY2bHkwYW50WjJkQVVDRE9FWHAzL3hNOVlQZnlXb2Q5\neUhoRStuaUl5WDJ1S0hkZldLWE5uUDdiVFZRblczYVdUeGhRMmt0VjloS1F6\nTzdqOTRzamxoYnZMN2JTZU10OUZOVDRLKzd3VTdKaGNEU1oxV0Z3SEx6b0NO\nV2JiMjUrUGd4RWRRR0N1Wk5hZTZZbHlYMlY4QnhLdjA4NzRZNmx2Z0YxdVBG\nbnk0RXpSTUZBUGhiM2d3cG03dytLVlVOVVpmV1ZUcUltaHRPZFUxSzBpczNL\nbi9YRkoxQ2FRVnZxalVhVFhSTzY0aEFYMzlUWlRqam43REpFTXBZWk8wSHIv\nZENzZkZyRDVRanZSU1FTNTh4OWgxWFNua05GRzVTM1prNmFUVTlSOWVmU1h1\nVVVJblFqcDJCVjYzeHJ3STZWR1pHMlBFR3BoWTZZTVEyOFNtcVNMZ2JqUkVL\naWtFTXZUS0EwT2Vva3g2U1VvR0FWbUpEbXJZSHVIMjNTYUFweEt1YVNzUVMw\nSU4vVlFEeU1qNlZsMENZZE44K2lwd1lHZElYUFpCZkpzNlBDTnBkczVYTzI1\nSHFWZUVXRk5acitGNElTMmlRWGlEbkV6anozUk4zSWpGT2IzdnlzbkN6Zito\nVnRKWVNsOTRlYWhuOXhnOEN5TWV2c0IxREpTWFJPeGFwSVRqem9DTDloZXRU\nd1Z3Q0xCS2FEZWZqQUVmRUZYMm5raW5oSEpTeE1EWkM1Z0dBMThCYUdkQlJQ\nYUNNbzhhYWJlRFNXUXJIMFpjbjUwUlVCTVUwaTF5QlNnNGd0djZCTG5NdDQy\nOGxPSXJ4YUR6bm43VTEwbkFVOHlCN1YzQzV4aGxOL1d1OWpnMVhzcW9lanYz\nTm0vNjMzajRyR3g5SFQ4N1JFUG5zaTVIRzlLR0NHcVlhN2FxNmFjVzRCYmVh\nakVsb2xiK1lmbEVIeEU1M0JnVGVLQi9mMjVId3VWTk03bnVzWndENThkWXpj\nYzhBWnRSb0VOWmxuRWMrQk03VHZwbUJCNzhUQWFXUzBiaVVJUDZkcis2RG1u\nWjlmaS9NOFpOSng3cExPNmJQSGljT0ZIbW96WE03VDlnVkhrcVNIRmQ5T2F5\ncmorZ3ZVUk9lNTBxMGZITWlWRXNjamtxR2tCVFhaL1RHS0MvYVNUR05hZjhu\nWlQ0akFiM1ZmcENaWE80RElGWGwzTUc4R3FqTmZtbkZuZHl6aklYNEVEcUEr\nTU15UnJCRGNuK1kzbjJDRVNtYi95eVZTY2NiUmpFeU1RaFBjWUUrT3duV2Mx\nYksyMXR0eHNIZXF0S1IxbXI4bWlxbHk3S3NtaG5XNldxOVFBc096WW5DNW1s\nU0RjV2FrSkNWdEhHZitBTUl4RGtxb2lkYVZoaFRKWld1TUxKSTFqRUNvZzFL\nYkMzOG1aMXFmMXg4eXBraS9DZ3FUdC9CdGV0MmNYcGxNa1p0ZVhHak5CVkI4\nUEtma0NUMnNiYUlPVS83NTZJY0NmZ2NTQVM5SnJyQTNZc3JUaW9pNHBWM2NN\nR0tiMTdabEovYXVLV2UxVHVEMmJqUUZIYVVveFc3UzNMcGxFYVVwRmo0anAr\neDkza1Rmem5Ba1pQQTBMTFVEUzBHMkd2L1Y4WnBpenI2dGRFL3YyZVk4cGVR\nbVVVZzZNb3NqRVJWbjhyblVFS0pNTUw1aXpscjI0OWtTSTQ2eWk0QzVOL21G\nN2hYT0crN1pSMEVpeXlZdERONnNTZ1QxaUxGTGdrRDN4Lzh0bUo0T3Nua2ZQ\nY1FPL0xrV09hWHJKOCtJWjMyQjRwblZSczd4RW4vb3FiMHp1SG5rR3FSakRJ\nS2pNUUd0RTQzMzg2bjhqM0VBVkVmS0NKa3hPSWZQeVV3dm53MTRjbTVBU2JS\nc3QwZnB3ODhzRkw0UUo0MGtoS1hGSFNzQW14TmltMVdDZFY1aGloQ1daUXJT\ndmVaMW1wVDIxRWZZdHlzZz09LjNXbUx4UDFkT0ZEdmlwblEuK0FMeHNwOXo0\nUlpaR25lZTM5d1kxZz09Iiwic2lnIjoiTW0zckRNZ09SMURYUVc4S3ZJYnI4\nbGpxTGFGTzFleldqOUd3TnArNmNuTExkMXJIc2N4N2xpZXJSOFViUjZYM0Vy\nUDQ0blpxamJkeUcwL3hrOHVEQUE9PSIsImFsZyI6ImFlcy0yNTYtZ2NtK2Vk\nMjU1MTkifQ==\n-----END LICENSE FILE-----\n";
const KEYGEN_LICENSE_KEY: &str = "33362C-D254BA-F54C3C-DAAE48-C71975-V3";
const KEYGEN_PUBLIC_KEY: &str = "e8601e48b69383ba520245fd07971e983d06d22c4257cfd82304601479cee788";

#[derive(Deserialize, Debug)]
struct LicenseFile<'a> {
  enc: &'a str,
  sig: &'a str,
  alg: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
  // Parse the hex-encoded public key.
  let public_key: PublicKey = match <[u8; PUBLIC_KEY_LENGTH]>::from_hex(KEYGEN_PUBLIC_KEY) {
    Ok(bytes) => PublicKey::from_bytes(&bytes)?,
    Err(_) => return Err("failed to parse public key".into()),
  };

  // Extract the encoded payload from the license file.
  let base64 = KEYGEN_LICENSE_FILE
    .replace("-----BEGIN LICENSE FILE-----", "")
    .replace("-----END LICENSE FILE-----", "")
    .replace('\n', "");

  // Decode the payload.
  let payload = match base64::decode(base64) {
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
  println!("  > {:?}", lic);

  // Hash the license key to obtain decryption key.
  let mut sha = Sha256::new();

  sha.update(KEYGEN_LICENSE_KEY.as_bytes());

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
  println!("license file was successfully decrypted!");
  println!("  > {}", plaintext);

  Ok(())
}
