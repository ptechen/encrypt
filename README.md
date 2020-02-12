# Encrypt Command Line Tool

[![Version info](https://img.shields.io/crates/v/encrypt.svg)](https://crates.io/crates/enrypt)

Support md5 & sha1 & sha2 & sha3 encryption

Install:

    cargo install encrypt

USAGE:

    encrypt <SUBCOMMAND>

FLAGS:

    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:

    help          Prints this message or the help of the given subcommand(s)
    keccak224     
    keccak256     
    keccak384     
    keccak512     
    md5           
    sha1          
    sha224        
    sha256        
    sha3-224      
    sha3-384      
    sha3-512      
    sha384        
    sha512        
    sha512T224    
    sha512T256    


USAGE:

    encrypt func [OPTIONS] --input <input>

FLAGS:

    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:

    -i, --input <input>      Input string or file path
    -o, --output <output>    Output uppercase or lowercase,  o = (u or l) [default: l]
    -t, --t <t>              Input-type t = (str or file) [default: str]
