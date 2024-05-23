
use openssl::rsa::Rsa;

fn
main()
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <private_key_file> <public_key_file>", args[0]);
        std::process::exit(1);
    }

    let rsa = Rsa::generate(2048).unwrap();

    std::fs::write(&args[1], &rsa.private_key_to_pem().unwrap()).unwrap();
    std::fs::write(&args[2], &rsa.public_key_to_pem().unwrap()).unwrap();
}
