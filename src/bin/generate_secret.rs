use rand::RngCore;
fn main() {
    let mut secret = [0u8; 32];
    rand::rng().fill_bytes(&mut secret);
    println!("{}", hex::encode(secret));
}
