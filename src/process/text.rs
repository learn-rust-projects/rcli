use std::{collections::HashMap, io::Read};

use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{cli::TextSignFormat, process::gen_pass};
trait TextSigner {
    fn sign(&self, input: &mut dyn Read) -> Result<Vec<u8>>;
}
trait TextVerifier {
    fn verify(&self, input: &mut dyn Read, signature: &[u8]) -> Result<bool>;
}
struct Blake3 {
    pub key: [u8; 32],
}
impl Blake3 {
    pub fn new(key: &[u8; 32]) -> Self {
        Self { key: *key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = key.try_into().unwrap();
        Ok(Self::new(&key))
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = gen_pass(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, input: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes().to_vec())
    }
}
impl TextVerifier for Blake3 {
    fn verify(&self, input: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes() == signature)
    }
}
pub struct Ed25519Signer {
    pub key: SigningKey,
}
pub struct Ed25519Verifier {
    pub key: VerifyingKey,
}
impl TextSigner for Ed25519Signer {
    fn sign(&self, input: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        Ok(self.key.sign(&buf).to_bytes().to_vec())
    }
}
impl TextVerifier for Ed25519Verifier {
    fn verify(&self, input: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let signature = Signature::from_bytes(signature.try_into()?);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}
pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> anyhow::Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };
    signer.sign(reader)
}
pub fn process_text_key_generate(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

pub fn process_text_verify(
    input: &mut dyn Read,
    key: &[u8],
    sign: &[u8],
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    verifier.verify(input, sign)
}

impl Ed25519Signer {
    pub fn new(key: &[u8; 32]) -> Self {
        Self {
            key: SigningKey::from_bytes(key),
        }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = key.try_into().unwrap();
        Ok(Self::new(key))
    }
    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());

        Ok(map)
    }
}

impl Ed25519Verifier {
    pub fn new(key: &[u8; 32]) -> Result<Self> {
        Ok(Self {
            key: VerifyingKey::from_bytes(key)?,
        })
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = key.try_into().unwrap();
        Self::new(&key)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

    use super::*;
    use crate::trim_whitespace;
    static KEY_LAZYLOCK: LazyLock<&[u8]> =
        LazyLock::new(|| trim_whitespace(include_bytes!("../../fixtures/blake3.txt")));
    static KEY_ED25519: LazyLock<&[u8]> =
        LazyLock::new(|| trim_whitespace(include_bytes!("../../fixtures/ed25519.sk")));
    static KEY_ED25519_PK: LazyLock<&[u8]> =
        LazyLock::new(|| trim_whitespace(include_bytes!("../../fixtures/ed25519.pk")));
    #[test]
    fn test_process_text_sign() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = process_text_sign(&mut reader, &KEY_LAZYLOCK, format)?;
        let ret = process_text_verify(&mut reader1, &KEY_LAZYLOCK, &sig, format)?;
        assert!(ret);
        Ok(())
    }
    #[test]
    fn test_process_text_verify() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = "33Ypo4rveYpWmJKAiGnnse-wHQhMVujjmcVkV4Tl43k";
        let sig = URL_SAFE_NO_PAD.decode(sig)?;
        let ret = process_text_verify(&mut reader, &KEY_LAZYLOCK, &sig, format)?;
        assert!(ret);
        Ok(())
    }
    #[test]
    fn test_process_text_sign_verify() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();
        let format = TextSignFormat::Ed25519;
        let sig = process_text_sign(&mut reader, &KEY_ED25519, format)?;
        println!("ed25519 sig: {}", URL_SAFE_NO_PAD.encode(&sig));
        let ret = process_text_verify(&mut reader1, &KEY_ED25519_PK, &sig, format)?;
        assert!(ret);
        Ok(())
    }
}
