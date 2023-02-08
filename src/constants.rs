//! Stark curve constants

use crate::core::{field_element::FieldElementCore, W};
use crate::FieldElement;

/// Internal bytes representation of defined constants.
///
/// See the tests below which ensure that specified internal bytes repr
/// indeed matches constant defined in the specs
mod internal_bytes_repr {
    pub const EQUATION_A: [u64; 4] = [
        18446744073709551585,
        18446744073709551615,
        18446744073709551615,
        576460752303422960,
    ];
    pub const EQUATION_B: [u64; 4] = [
        3863487492851900874,
        7432612994240712710,
        12360725113329547591,
        88155977965380735,
    ];
    pub const GENERATOR_X: [u64; 4] = [
        14484022957141291997,
        5884444832209845738,
        299981207024966779,
        232005955912912577,
    ];
    pub const GENERATOR_Y: [u64; 4] = [
        6241159653446987914,
        664812301889158119,
        18147424675297964973,
        405578048423154473,
    ];
}

/// Coefficient $\alpha$ of curve equation
///
/// $\alpha = 1$
pub const EQUATION_A: FieldElement = W::new(FieldElementCore::from_internal_repr(
    internal_bytes_repr::EQUATION_A,
));

/// Coefficient $\beta$ of curve equation
///
/// $\beta = 3141592653589793238462643383279502884197169399375105820974944592307816406665$
pub const EQUATION_B: FieldElement = W::new(FieldElementCore::from_internal_repr(
    internal_bytes_repr::EQUATION_B,
));

/// Curve generator coordinates $(x, y)$
///
/// * $x = 874739451078007766457464989774322083649278607533249481151382481072868806602$
/// * $y = 152666792071518830868575557812948353041420400780739481342941381225525861407$
pub const GENERATOR: (FieldElement, FieldElement) = (
    W::new(FieldElementCore::from_internal_repr(
        internal_bytes_repr::GENERATOR_X,
    )),
    W::new(FieldElementCore::from_internal_repr(
        internal_bytes_repr::GENERATOR_Y,
    )),
);

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use crate::FieldElement;

    #[test]
    fn defined_cosntants_align_with_specs() {
        // Hex-encoded constants can be found here:
        // https://github.com/starkware-libs/starkex-resources/blob/844ac3dcb1f735451457f7eecc6e37cd96d1cb2d/crypto/starkware/crypto/signature/signature.js#L38-L50

        let a = FieldElement::ONE;
        let b = FieldElement::from_be_bytes(
            hex!("06f21413 efbe40de 150e596d 72f7a8c5 609ad26c 15c915c1 f4cdfcb9 9cee9e89").into(),
        )
        .unwrap();

        let g_x = FieldElement::from_be_bytes(
            hex!("01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca").into(),
        )
        .unwrap();
        let g_y = FieldElement::from_be_bytes(
            hex!("005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f").into(),
        )
        .unwrap();

        println!("a: {:?}", a.internal_repr());
        println!("b: {:?}", b.internal_repr());
        println!("g_x: {:?}", g_x.internal_repr());
        println!("g_y: {:?}", g_y.internal_repr());

        assert_eq!(super::EQUATION_A, a);
        assert_eq!(super::EQUATION_B, b);
        assert_eq!(super::GENERATOR.0, g_x);
        assert_eq!(super::GENERATOR.1, g_y);
    }
}
