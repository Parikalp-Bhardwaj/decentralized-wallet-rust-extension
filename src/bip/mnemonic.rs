use bip39::{Language, Mnemonic, MnemonicType, Seed};


pub struct BipMenomic{}

impl BipMenomic{
    pub fn generate_mnemonic() -> Mnemonic{
        Mnemonic::new(MnemonicType::Words12, Language::English)
    }

    pub fn get_mnemonic_to_str(mnemonic: &Mnemonic) -> &str{
        mnemonic.phrase()
    }

    pub fn get_mnemonic_from_phrase(phrase: &str) -> Mnemonic{
        Mnemonic::from_phrase(phrase, Language::English).expect("Failed to get mnemonic from phrase")
    }

    pub fn derive_seed(mnemonic: &Mnemonic, password: &str) -> Seed{
        Seed::new(mnemonic, password)
    }
}

