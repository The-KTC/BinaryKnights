use crypto_layer::common::crypto::algorithms::encryption::AsymmetricEncryption;
use crypto_layer::common::crypto::algorithms::hashes::{Hash, Sha2Bits};
use crypto_layer::common::crypto::algorithms::KeyBits;
use crypto_layer::common::factory::{SecModules, SecurityModule};
use crypto_layer::tpm::core::instance::TpmType;
use crypto_layer::tpm::macos::logger::Logger;
use crypto_layer::tpm::macos::SecureEnclaveConfig;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
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
    
    let app_ui = AppWindow::new()?;
    let mut ui = app_ui.as_weak().unwrap();

    app_ui.on_encrypt(move |string| {
        println!("{}",string.trim());
    });
    app_ui.on_decrypt(move |string| {
        println!("{}",string.trim());
    });
    app_ui.on_sign(move |string| {
        println!("{}",string.trim());
    });
    app_ui.on_verifyData(move |string| {
        println!("{}",string.trim());
    });
    app_ui.on_reset(move || {
        println!("RESET LoL");
    });

    // ui = app_ui.as_weak().unwrap();
    match tpm_provider
        .lock()
        .unwrap()
        .create_key(key_id, Box::new(config.clone()))
    {
        Ok(()) => {ui.set_statusKey("Key created successful".into()); println!("Key created successful");},
        Err(e) => {ui.set_statusKey(format!("Failed to create key: {:?}", e).into()); println!("{}",format!("Failed to create key: {:?}", e));},
    }
    app_ui.run()
}
