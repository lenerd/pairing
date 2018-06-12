use super::{Fq, FqRepr, Fr, FrRepr, G1, G1Affine, G2, G2Affine};
use {CurveAffine, CurveProjective, EncodedPoint, PrimeField};

use serde::de::Error as DeserializeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const ERR_LEN: &str = "wrong length of deserialized group element";
const ERR_CODE: &str = "deserialized bytes don't encode a group element";

impl Serialize for G1 {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.into_affine().serialize(s)
    }
}

impl<'de> Deserialize<'de> for G1 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(G1Affine::deserialize(d)?.into_projective())
    }
}

impl Serialize for G1Affine {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_affine(self, s)
    }
}

impl<'de> Deserialize<'de> for G1Affine {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(deserialize_affine(d)?)
    }
}

impl Serialize for G2 {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.into_affine().serialize(s)
    }
}

impl<'de> Deserialize<'de> for G2 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(G2Affine::deserialize(d)?.into_projective())
    }
}

impl Serialize for G2Affine {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serialize_affine(self, s)
    }
}

impl<'de> Deserialize<'de> for G2Affine {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(deserialize_affine(d)?)
    }
}

/// Serializes a group element using its compressed representation.
fn serialize_affine<S: Serializer, C: CurveAffine>(c: &C, s: S) -> Result<S::Ok, S::Error> {
    c.into_compressed().as_ref().serialize(s)
}

/// Deserializes the compressed representation of a group element.
fn deserialize_affine<'de, D: Deserializer<'de>, C: CurveAffine>(d: D) -> Result<C, D::Error> {
    let bytes = <Vec<u8>>::deserialize(d)?;
    if bytes.len() != C::Compressed::size() {
        return Err(D::Error::custom(ERR_LEN));
    }
    let mut compressed = C::Compressed::empty();
    compressed.as_mut().copy_from_slice(&bytes);
    let to_err = |_| D::Error::custom(ERR_CODE);
    Ok(compressed.into_affine().map_err(to_err)?)
}

impl Serialize for Fr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.into_repr().serialize(s)
    }
}

impl<'de> Deserialize<'de> for Fr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Fr::from_repr(FrRepr::deserialize(d)?).map_err(|_| D::Error::custom(ERR_CODE))
    }
}

impl Serialize for FrRepr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for FrRepr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(FrRepr(<_>::deserialize(d)?))
    }
}

impl Serialize for Fq {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.into_repr().serialize(s)
    }
}

impl<'de> Deserialize<'de> for Fq {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Fq::from_repr(FqRepr::deserialize(d)?).map_err(|_| D::Error::custom(ERR_CODE))
    }
}

impl Serialize for FqRepr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for FqRepr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(FqRepr(<_>::deserialize(d)?))
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;

    use std::fmt::Debug;

    use rand::{Rng, SeedableRng, XorShiftRng};

    fn test_roundtrip<T: Serialize + for<'a> Deserialize<'a> + Debug + PartialEq>(t: &T) {
        let ser = serde_json::to_vec(t).unwrap();
        assert_eq!(*t, serde_json::from_slice(&ser).unwrap());
    }

    #[test]
    fn serde_g1() {
        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
        let g: G1 = rng.gen();
        test_roundtrip(&g);
        test_roundtrip(&g.into_affine());
    }

    #[test]
    fn serde_g2() {
        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
        let g: G2 = rng.gen();
        test_roundtrip(&g);
        test_roundtrip(&g.into_affine());
    }

    #[test]
    fn serde_fr() {
        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
        let f: Fr = rng.gen();
        test_roundtrip(&f);
        test_roundtrip(&f.into_repr());
    }

    #[test]
    fn serde_fq() {
        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
        let f: Fq = rng.gen();
        test_roundtrip(&f);
        test_roundtrip(&f.into_repr());
    }
}
