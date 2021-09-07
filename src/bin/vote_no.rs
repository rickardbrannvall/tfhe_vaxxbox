#![allow(non_snake_case)]
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {

    let encfile = "box.enc";

    let _abstain: Vec<f64> = vec![0., 0., 1.];
    let _affirm: Vec<f64> = vec![0., 1., 0.];
    let _vote_no: Vec<f64> = vec![1., 0., 0.];
        
    let mut c0 = VectorLWE::load(&encfile).unwrap();

    //println!("vote");
    c0.add_constant_static_encoder_inplace(&_vote_no)?; 
    //c0.pp();   

    //println!("save...");
    c0.save(&encfile).unwrap();
    
    Ok(())
}
