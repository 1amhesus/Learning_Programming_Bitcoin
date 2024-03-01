

/*******************************************************************************************************************************
* Compared to Jimmy's Python code(class FieldElement, ecc.py), what the difference is that the concept of 'magnitude'
* by referring to the following file : https://github.com/bitcoin-core/secp256k1/blob/master/src/field.h
* In the context of elliptic curve cryptography and field arithmetic, "magnitude" refers to the size or order of a field element. 
* It represents the number of bits required to represent the element in binary form.
********************************************************************************************************************************/


/* The use of 'magnitude', 'normalized' in the SECP256K1_FE_VERIFY_FIELDS macro appears to be necessary to track the size of the corresponding 
*  field element, allowing the program to easily check the size of the field element and validate it.
*
*  When the rust code is translated in response to the object-oriented concept of the field element class from Jimmy Song's Python code as follows, 
*  the method code line can be reduced through the traces function that can implement 'polymorphism' in Rust as follows.
*  
*  Detailed method modifications are as follows.
*  The __init__ method initializes a structure, which can be initialized by invoking a new function or other static method when creating a structure.
*  The __repr__ method works similar to Rust's Debug trait, which allows you to output structures for debug purposes using println!, {:?} macro.
*  The __eq__ method can be compared by implementing the PartialEq trait of Rust, which allows you to compare whether two values are equivalent.
*  The __ne___ method is automatically provided when implementing the PartialEq trait.
*
*/

use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
struct FieldElement {
    value: i32,
    magnitude: i32,
    normalized: bool,
}

/* In Jimmy Song's book, he helped with a mathematical understanding of prime parameters by explaining the concept of order, 
*  but this code was written instead using magnitude, which is more useful from a programming perspective. 
*  
*  The secp256k1_fe structure is declared in the field.h file of the secp256k1 library and contains definitions for the FieldElement     
*/
impl FieldElement {
    fn new(value: i32, magnitude: i32) -> Result<Self, &'static str> {
        // Check if the magnitude and value are valid
        if magnitude <= 0 || value < 0 || value >= magnitude {
            return Err("Invalid magnitude or value");
        }
        //Sets the normalized field, which is set to True only if the magnetude is less than 1.
        let normalized = magnitude <= 1;
        // Create and return a new FieldElement object
        Ok(FieldElement {
            value,
            magnitude,
            normalized,
        })
    }

    fn add(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
        if self.magnitude != other.magnitude {
            return Err("Cannot add two numbers in different Fields");
        }
        let new_value = (self.value + other.value) % self.magnitude;
        let new_element = FieldElement::new(new_value, self.magnitude)?;

        // Ensure that the magnitude of the new element is valid
        if new_element.magnitude != self.magnitude {
            return Err("Invalid magnitude of result FieldElement");
        }

        // Ensure that the normalization of the new element is valid
        let expected_normalized: bool = new_element.magnitude <= 1;
        if new_element.normalized != expected_normalized {
            return Err("Invalid normalization of result FieldElement");
        }

        Ok(new_element)
    }

    fn sub(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
        // Ensure that magnitudes of both FieldElement objects are equal
        if self.magnitude != other.magnitude {
            return Err("Cannot subtract two numbers in different Fields");
        }

        // Perform subtraction operation
        let new_value = (self.value - other.value) % self.magnitude;
        let new_value = if new_value < 0 { new_value + self.magnitude } else { new_value };

        // Create a new FieldElement object with the result value
        let new_element = FieldElement::new(new_value, self.magnitude)?;

        // Ensure that the magnitude of the new element is valid
        if new_element.magnitude != self.magnitude {
            return Err("Invalid magnitude of result FieldElement");
        }

        // Ensure that the normalization of the new element is valid
        let expected_normalized = new_element.magnitude <= 1;
        if new_element.normalized != expected_normalized {
            return Err("Invalid normalization of result FieldElement");
        }

        Ok(new_element)
    }

    fn mul(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
        // Ensure that magnitudes of both FieldElement objects are equal
        if self.magnitude != other.magnitude {
            return Err("Cannot multiply two numbers in different Fields");
        }

        // Perform multiplication operation
        let new_value = (self.value * other.value) % self.magnitude;

        // Create a new FieldElement object with the result value
        let new_element = FieldElement::new(new_value, self.magnitude)?;

        // Ensure that the magnitude of the new element is valid
        if new_element.magnitude != self.magnitude {
            return Err("Invalid magnitude of result FieldElement");
        }

        // Ensure that the normalization of the new element is valid
        let expected_normalized = new_element.magnitude <= 1;
        if new_element.normalized != expected_normalized {
            return Err("Invalid normalization of result FieldElement");
        }

        Ok(new_element)
    }

    fn pow(&self, exp: i32) -> Result<FieldElement, &'static str> {
        // Ensure that the exponent is non-negative
        if exp < 0 {
            return Err("Exponent must be non-negative");
        }

        // Perform exponentiation operation
        let new_value = self.value.pow(exp as u32) % self.magnitude;

        // Create a new FieldElement object with the result value
        let new_element = FieldElement::new(new_value, self.magnitude)?;

        // Ensure that the magnitude of the new element is valid
        if new_element.magnitude != self.magnitude {
            return Err("Invalid magnitude of result FieldElement");
        }

        // Ensure that the normalization of the new element is valid
        let expected_normalized = new_element.magnitude <= 1;
        if new_element.normalized != expected_normalized {
            return Err("Invalid normalization of result FieldElement");
        }

        Ok(new_element)
    }

    fn truediv(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
        // Ensure that the magnitudes of both FieldElement objects are equal
        if self.magnitude != other.magnitude {
            return Err("Cannot divide two numbers in different Fields");
        }

        // Perform finite-body division operation using Fermat's predetermined value
        let num = (self.value * mod_inverse(other.value, self.magnitude)) % self.magnitude;

        // Create a new FieldElement object with the result value
        let new_element = FieldElement::new(num, self.magnitude)?;

        // Ensure that the magnitude of the new element is valid
        if new_element.magnitude != self.magnitude {
            return Err("Invalid magnitude of result FieldElement");
        }

        // Ensure that the normalization of the new element is valid
        let expected_normalized = new_element.magnitude <= 1;
        if new_element.normalized != expected_normalized {
            return Err("Invalid normalization of result FieldElement");
        }

        Ok(new_element)
    }

    fn rmul(&self, coefficient: i32) -> Result<FieldElement, &'static str> {
        // Perform multiplication operation with the coefficient
        let num = (self.value * coefficient) % self.magnitude;

        // Create a new FieldElement object with the result value
        let new_element = FieldElement::new(num, self.magnitude)?;

        // Ensure that the magnitude of the new element is valid
        if new_element.magnitude != self.magnitude {
            return Err("Invalid magnitude of result FieldElement");
        }

        // Ensure that the normalization of the new element is valid
        let expected_normalized = new_element.magnitude <= 1;
        if new_element.normalized != expected_normalized {
            return Err("Invalid normalization of result FieldElement");
        }

        Ok(new_element)
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement(value: {}, magnitude: {})", self.value, self.magnitude)
    }
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



/* Bounds analysis (over the rationals).
*
* Let m = r->magnitude
*     C = 0xFFFFFFFFFFFFFULL * 2
*     D = 0x0FFFFFFFFFFFFULL * 2
*
* Initial bounds: t0..t3 <= C * m
*                     t4 <= D * m
*/

/* 
impl Signature {
    // Constructs an ECDSA Bitcoin signature for [`EcdsaSighashType::All`].
    pub fn sighash_all(signature: secp256k1::ecdsa::Signature) -> Self {
        Self { signature, sighash_type: EcdsaSighashType::All }
    }
}
*/

impl PrivateKey {
    fn hex() {}
    fn sign() {}
    fn deterministic_k() {}
    fn wif() {}

}

// A function that uses the Euclidean algorithm to calculate the modular reciprocal
fn mod_inverse(a: i32, m: i32) -> i32 {
    let mut m0 = m;
    let mut a0 = a;
    let mut t;
    let mut q;
    let mut x0 = 0;
    let mut x1 = 1;
    
    if m == 1 {
        return 0;
    }

// Performing the Extended Euclidean Algorithm
while a0 > 1 {
        // q is quotient
        q = a0 / m0;
        t = m0;

        // m0 is remainder now, process same as
        // Euclid's algo
        m0 = a0 % m0;
        a0 = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    // Make x1 positive
    if x1 < 0 {
        x1 = x1 + m;
    }

    x1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let field_element = FieldElement::new(5, 10);
        assert!(field_element.is_ok());
    }

    #[test]
    fn test_new_invalid_magnitude() {
        let field_element = FieldElement::new(5, 0);
        assert!(field_element.is_err());
    }

    #[test]
    fn test_new_invalid_value() {
        let field_element = FieldElement::new(10, 5);
        assert!(field_element.is_err());
    }

    #[test]
    fn test_add_valid() {
        let field_element1 = FieldElement::new(5, 10).unwrap();
        let field_element2 = FieldElement::new(7, 10).unwrap();
        let result = field_element1.add(&field_element2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_invalid_magnitude() {
        let field_element1 = FieldElement::new(5, 10).unwrap();
        let field_element2 = FieldElement::new(7, 5).unwrap();
        let result = field_element1.add(&field_element2);
        assert!(result.is_err());
    }

    #[test]
    fn test_sub_valid() {
        let field_element1 = FieldElement::new(7, 10).unwrap();
        let field_element2 = FieldElement::new(5, 10).unwrap();
        let result = field_element1.sub(&field_element2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sub_invalid_magnitude() {
        let field_element1 = FieldElement::new(7, 10).unwrap();
        let field_element2 = FieldElement::new(5, 5).unwrap();
        let result = field_element1.sub(&field_element2);
        assert!(result.is_err());
    }

    #[test]
    fn test_mul_valid() {
        let field_element1 = FieldElement::new(5, 10).unwrap();
        let field_element2 = FieldElement::new(7, 10).unwrap();
        let result = field_element1.mul(&field_element2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mul_invalid_magnitude() {
        let field_element1 = FieldElement::new(5, 10).unwrap();
        let field_element2 = FieldElement::new(7, 5).unwrap();
        let result = field_element1.mul(&field_element2);
        assert!(result.is_err());
    }

    #[test]
    fn test_pow_valid() {
        let field_element = FieldElement::new(5, 10).unwrap();
        let result = field_element.pow(3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pow_invalid_exponent() {
        let field_element = FieldElement::new(5, 10).unwrap();
        let result = field_element.pow(-3);
        assert!(result.is_err());
    }

    #[test]
    fn test_truediv_valid() {
        let field_element1 = FieldElement::new(7, 10).unwrap();
        let field_element2 = FieldElement::new(5, 10).unwrap();
        let result = field_element1.truediv(&field_element2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_truediv_invalid_magnitude() {
        let field_element1 = FieldElement::new(7, 10).unwrap();
        let field_element2 = FieldElement::new(5, 5).unwrap();
        let result = field_element1.truediv(&field_element2);
        assert!(result.is_err());
    }

    #[test]
    fn test_rmul_valid() {
        let field_element = FieldElement::new(5, 10).unwrap();
        let result = field_element.rmul(3);
        assert!(result.is_ok());
    }

}

fn main() {
    println!("Hello, world!");
}
