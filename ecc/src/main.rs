

/***********************************************************************************************************************************************
* Compared to Jimmy's Python code(class FieldElement, ecc.py), what the different approach to field elements is that the concept of 'magnitude'*
* by referring to the following file : https://github.com/bitcoin-core/secp256k1/blob/master/src/field.h                                       *
* In the context of elliptic curve cryptography and field arithmetic, "magnitude" refers to the size or order of a field element.              *
* It is created with FieldElement objects and verified that the generated objects' magnitude and normalized are valid to verify their validity *
* In addition, it represents the number of bits required to represent the element in binary form.                                              *
************************************************************************************************************************************************/

use std::fmt;
use std::fmt::{Debug, Display};

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
#[derive(Debug, PartialEq, Clone)]
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

    // Returns a new FieldElement with value 0 and the given magnitude
    fn zero(magnitude: i32) -> FieldElement {
        FieldElement {
            value: 0,
            magnitude,
            normalized: magnitude <= 1,
        }
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

/* Implementation of methods in Point class does not require concepts such as magnetude and normalized, for two reasons:
*  
*  1. The Point class represents a point on an elliptic curve, which does not require verifying the properties of a finite element like the FieldElement struct does. 
*     Objects in the Point struct represent coordinates on an elliptic curve, which is different from the operations defined in a finite element.
*
*  2. In addition, concepts such as 'magnitude' and 'normalized' are not used to deal with points on elliptic curves. 
*     The points on the elliptic curve have their respective x and y coordinate values, and you just need to check if they satisfy the equation of the elliptic curve. 
*     Therefore, no code is needed to verify properties such as 'magnitude' and 'normalized'.
*
*  Therefore, validation using concepts such as 'magnitude' and 'normalized' can be skipped in the implementation of methods in Point struct.
*/
#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
    infinity: bool,
}


impl Point {
    fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Result<Self, &'static str> {
        if let (Some(ref x), Some(ref y)) = (&x, &y) {
            let x_cubed = x.pow(3)?;
            let y_squared = y.pow(2)?;
            let equation = x_cubed.clone() + (&a.clone() * x.clone()) + b.clone(); // For some reasons, it's generally recommended to implement the Add trait when defining operator overloads or operations.
            if y_squared != equation {
                return Err("Point is not on the curve");
            }
        }
        Ok(Point { x: Some(x.unwrap_or_else(|| FieldElement::zero())), y: Some(y.unwrap_or_else(|| FieldElement::zero())), a, b, infinity: x.is_none() })
    }

    fn add(&self, other: &Point) -> Result<Point, &'static str> {
        if self.a != other.a || self.b != other.b {
            return Err("Points are not on the same curve");
        }

        if self.x.infinity() {
            return Ok(other.clone());
        }
        if other.x.infinity() {
            return Ok(self.clone());
        }

        if self.x == other.x && self.y != other.y {
            return Ok(Point::new(None, None, self.a, self.b)?); // Return point at infinity
        }

        if self == other {
            let s = (3 * self.x.pow(2) + self.a) / (2 * self.y);
            let x3 = s.pow(2) - 2 * self.x;
            let y3 = s * (self.x - x3) - self.y;
            return Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?);
        }

        if self.x != other.x {
            let s = (other.y - self.y) / (other.x - self.x);
            let x3 = s.pow(2) - self.x - other.x;
            let y3 = s * (self.x - x3) - self.y;
            return Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?);
        }

        // This case is unlikely to happen in practice as it's covered by the previous cases.
        // But for completeness, we handle it here.
        Err("Unexpected condition reached")
    }

    // Method to perform scalar multiplication operation
    fn rmul(&self, coefficient: usize) -> Result<Point, &'static str> {
        let mut coef = coefficient; // Copy of the scalar value
        let mut current = self.clone(); // Copy of the current point
        let mut result = Point::new(None, None, self.a.clone(), self.b.clone())?; // Create a new point to store the result

        // Repeat the multiplication operation until the scalar value becomes zero
        while coef > 0 {
            if coef & 1 == 1 {
                result = result.add(&current)?; // Add the current point to the result
            }
            current = current.add(&current)?; // Double the current point
            coef >>= 1; // Halve the scalar value
        }
        Ok(result)
    }
}


/* 
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

*/

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement(value: {}, magnitude: {})", self.value, self.magnitude)
    }
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

/* In the case of Jimmy Song's Python-written test code, invalid field values were taken into account, 
*  but in Rust, it is not common to test for invalid values or exceptions to incorrect situations when writing test scenarios. 
*  Instead, it is important to use valid inputs to verify that the code is working as expected Languages such as Python allow you 
*  to handle exception situations using a variety of patterns related to exception handling, but this pattern is not applied in Rust. 
*  Rust uses panics to handle runtime errors, which are primarily used by developers to modify or debug code.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let field_element = FieldElement::new(5, 10);
        assert!(field_element.is_ok());
    }


    #[test]
    fn test_add_valid() {
        let field_element1 = FieldElement::new(5, 10).unwrap();
        let field_element2 = FieldElement::new(7, 10).unwrap();
        let result = field_element1.add(&field_element2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sub_valid() {
        let field_element1 = FieldElement::new(7, 10).unwrap();
        let field_element2 = FieldElement::new(5, 10).unwrap();
        let result = field_element1.sub(&field_element2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mul_valid() {
        let field_element1 = FieldElement::new(5, 10).unwrap();
        let field_element2 = FieldElement::new(7, 10).unwrap();
        let result = field_element1.mul(&field_element2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pow_valid() {
        let field_element = FieldElement::new(5, 10).unwrap();
        let result = field_element.pow(3);
        assert!(result.is_ok());
    }
    
    /*  If field_element2 is 0, the truediv function is called and an error occurs, but the test also works successfully in this situation
    *  by returning the result and checking the result in the test code
    *
    #[test]
    fn test_truediv_valid() {
        let field_element1 = FieldElement::new(7, 10).unwrap();
        let field_element2 = FieldElement::new(5, 10).unwrap();
    
        let result = if field_element2.value != 0 {
            field_element1.truediv(&field_element2)
        } else {
            Err("Attempted to divide by zero.")
        };
    
        assert!(result.is_ok() || result.is_err() && result.err().unwrap() == "Attempted to divide by zero.");
    }
    */

    #[test]
    fn test_rmul_valid() {
        let field_element = FieldElement::new(5, 10).unwrap();
        let result = field_element.rmul(3);
        assert!(result.is_ok());
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point_new_valid() {
        // Test valid point creation
        let x = FieldElement::new(2, 5).unwrap();
        let y = FieldElement::new(3, 5).unwrap();
        let a = FieldElement::new(4, 5).unwrap();
        let b = FieldElement::new(5, 5).unwrap();
        let point = Point::new(Some(x), Some(y), a, b);
        assert!(point.is_ok());
    }
    
    #[test]
    fn test_point_new_invalid() {
        // Test invalid point creation (point not on the curve)
        let x = FieldElement::new(2, 5).unwrap();
        let y = FieldElement::new(3, 5).unwrap();
        let a = FieldElement::new(4, 5).unwrap();
        let b = FieldElement::new(6, 5).unwrap(); // Incorrect b value
        let point = Point::new(Some(x), Some(y), a, b);
        assert!(point.is_err());
    }
    
    #[test]
    fn test_point_add() {
        // Test point addition
        let a = FieldElement::new(2, 5).unwrap();
        let b = FieldElement::new(3, 5).unwrap();
        let x1 = FieldElement::new(1, 5).unwrap();
        let y1 = FieldElement::new(1, 5).unwrap();
        let x2 = FieldElement::new(2, 5).unwrap();
        let y2 = FieldElement::new(4, 5).unwrap();
        let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone()).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone()).unwrap();
        let sum = p1.add(&p2);
        assert!(sum.is_ok());
        let result = sum.unwrap();
        assert_eq!(result.x.unwrap().num, 3);
        assert_eq!(result.y.unwrap().num, 1);
    }
    
    #[test]
    fn test_point_rmul() {
        // Test scalar multiplication
        let a = FieldElement::new(2, 5).unwrap();
        let b = FieldElement::new(3, 5).unwrap();
        let x = FieldElement::new(1, 5).unwrap();
        let y = FieldElement::new(1, 5).unwrap();
        let p = Point::new(Some(x), Some(y), a.clone(), b.clone()).unwrap();
        let scalar = 3;
        let result = p.rmul(scalar);
        assert!(result.is_ok());
        let result_point = result.unwrap();
        assert_eq!(result_point.x.unwrap().num, 3);
        assert_eq!(result_point.y.unwrap().num, 1);
    }
}


fn main() {
    println!("Hello, world!");
}
