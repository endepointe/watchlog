
use openssl::encrypt::{Decrypter};
use openssl::rsa::Rsa;
use openssl::rsa::Padding;

use std::io::Read;
use std::io::Write;

fn 
main()
{
    let private_key = std::fs::read("private.pem").unwrap();
    let pkey = Rsa::private_key_from_pem(&private_key).unwrap();

    println!("{:?}",std::env::args().collect::<Vec<String>>());  

    let path = std::env::args().collect::<Vec<String>>()[1].clone();
    println!("{:?}", path);

    let mut contents: Vec<u8> = std::fs::read(&path).unwrap();
    println!("{:?}", contents);

    //let decompressed = zstd::stream::decode_all(&*contents).unwrap();
    //println!("{:?}", decompressed);

    //let mut buf = vec![0; pkey.size() as usize];
    //let dec_len = pkey.private_decrypt(, &mut buf, Padding::PKCS1).unwrap();

    //println!("{:?}", &buf[..dec_len]);
}
