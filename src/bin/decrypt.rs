#![allow(non_snake_case)]
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {

    let path = "keys";
    let encfile = "box.enc";
    
    println!("loading LWE key... \n");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    
    let c0 = VectorLWE::load(&encfile).unwrap();
    //let m0 = c0.decrypt_decode(&sk0).unwrap();
    let m0 = c0.decrypt_decode_round(&sk0).unwrap();

    //let mut file = File::create(&txtfile).unwrap();
    //let fib_str = f64::to_string(&decrypted[0]);
    //let fib_str = f64::to_string(&decrypted);
    //file.write_all(fib_str.as_bytes()).unwrap();

    println!("decrypted results: {:?}", m0);
    
    Ok(())
}
