use base64::engine::general_purpose;
use base64::Engine;
use rand::rngs::OsRng;
use rand::Rng;

pub fn generate_unique_string() -> String {
    let mut rng = OsRng;
    let mut buffer = [0u8; 32];
    rng.fill(&mut buffer[..]);
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(buffer);
    encoded
}
