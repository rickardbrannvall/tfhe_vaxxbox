#![allow(non_snake_case)]
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {

    let path = "keys";
    let encfile = "box.enc";
    
    println!("loading LWE key... \n");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    
    // create an encoder
    let enc = Encoder::new(0., 63., 8, 1)?;

    let m0: Vec<f64> = vec![0., 0., 0.];
    println!("create zero vector {:?}\n", m0);
    
    let c0 = VectorLWE::encode_encrypt(&sk0, &m0, &enc)?;  
    //println!("encrypted zero {:?}", c0.decrypt_decode(&sk0).unwrap());
    //c0.pp();

    c0.save(&encfile).unwrap();
    
    Ok(())
}
