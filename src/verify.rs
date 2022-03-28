use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Verifier;

pub fn verify(data: &[u8], signature: String) -> bool {
    let keypair = Rsa::public_key_from_pem(&std::fs::read(".pubkey.pem").unwrap()).unwrap();
    let keypair = PKey::from_rsa(keypair).unwrap();

    // Verify the data
    let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
    verifier.update(data).unwrap();
    
    verifier
        .verify(&base64::decode(signature).unwrap())
        .unwrap()
}
