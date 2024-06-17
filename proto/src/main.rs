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
    let tpm_provider = SecModules::get_instance(key_id.to_string(), SecurityModule::Tpm(TpmType::MacOs), Some(logger))
    .expect("Failed to create TPM provider"); 

    let bla = [68, 102, 121, 117, 119, 47, 116, 100, 65, 72, 65, 110, 50, 101, 122, 73, 108, 111, 54, 81, 47, 117, 47, 43, 65, 52, 75, 107, 54, 73, 67, 113, 83, 116, 71, 52, 87, 105, 50, 55, 99, 122, 100, 73, 69, 84, 66, 83, 111, 76, 78, 97, 97, 50, 53, 90, 113, 73, 43, 108, 89, 67, 81, 104, 117, 105, 54, 104, 73, 75, 98, 54, 115, 52, 109, 54, 80, 71, 104, 66, 66, 90, 115, 86, 116, 74, 103, 75, 106, 98, 112, 84, 79, 49, 122, 114, 108, 48, 65, 78, 54, 55, 120, 118, 107, 78, 110, 67, 82, 112, 68, 82, 77, 79, 117, 119, 107, 116, 98, 99, 76, 78, 108, 57, 105, 74, 114, 109, 51, 55, 98, 68, 43, 119, 90, 100, 99, 77, 75, 104, 120, 105, 116, 55, 90, 83, 72, 56, 70, 111, 112, 118, 78, 65, 47, 73, 82, 68, 43, 53, 107, 78, 66, 89, 112, 50, 116, 121, 97, 116, 69, 61];

    let key_algorithm = AsymmetricEncryption::Rsa(KeyBits::Bits1024);
    let hash = Hash::Sha2(Sha2Bits::Sha224);
    let config: SecureEnclaveConfig = SecureEnclaveConfig::new(Some(key_algorithm), Some(hash));
    let mut encrypted_data_bytes: Vec<u8> = Vec::new();
    let app_ui = AppWindow::new()?;
    let mut ui = app_ui.as_weak().unwrap();

    // ui = app_ui.as_weak().unwrap();
    match tpm_provider.lock().unwrap().create_key(key_id, Box::new(config.clone())) {
        Ok(()) => {ui.set_statusKey("Key created successful".into()); println!("Key created successful");},
        Err(e) => {ui.set_statusKey(format!("Failed to create key: {:?}", e).into()); println!("{}",format!("Failed to create key: {:?}", e));},
    }

    // Load Key
    match tpm_provider.lock().unwrap().load_key(key_id, Box::new(config.clone())) {
        Ok(()) => println!("Key existing and ready for operations"),
        Err(e) => println!("Failed to load Key: {:?}", e),
    }
    let tpm_provider2 = tpm_provider.clone();
    app_ui.on_encrypt(move |string| {
        match tpm_provider2.lock().unwrap().encrypt_data(string.as_bytes()) {
            Ok(encrypted_data) => {
                let tmp = String::from_utf8(encrypted_data);
                ui.set_encryptedValue(format!("{}",tmp.expect("REASON")).into());
                println!("{}", ui.get_encryptedValue());
            }
            Err(e) => println!("Failed to encrypt data: {:?}", e),
        }
    });
    let tpm_provider2 = tpm_provider.clone();
    ui = app_ui.as_weak().unwrap();
    app_ui.on_decrypt(move |string| {
        println!("{}",string);
        match tpm_provider2.lock().unwrap().decrypt_data(&string.as_bytes()) {
            Ok(decrypted_data) => {let tmp = String::from_utf8(decrypted_data); 
                ui.set_decryptedValue(format!("{}",tmp.expect("REASON")).into()); 
                println!("{}",ui.get_decryptedValue());},
            Err(e) => println!("Failed to decrypt data: {:?}", e),
        }
        // println!("{}",string.trim());
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

    
    app_ui.run()
}