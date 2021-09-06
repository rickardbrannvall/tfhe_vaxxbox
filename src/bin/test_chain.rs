#![allow(non_snake_case)]
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {

    let path = "keys";
    
    println!("loading LWE key... \n");
    let sk0_LWE_path = format!("{}/sk0_LWE.json",path);
    let sk0 = LWESecretKey::load(&sk0_LWE_path).unwrap();    
    
    // create an encoder
    let enc = Encoder::new(0., 63., 8, 1)?;

    let m0: Vec<f64> = vec![0., 0.];
    println!("plaintext value {:?}\n", m0);
    
    let mut c0 = VectorLWE::encode_encrypt(&sk0, &m0, &enc)?;  
    println!("encrypted value {:?}", c0.decrypt_decode(&sk0).unwrap());
    c0.pp();
    
    let abstain: Vec<f64> = vec![0., 0.];
    let affirm: Vec<f64> = vec![0., 1.];
    let vote_no: Vec<f64> = vec![1., 0.];
    
    for _i in 0..1 {
        
        c0.add_constant_static_encoder_inplace(&affirm)?; 
        println!("after adding yes vote {:?}", c0.decrypt_decode(&sk0).unwrap());
        c0.pp();   

        c0.add_constant_static_encoder_inplace(&vote_no)?; 
        println!("after adding no vote {:?}", c0.decrypt_decode(&sk0).unwrap());
        c0.pp();     

        c0.add_constant_static_encoder_inplace(&abstain)?; 
        println!("after person abstained {:?}", c0.decrypt_decode(&sk0).unwrap());
        c0.pp();         
    }
    
    Ok(())
}
