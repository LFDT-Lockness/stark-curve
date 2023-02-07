use primeorder::elliptic_curve::{sec1::ToEncodedPoint, Field};
use rand_dev::DevRng;
use stark_curve::Scalar;

#[test]
fn generator_at_small_scalar() {
    for i in 0..=10u16 {
        println!("scalar: {i}");
        let mut scalar_be = [0u8; 32];
        scalar_be[30..].copy_from_slice(&i.to_be_bytes());

        test_mult_generator_at_scalar(&scalar_be);
    }
}

#[test]
fn generator_at_large_scalar() {
    let mut rng = DevRng::new();

    for _ in 0..50 {
        let scalar = Scalar::random(&mut rng).to_be_bytes();
        println!("scalar: {:?}", scalar);

        test_mult_generator_at_scalar(&scalar.into());
    }
}

fn test_mult_generator_at_scalar(scalar_be: &[u8; 32]) {
    let actual = {
        let generator = stark_curve::AffinePoint::GENERATOR;
        let scalar = Scalar::from_be_bytes((*scalar_be).into()).unwrap();
        (generator * scalar).to_affine()
    };

    let expected = {
        let generator = starknet_curve::curve_params::GENERATOR;
        let scalar = starknet_ff::FieldElement::from_bytes_be(scalar_be).unwrap();
        println!("scalar bits: {:?}", scalar.to_bits_le());
        &generator * scalar.to_bits_le().as_slice()
    };

    assert_eq!(
        bool::from(actual.is_identity()),
        expected.infinity,
        "infinity"
    );

    if !expected.infinity {
        let actual_point_encoded = actual.to_encoded_point(false);
        let x = actual_point_encoded.x().unwrap();
        let y = actual_point_encoded.y().unwrap();

        assert_eq!(x.as_slice(), expected.x.to_bytes_be(), "x");
        assert_eq!(y.as_slice(), expected.y.to_bytes_be(), "y");
    }
}
