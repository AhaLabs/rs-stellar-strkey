use stellar_strkey::*;
extern crate proptest;
use proptest::prelude::*;

#[test]
fn test_valid_public_keys() {
    // Valid account.
    assert_convert_roundtrip(
        "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5",
        &Strkey::PublicKeyEd25519(StrkeyPublicKeyEd25519([
            0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
            0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
            0x05, 0xc7, 0xb1, 0x03,
        ])),
    );
}

#[test]
fn test_invalid_public_keys() {
    // Invalid length (Ed25519 should be 32 bytes, not 5).
    let r = Strkey::from_string("GAAAAAAAACGC6");
    assert_eq!(r, Err(DecodeError::Invalid));

    // Invalid length (congruent to 1 mod 8).
    let r = Strkey::from_string("GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZA");
    assert_eq!(r, Err(DecodeError::Invalid));

    // Invalid length (base-32 decoding should yield 35 bytes, not 36).
    let r = Strkey::from_string("GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUACUSI");
    assert_eq!(r, Err(DecodeError::Invalid));

    // Invalid algorithm (low 3 bits of version byte are 7).
    let r = Strkey::from_string("G47QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVP2I");
    assert_eq!(r, Err(DecodeError::Invalid));
}

#[test]
fn test_valid_private_keys() {
    // Valid private key.
    assert_convert_roundtrip(
        "SBU2RRGLXH3E5CQHTD3ODLDF2BWDCYUSSBLLZ5GNW7JXHDIYKXZWHOKR",
        &Strkey::PrivateKeyEd25519(StrkeyPrivateKeyEd25519([
            0x69, 0xa8, 0xc4, 0xcb, 0xb9, 0xf6, 0x4e, 0x8a, 0x07, 0x98, 0xf6, 0xe1, 0xac, 0x65,
            0xd0, 0x6c, 0x31, 0x62, 0x92, 0x90, 0x56, 0xbc, 0xf4, 0xcd, 0xb7, 0xd3, 0x73, 0x8d,
            0x18, 0x55, 0xf3, 0x63,
        ])),
    );
}

proptest! {
    #[test]
    fn test_public_key_ed25519_from_string_doesnt_panic(data: String) {
        let _ = Strkey::from_string(&data);
    }
}

proptest! {
    #[test]
    fn test_public_key_ed25519_to_string_doesnt_panic(data: [u8; 32]) {
        Strkey::PublicKeyEd25519(StrkeyPublicKeyEd25519(data)).to_string();
    }
}

fn assert_convert_roundtrip(s: &str, strkey: &Strkey) {
    let strkey_result = Strkey::from_string(&s).unwrap();
    assert_eq!(&strkey_result, strkey);
    let str_result = strkey.to_string();
    assert_eq!(s, str_result)
}
