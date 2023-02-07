use hex_literal::hex;
use primeorder::elliptic_curve::sec1::ToEncodedPoint;
use stark_curve::{AffinePoint, Scalar};

// Test vectors are taken from: https://github.com/paulmillr/noble-curves/blob/fcd422d24617e610a41eaef82e5d8ebc1b190971/test/stark/basic.test.js#L8
const TEST_VECTORS: &[TestVector] = &[
    TestVector {
        k: hex!("0000000000000000 0000000000000000 0000000000000000 0000000000000001"),
        x: hex!("01ef15c18599971b 7beced415a40f0c7 deacfd9b0d1819e0 3d723d8bc943cfca"),
        y: hex!("005668060aa49730 b7be4801df46ec62 de53ecd11abe43a3 2873000c36e8dc1f"),
    },
    TestVector {
        k: hex!("0000000000000000 0000000000000000 0000000000000000 0000000000000002"),
        x: hex!("0759ca09377679ec d535a81e83039658 bf40959283187c65 4c5416f439403cf5"),
        y: hex!("06f524a3400e7708 d5c01a28598ad272 e7455aa88778b19f 93b562d7a9646c41"),
    },
    TestVector {
        k: hex!("0000000000000000 0000000000000000 0000000000000000 0000000000000003"),
        x: hex!("0411494b501a98ab d8262b0da1351e17 899a0c4ef23dd2f9 6fec5ba847310b20"),
        y: hex!("07e1b3ebac08924d 2c26f409549191fc f94f3bf6f301ed35 53e22dfb802f0686"),
    },
    TestVector {
        k: hex!("0800000000000010 ffffffffffffffff b781126dcae7b232 1e66a241adc64d2e"),
        x: hex!("01ef15c18599971b 7beced415a40f0c7 deacfd9b0d1819e0 3d723d8bc943cfca"),
        y: hex!("07a997f9f55b68e0 4841b7fe20b9139d 21ac132ee541bc5c d78cfff3c91723e2"),
    },
];

struct TestVector {
    k: [u8; 32],
    x: [u8; 32],
    y: [u8; 32],
}

#[test]
fn test_vectors() {
    for &TestVector { k, x, y } in TEST_VECTORS {
        let k = Scalar::from_be_bytes(k.into()).unwrap();
        let actual = (AffinePoint::GENERATOR * k)
            .to_affine()
            .to_encoded_point(false);

        assert_eq!(actual.x().unwrap().as_slice(), x);
        assert_eq!(actual.y().unwrap().as_slice(), y);
    }
}
