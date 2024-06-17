use crypto_layer::common::crypto::algorithms::encryption::AsymmetricEncryption;
use crypto_layer::common::crypto::algorithms::hashes::{Hash, Sha2Bits};
use crypto_layer::common::crypto::algorithms::KeyBits;
use crypto_layer::common::factory::{SecModules, SecurityModule};
use crypto_layer::tpm::core::instance::TpmType;
use crypto_layer::tpm::macos::logger::Logger;
use crypto_layer::tpm::macos::SecureEnclaveConfig;
use slint::slint;

slint::include_modules!();

fn main() {
    let key_id = "Beispie";
    let logger = Logger::new_boxed();
    let tpm_provider = SecModules::get_instance(
        key_id.to_string(),
        SecurityModule::Tpm(TpmType::MacOs),
        Some(logger),
    )
    .expect("Failed to create TPM provider");

    let key_algorithm = AsymmetricEncryption::Rsa(KeyBits::Bits1024);
    let hash = Hash::Sha2(Sha2Bits::Sha224);
    let config: SecureEnclaveConfig = SecureEnclaveConfig::new(Some(key_algorithm), Some(hash));

    let ui = AppWindow::new();
    ui::setKey("TEST");
    match ui {
        Ok(window) => {
            window.on_request_generate_keys(move || {
                match tpm_provider
                    .lock()
                    .unwrap()
                    .create_key(key_id, Box::new(config.clone()))
                {
                    Ok(()) => ui::setKey("TEST"),
                    Err(e) => println!("Failed to create key: {:?}", e),
                }
            });
            if let Err(e) = window.run() {
                eprintln!("Failed to run the app window: {:?}", e);
            }
        }
        Err(e) => eprintln!("Failed to create app window: {:?}", e),
    }
}
