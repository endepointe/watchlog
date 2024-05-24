
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

mod generate_tests {
    #[test]
    fn
    test_file_exists()
    {
        let file = std::fs::OpenOptions"private_key.pem";
        assert!(std::fs::metadata("private_key.pem").is_err());
        assert!(std::fs::metadata("public_key.pem").is_err());

        super::main();

        assert!(std::fs::metadata("private_key.pem").is_ok());
        assert!(std::fs::metadata("public_key.pem").is_ok());
    }
}
