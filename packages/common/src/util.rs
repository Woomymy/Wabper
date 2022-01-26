use rand::Rng;
// All chars allowed in url IG
pub const ID_LEN: i32 = 8; // Lenght of paste ID
pub const ID_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
/// Generated valid paste ID
pub fn gen_id() -> String {
    let mut rng = rand::thread_rng();

    (0..=ID_LEN)
        .map(|_| ID_CHARSET[rng.gen_range(0..ID_CHARSET.len())] as char)
        .collect()
}
