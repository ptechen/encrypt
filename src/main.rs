use structopt::StructOpt;
use exitfailure::ExitFailure;
pub mod encrypt;
use encrypt::encrypt::{ApplicationArguments, Command, Encrypt};
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::sha2::{Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};
use crypto::sha3::Sha3;

fn main() -> Result<(), ExitFailure> {
    let opt = ApplicationArguments::from_args();
    let key = opt.command;
    let val:String;
    match key {
        Command::Md5(s) => {
            let mut hasher = Md5::new();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Sha1(s) => {
            let mut hasher= Sha1::new();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Sha224(s) => {
            let mut hasher= Sha224::new();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Sha256(s) => {
            let mut hasher= Sha256::new();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Sha384(s) => {
            let mut hasher= Sha384::new();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Sha512(s) => {
            let mut hasher= Sha512::new();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Sha512Trunc224(s) => {
            let mut hasher= Sha512Trunc224::new();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Sha512Trunc256(s) => {
            let mut hasher= Sha512Trunc256::new();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::SHA3224(s) => {
            let mut hasher= Sha3::sha3_224();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::SHA3384(s) => {
            let mut hasher= Sha3::sha3_384();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::SHA3512(s) => {
            let mut hasher= Sha3::sha3_512();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Keccak224(s) => {
            let mut hasher= Sha3::keccak224();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Keccak256(s) => {
            let mut hasher= Sha3::keccak256();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Keccak384(s) => {
            let mut hasher= Sha3::keccak384();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }

        Command::Keccak512(s) => {
            let mut hasher= Sha3::keccak512();
            val = Encrypt::crypt(&s, &mut hasher).unwrap();
        }
    };
    println!("{}", val);
    Ok(())
}
