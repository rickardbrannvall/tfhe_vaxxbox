#![allow(non_snake_case)]
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    
    // place keys in common directory
    let path = "keys";
    
    println!("Creating LWE key ...");
    
    let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    //let sk0_RLWE_path = format!("{}/sk0_RLWE.json",path);
      
    //let sk0_RLWE = RLWESecretKey::new(&RLWE128_1024_1); 
    let sk0_RLWE = RLWESecretKey::new(&RLWE80_1024_1); 
    //let sk0_RLWE = RLWESecretKey::new(&RLWE80_2048_1); 
    //sk0_RLWE.save(&sk0_RLWE_path).unwrap();

    let sk0_LWE = sk0_RLWE.to_lwe_secret_key();
    sk0_LWE.save(&sk0_LWE_path).unwrap();

    /* for this project we only need a single secret key
    
    let sk1_LWE_path = format!("{}/sk1_LWE.json",path);
    let sk1_RLWE_path = format!("{}/sk1_RLWE.json",path);
        
    //let sk1_RLWE = RLWESecretKey::new(&RLWE128_1024_1);
    let sk1_RLWE = RLWESecretKey::new(&RLWE80_1024_1);
    //let sk1_RLWE = RLWESecretKey::new(&RLWE80_2048_1);
    sk1_RLWE.save(&sk1_RLWE_path).unwrap();
    
    let sk1_LWE = sk1_RLWE.to_lwe_secret_key();
    sk1_LWE.save(&sk1_LWE_path).unwrap();

    // generate the key switching key
    
    println!("Creating key switching keys...");

    let ksk01_path = format!("{}/ksk01_LWE.json",path);
    let ksk10_path = format!("{}/ksk10_LWE.json",path);

    let ksk_10 = LWEKSK::new(&sk1_LWE, &sk0_LWE, 2, 6);
    ksk_10.save(&ksk10_path);//.unwrap();

    let ksk_01 = LWEKSK::new(&sk0_LWE, &sk1_LWE, 2, 6);
    ksk_01.save(&ksk01_path);//.unwrap();

    // bootstrapping keys
    
    println!("Creating bootstrap key 01 ...");

    let bsk01_path = format!("{}/bsk01_LWE.json",path);

    let base_log: usize = 5;
    let level: usize = 3;
    let bsk = LWEBSK::new(&sk0_LWE, &sk1_RLWE, base_log, level);
    bsk.save(&bsk01_path);
        
    println!("Creating bootstrap key 10 ...");
    
    let bsk10_path = format!("{}/bsk10_LWE.json",path);
    
    let base_log: usize = 5;
    let level: usize = 3;
    let bsk = LWEBSK::new(&sk1_LWE, &sk0_RLWE, base_log, level);
    bsk.save(&bsk10_path);
    
    */

    Ok(())    
    
}
