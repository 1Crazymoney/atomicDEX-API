use base58::ToBase58;
use bip39::Language;
use ed25519_dalek_bip32::derivation_path::DerivationPath;
use ed25519_dalek_bip32::ExtendedSecretKey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use std::str::FromStr;

fn generate_key_pair_from_seed(seed: String) -> Keypair {
    let derivation_path = DerivationPath::from_str("m/44'/501'/0'").unwrap();
    let mnemonic = bip39::Mnemonic::from_phrase(seed.as_str(), Language::English).unwrap();
    let seed = bip39::Seed::new(&mnemonic, "");
    let seed_bytes: &[u8] = seed.as_bytes();

    let ext = ExtendedSecretKey::from_seed(seed_bytes)
        .unwrap()
        .derive(&derivation_path)
        .unwrap();
    let ref priv_key = ext.secret_key;
    let pub_key = ext.public_key();
    let pair = ed25519_dalek::Keypair {
        secret: ext.secret_key,
        public: pub_key,
    };

    solana_sdk::signature::keypair_from_seed(pair.to_bytes().as_ref()).unwrap()
}

mod tests {
    use super::*;

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn solana_prerequisites() {
        // same test as trustwallet
        {
            let fin = generate_key_pair_from_seed(
                "shoot island position soft burden budget tooth cruel issue economy destroy above".to_string(),
            );
            let public_address = fin.pubkey().to_string();
            let priv_key = &fin.secret().to_bytes()[..].to_base58();
            assert_eq!(public_address.len(), 44);
            assert_eq!(public_address, "2bUBiBNZyD29gP1oV6de7nxowMLoDBtopMMTGgMvjG5m");
            assert_eq!(priv_key, "F6czu7fdefbsCDH52JesQrBSJS5Sz25AkPLWFf8zUWhm");
            let client = solana_client::rpc_client::RpcClient::new("https://api.testnet.solana.com/".parse().unwrap());
            let balance = client.get_balance(&fin.pubkey()).expect("Expect to retrieve balance");
            assert_eq!(balance, 0);
        }

        {
            let fin = generate_key_pair_from_seed(
                "powder verify clutch illegal spider old grain curve robust fade twice sphere".to_string(),
            );
            let public_address = fin.pubkey().to_string();
            assert_eq!(public_address.len(), 44);
            assert_eq!(public_address, "DJ8wwseey5LEoMeMWb3tLDLywK8SecyYcqdzoVw24QpP");
            let client = solana_client::rpc_client::RpcClient::new("https://api.testnet.solana.com/".parse().unwrap());
            let balance = client.get_balance(&fin.pubkey()).expect("Expect to retrieve balance");
            assert_eq!(solana_sdk::native_token::lamports_to_sol(balance), 1.0);
            assert_eq!(balance, 1000000000);
        }
    }
}
