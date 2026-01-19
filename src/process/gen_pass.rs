use rand::seq::SliceRandom;
const UPPER_CASE: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWER_CASE: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";
pub fn gen_pass(
    length: u8,
    upper_case: bool,
    lower_case: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut pass = Vec::new();
    let mut chars = Vec::new();
    if upper_case {
        chars.extend_from_slice(UPPER_CASE);
        pass.push(*UPPER_CASE.choose(&mut rng).unwrap());
    }
    if lower_case {
        chars.extend_from_slice(LOWER_CASE);
        pass.push(*LOWER_CASE.choose(&mut rng).unwrap());
    }
    if number {
        chars.extend_from_slice(NUMBER);
        pass.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        pass.push(*SYMBOL.choose(&mut rng).unwrap());
    }
    for _ in 0..(length - pass.len() as u8) {
        pass.push(*chars.choose(&mut rng).unwrap());
    }
    pass.shuffle(&mut rng);
    let pass = String::from_utf8(pass).expect("Failed to convert Vec<u8> to String");

    Ok(pass)
}
