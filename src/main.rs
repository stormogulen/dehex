//
// main.rs
//

extern crate  pretty_hex;

fn main() {

    use pretty_hex::*;
    let v = vec![222, 173, 190, 239, 202, 254, 32, 24];
    assert_eq!(pretty_hex(&v), format!("{:?}", v.hex_dump()));

    println!("{:?}", v.hex_dump());


}
