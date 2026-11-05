use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use argon2::{Algorithm, Argon2, Params, Version};
use base64::Engine;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::path::Path;
use ts_rs::TS;
use zeroize::Zeroizing;

use crate::error::{Error, Result};

pub const KEY_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;
const SALT_LENGTH: usize = 16;
const KDF_MEMORY_KIB: u32 = 64 * 1024;
const KDF_ITERATIONS: u32 = 3;
const KDF_PARALLELISM: u32 = 1;
pub const KEY_FILE: &str = "master.key";
// ! Sealed into every vault: changing it, even for the app's new name, locks
// every existing vault.
const CHECK_PLAINTEXT: &[u8] = b"twitch-drops-points-miner vault check";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum VaultMode {
    KeyFile,
    Passphrase,
}

impl VaultMode {
    pub const ALL: [VaultMode; 2] = [Self::KeyFile, Self::Passphrase];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeyFile => "KEY_FILE",
            Self::Passphrase => "PASSPHRASE",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|mode| mode.as_str() == name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sealed {
    pub nonce: String,
    pub ciphertext: String,
}

#[derive(Clone)]
pub struct Vault {
    key: Zeroizing<[u8; KEY_LENGTH]>,
}

fn b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

fn unb64(value: &str) -> Result<Vec<u8>> {
    base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(|_| Error::InvalidVault)
}

pub fn random_salt() -> String {
    let mut salt = [0u8; SALT_LENGTH];

    rand::rng().fill_bytes(&mut salt);

    b64(&salt)
}

impl Vault {
    pub fn from_passphrase(passphrase: &str, salt_b64: &str) -> Result<Self> {
        let salt = unb64(salt_b64)?;
        let params = Params::new(
            KDF_MEMORY_KIB,
            KDF_ITERATIONS,
            KDF_PARALLELISM,
            Some(KEY_LENGTH),
        )
        .map_err(|_| Error::InvalidVault)?;
        let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut key = Zeroizing::new([0u8; KEY_LENGTH]);

        argon
            .hash_password_into(passphrase.as_bytes(), &salt, key.as_mut())
            .map_err(|_| Error::InvalidVault)?;

        Ok(Self { key })
    }

    pub fn from_key_file(dir: &Path) -> Result<Self> {
        let path = dir.join(KEY_FILE);

        if path.exists() {
            let content = Zeroizing::new(
                std::fs::read_to_string(&path).map_err(|e| Error::Storage(e.to_string()))?,
            );
            let bytes =
                Zeroizing::new(hex::decode(content.trim()).map_err(|_| Error::InvalidVault)?);
            let mut key = Zeroizing::new([0u8; KEY_LENGTH]);

            if bytes.len() != KEY_LENGTH {
                return Err(Error::InvalidVault);
            }

            key.copy_from_slice(&bytes);

            return Ok(Self { key });
        }

        let mut key = Zeroizing::new([0u8; KEY_LENGTH]);

        rand::rng().fill_bytes(key.as_mut());
        write_private(&path, Zeroizing::new(hex::encode(key.as_ref())).as_bytes())?;

        Ok(Self { key })
    }

    pub fn from_raw(key: [u8; KEY_LENGTH]) -> Self {
        Self {
            key: Zeroizing::new(key),
        }
    }

    pub fn seal(&self, plaintext: &[u8]) -> Result<Sealed> {
        let cipher =
            Aes256Gcm::new_from_slice(self.key.as_ref()).map_err(|_| Error::InvalidVault)?;
        let mut nonce = [0u8; NONCE_LENGTH];

        rand::rng().fill_bytes(&mut nonce);

        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext)
            .map_err(|_| Error::InvalidVault)?;

        Ok(Sealed {
            nonce: b64(&nonce),
            ciphertext: b64(&ciphertext),
        })
    }

    pub fn open(&self, sealed: &Sealed) -> Result<Zeroizing<Vec<u8>>> {
        let nonce = unb64(&sealed.nonce)?;

        if nonce.len() != NONCE_LENGTH {
            return Err(Error::InvalidVault);
        }

        let ciphertext = unb64(&sealed.ciphertext)?;
        let cipher =
            Aes256Gcm::new_from_slice(self.key.as_ref()).map_err(|_| Error::InvalidVault)?;

        cipher
            .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
            .map(Zeroizing::new)
            .map_err(|_| Error::WrongPassphrase)
    }

    pub fn check_blob(&self) -> Result<Sealed> {
        self.seal(CHECK_PLAINTEXT)
    }

    pub fn verify(&self, check: &Sealed) -> Result<()> {
        let plain = self.open(check)?;

        if plain.as_slice() == CHECK_PLAINTEXT {
            Ok(())
        } else {
            Err(Error::WrongPassphrase)
        }
    }
}

// ! Overwritten before it goes, so the old key does not stay readable on the
// disk; best effort: a copy-on-write or flash disk may keep it anyway.
pub fn remove_key_file(dir: &Path) {
    use std::io::Write;

    let path = dir.join(KEY_FILE);

    if let Ok(mut file) = std::fs::OpenOptions::new().write(true).open(&path) {
        let length = file.metadata().map_or(0, |metadata| metadata.len());

        let _ = file.write_all(&vec![0u8; usize::try_from(length).unwrap_or(0)]);
        let _ = file.sync_all();
    }

    let _ = std::fs::remove_file(&path);
}

// ! On the disk before the database names this key: a crash right after the
// vault switches to it must not lose it.
fn write_private(path: &Path, content: &[u8]) -> Result<()> {
    use std::io::Write;

    let mut options = std::fs::OpenOptions::new();

    options.write(true).create_new(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(0o600);
    }

    let mut file = options
        .open(path)
        .map_err(|e| Error::Storage(e.to_string()))?;

    file.write_all(content)
        .and_then(|()| file.sync_all())
        .map_err(|e| Error::Storage(e.to_string()))?;

    if let Some(dir) = path.parent() {
        let _ = std::fs::File::open(dir).and_then(|dir| dir.sync_all());
    }

    Ok(())
}
