#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::create_md5;
    fn create_md5_test() -> String {
        let opt = create_md5::create_md5::Opt{
            input: PathBuf::from("src/main.rs"),
            output: "l".to_string(),
            t: "str".to_string(),
        };
        let res = create_md5::create_md5::create_md5(opt).unwrap();
        return res
    }

    #[test]
    fn it_works() {
        assert_eq!(create_md5_test(), "639fbc4ef05b315af92b4d836c31b023".to_string());
    }
}

pub mod encrypt {
    use structopt::StructOpt;
    use exitfailure::ExitFailure;
    use std::path::{Path, PathBuf};
    use std::io::{BufReader, Read};
    use std::fs::File;
    use failure::{Error, ResultExt, ensure};
    use failure::_core::fmt::Debug;
    extern crate crypto;
    use self::crypto::digest::Digest;

    #[derive(Debug, StructOpt)]
    pub enum Command {
        #[structopt(name = "md5")]
        Md5(Opt),

        #[structopt(name = "sha1")]
        Sha1(Opt),

        #[structopt(name = "sha224")]
        Sha224(Opt),

        #[structopt(name = "sha256")]
        Sha256(Opt),

        #[structopt(name = "sha384")]
        Sha384(Opt),

        #[structopt(name = "sha512")]
        Sha512(Opt),

        #[structopt(name = "sha512T224")]
        Sha512Trunc224(Opt),

        #[structopt(name = "sha512T256")]
        Sha512Trunc256(Opt),

        #[structopt(name = "sha3-224")]
        SHA3224(Opt),

        #[structopt(name = "sha3-384")]
        SHA3384(Opt),

        #[structopt(name = "sha3-512")]
        SHA3512(Opt),

        #[structopt(name = "keccak224")]
        Keccak224(Opt),

        #[structopt(name = "keccak256")]
        Keccak256(Opt),

        #[structopt(name = "keccak384")]
        Keccak384(Opt),

        #[structopt(name = "keccak512")]
        Keccak512(Opt),
    }

    #[derive(StructOpt, Debug)]
    pub struct Opt {
        /// Input string or file path
        #[structopt(short, long, parse(from_os_str))]
        pub input: PathBuf,

        /// Input-type t = (str or file)
        #[structopt(short, long, default_value = "str")]
        pub t: String,

        /// Output uppercase or lowercase,  o = (u or l)
        #[structopt(short, long, default_value = "l")]
        pub output: String,
    }

    pub trait Encrypt {
        fn crypt<D: Digest>(&self, hasher: &mut D) -> Result<String, ExitFailure>;
    }

    impl Encrypt for Opt{
        fn crypt<D: Digest>(&self,  hasher: &mut D) -> Result<String, ExitFailure> {
            let mut val: String;
            if &self.t == "str" {
                let input = self.input.to_str().unwrap();
                hasher.input_str(input);
                val = hasher.result_str();
            } else if &self.t == "file" {
                let input = read_file(&self.input)
                    .with_context(|_| format!("could not read file `{:?}`", &self.input))?;
                hasher.input(&input);
                val = hasher.result_str();
            } else {
                panic!("请输入正确的 -t 参数")
            };
            if &self.output == "u" {
                val = val.to_uppercase()
            }
            Ok(val)
        }
    }

    #[derive(Debug, StructOpt)]
    #[structopt(name = "classify")]
    pub struct ApplicationArguments {
        #[structopt(subcommand)]
        pub command: Command,
    }

    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
        let path = path.as_ref();
        ensure!(
        path.exists() && path.is_file(),
        "Path {:?} is not a file!",
        path
    );
        let file = File::open(path).with_context(|_| format!("Could not open file {:?}", path))?;
        let mut file = BufReader::new(file);
        let mut result = Vec::new();
        file.read_to_end(&mut result)
            .with_context(|_| format!("Could not read file {:?}", path))?;

        Ok(result)
    }
}


