use ed25519_dalek::Signer;
use ed25519_dalek::Verifier;
use ed25519_dalek::ed25519::signature;
use getrandom::{SysRng, rand_core::UnwrapErr};
use rand_core::TryRng;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;
use ssh_key::{PublicKey, public::Ed25519PublicKey};
fn main() {
    let mut csprng = UnwrapErr(SysRng);

    // private key 생성
    let signing_key = SigningKey::generate(&mut csprng);

    // public key 생성
    let verifying_key = signing_key.verifying_key();

    // 메시지 
    let message = b"hello world!";
    let fake_msg = b"hello world";

    // 서명 생성
    let signature = signing_key.sign(message);

    // 서명 검증
    verifying_key.verify(message, &signature).unwrap();
    // verifying_key.verify(fake_msg, &signature).unwrap();
    println!("서명 검증 완료");

    // Ed25519 key의 raw 32바이트
    let private_bytes = signing_key.to_bytes();
    let public_bytes = verifying_key.to_bytes();

    println!("private_bytes : {:?},  public_bytes : {:?}", private_bytes, public_bytes);

    // OpenSSH bytes 형식으로 변경
    let public_key_bytes_openssh = Ed25519PublicKey::try_from(public_bytes.as_slice()).unwrap();

    // ssh 형식으로 변경
    let public_key_openssh = PublicKey::from(public_key_bytes_openssh);
    println!("public_key_openssh : {:?}", public_key_openssh);
    
}



