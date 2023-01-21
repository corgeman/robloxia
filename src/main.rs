
use aes::cipher::block_padding::UnpadError;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;
use std::time::Instant;
fn main() -> io::Result<()>{
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
    type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;
    type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
    println!("Ciphertext?");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Bad input?");
    let ciphertext = base64::decode(&response.trim()).unwrap();
    println!("Path to wordlist?");
    response = String::new();
    io::stdin().read_line(&mut response).expect("Bad input?");
    let file = File::open(&response.trim())?;
    const IV: [u8; 16] = [49,50,51,52,53,54,55,56,98,48,122,50,51,52,53,110]; // aesencryption.net
    // const IV: [u8; 16] = [53,54,53,54,53,57,105,117,56,103,104,98,118,53,54,55]; // devglan
    println!("Lastly, keysize? Respond with 128, 192 or 256. If you don't know, respond with 911 to try all three.");
    response = String::new();
    io::stdin().read_line(&mut response).expect("Bad input?");
    let mut keysize: usize = response.trim().parse().expect("Please input just a number next time");
    if keysize == 911{all(&file, ciphertext.clone());}
    keysize /= 8;
    let reader = BufReader::new(file);
    let mut buf: Vec<u8> = vec![0u8;ciphertext.len()];
    let now = Instant::now();
    for line in reader.lines() {
    let key: &[u8] = &fix(line.unwrap(),keysize);
    // let key: &[u8] = &fix(line.unwrap(),16); // devglan
    let decrypted = match keysize{
        16 => Aes128CbcDec::new(key.into(), &IV.into()).decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf),
        24 => Aes192CbcDec::new(key.into(), &IV.into()).decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf),
        32 => Aes256CbcDec::new(key.into(), &IV.into()).decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf),
        _ => panic!("Incorrect keysize."),
    };
    // let decrypted = Aes128CbcDec::new(key.into(), &IV.into()).decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf); // devglan
    if let Ok(_v) = decrypted{
        if let Ok(p) = std::str::from_utf8(decrypted.unwrap()){println!("{}:{}",std::str::from_utf8(key).unwrap().trim_end_matches(char::from(0)),p);}
        // if let Ok(p) = std::str::from_utf8(decrypted.unwrap()){println!("{}",p);}

    }
    }
    let elapsed = now.elapsed();
    println!("Time taken: {:.2?}",elapsed);
    Ok(())
}
fn fix(attempt: String,ks:usize) -> Vec<u8>{
    let mut key: Vec<u8> = attempt.into_bytes();
    key.resize(ks, 0); //aesencryption
    // key.resize(ks,116); //devglan
    key
}
fn all(f: &File, ciphertext: Vec<u8>){
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
    type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;
    type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
    const IV: [u8; 16] = [49,50,51,52,53,54,55,56,98,48,122,50,51,52,53,110];
    let reader = BufReader::new(f);
    let mut buf = (vec![0u8;ciphertext.len()],vec![0u8;ciphertext.len()],vec![0u8;ciphertext.len()]);
    let now = Instant::now();
    for line in reader.lines() { 
        let keyz = fixall(line.unwrap());
        let keys: [&[u8];3] = [&keyz[0],&keyz[1],&keyz[2]];
        let resultz: [Result<&[u8], UnpadError>;3] = [
        Aes128CbcDec::new(keys[0].into(), &IV.into()).decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf.0),
        Aes192CbcDec::new(keys[1].into(), &IV.into()).decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf.1),
        Aes256CbcDec::new(keys[2].into(), &IV.into()).decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf.2)
        ];
        let mut x: u8 = 8;
        for decrypted in resultz{
            x += 8;
            if let Ok(_v) = decrypted{
                if let Ok(p) = std::str::from_utf8(decrypted.unwrap()){println!("AES-{}:{}:{}",x*8,std::str::from_utf8(keys[0]).unwrap().trim_end_matches(char::from(0)),p);}
            }
        }

    }
    let elapsed = now.elapsed();
    println!("Time taken: {:.2?}",elapsed);
    process::exit(0);
}
fn fixall(attempt: String) -> [Vec<u8>; 3]{
    let mut keys = [(); 3].map(|_| attempt.as_bytes().to_vec());
    for _num in 0..3{
        keys[_num].resize((_num+2)*8,0);
    }
    keys

}
