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

    let key_algorithm = AsymmetricEncryption::Rsa(KeyBits::Bits1024);
    let hash = Hash::Sha2(Sha2Bits::Sha224);
    let config: SecureEnclaveConfig = SecureEnclaveConfig::new(Some(key_algorithm), Some(hash));
    let app_ui = AppWindow::new()?;
    let mut ui = app_ui.as_weak().unwrap();

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
        ui.set_input(string.clone());
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
    });
    let tpm_provider2 = tpm_provider.clone();
    ui = app_ui.as_weak().unwrap();
    app_ui.on_sign(move |string| {
        ui.set_signValue(string.clone());
        let data = string.as_bytes();
        match tpm_provider2.lock().unwrap().sign_data(data) {
            Ok(signature) => {
                let signed_data_bytes = String::from_utf8(signature.clone()); 
                ui.set_verify(format!("{}",signed_data_bytes.expect("REASON")).into()); 
                println!("Signature of '{}' => \n{:?}", string, ui.get_verify());}
            Err(e) => println!("Failed to sign data: {:?}", e),
        }; 
        println!("{}",string.trim());
    });
    ui = app_ui.as_weak().unwrap();
    let tpm_provider2 = tpm_provider.clone();
    app_ui.on_verifyData(move |string, string2| {
        match tpm_provider2.lock().unwrap().verify_signature(string2.as_bytes(), &string.as_bytes()) {
            Ok(valid) => {
                if valid {
                    ui.set_status(format!("true").into());
                } else {
                    ui.set_status(format!("false").into());
                    println!("{:?} UND {:?}",string2.as_bytes(), &string.as_bytes());
                }
            }
            Err(e) => println!("Failed to verify signature: {:?}", e),
        }
    });
    app_ui.run()
}