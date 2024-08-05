mod t_rsa;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

use t_rsa::g;


fn my_hash(obj: &str) -> Digest
{
    let mut context = Context::new(&SHA256);
    context.update(obj.as_bytes());
    context.finish()
}

fn main() {
    println!("{:?}", HEXUPPER.encode(my_hash("person1").as_ref()));
    println!("{:?}", my_hash("person2"));


    g();
}
