
struct FieldElement {
    repr,
    eq,
    ne,
    add,
    sub,
    mul,
    pow,
    truediv,
    rmul,
}

struct Point {
    eq,
    ne,
    repr,
    add,
    rmul,
}

struct S256field {

}

struct S256Point {

}

struct Signature {
    repr,
    der,
    parse,
}

struct PrivateKey {
    hex,
    sign,
    deterministic_k
    wif,   
}

impl FieldElement {
    fn repr() {}
    fn eq() {}
    fn ne() {}
    fn add() {}
    fn sub() {}
    fn mul() {}
    fn pow() {}
    fn truediv() {}
    fn rmul() {}
}

impl Point {
    fn eq() {}
    fn ne() {}
    fn repr() {}
    fn add() {}
    fn rmul() {}
}

impl S256Field {}
impl S256Point {}

impl Signature {
    fn repr() {}
    fn der() {}
    fn parse() {}
}

impl PrivateKey {
    fn hex() {}
    fn sign() {}
    fn deterministic_k() {}
    fn wif() {}

}



fn main() {
    println!("Hello, world!");
}
