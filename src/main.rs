use getrandom::{SysRng, rand_core::UnwrapErr};
use rand_core::TryRng;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;
fn main() {
    let mut csprng = UnwrapErr(SysRng);

    let signing_key = SigningKey::generate(&mut csprng);

    println!("{:?}:", signing_key);
}
