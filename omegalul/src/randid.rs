use rand::Rng;

pub type RandID = String;

pub fn generate() -> RandID {
    let mut rng = rand::thread_rng();

    let charset: &[u8] = b"23456789ABCDEFGHJKLMNPQRSTUVWXYZ";

    (0..8)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());

            charset[idx] as char
        })
        .collect()
}
