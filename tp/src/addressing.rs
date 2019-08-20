use crypto::digest::Digest;
use crypto::sha2::Sha512;

const FAMILY_NAME: &str = "trade";
const AGENT: &str = "00";

pub fn hash(to_hash: &str, num: usize) -> String {
    let mut sha = Sha512::new();
    sha.input_str(to_hash);
    let temp = sha.result_str().to_string();
    let hash = match temp.get(..num) {
        Some(x) => x,
        None => "",
    };
    hash.to_string()
}

pub fn get_supply_chain_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str(&FAMILY_NAME);
    sha.result_str()[..6].to_string()
}

pub fn make_agent_address(identifier: &str) -> String {
    get_supply_chain_prefix() + &AGENT + &hash(identifier, 62)
}