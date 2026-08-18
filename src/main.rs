use ed25519_dalek::Signer;
use ed25519_dalek::ed25519::signature;
use getrandom::{SysRng, rand_core::UnwrapErr};
use rand_core::TryRng;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;

fn main() {
    let mut csprng = UnwrapErr(SysRng);

    let signing_key = SigningKey::generate(&mut csprng);

    let message = b"hello world!";

    let signature = signing_key.sign(message);
    
    let verifying_key = signing_key.verifying_key();

    println!("{:?}", signature);

    println!("{:?}", verifying_key);

    
}
