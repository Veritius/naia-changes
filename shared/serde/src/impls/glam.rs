//! Trait implementations for glam types.

use glam::{Affine2, Affine3A, BVec2, BVec3, BVec3A, Vec2, Vec3, Vec3A, Vec4, Mat2, Mat3, Mat3A, Mat4, Quat, DVec2};
use crate::serde::Serde;

macro_rules! impl_glam {
    (f32, $target:ident, $size:expr, $to:ident, $from:ident) => {
        impl Serde for $target {
            fn ser(&self, writer: &mut dyn crate::BitWrite) {
                for x in self.$to() {
                    writer.write_bits(x.to_bits());
                }
            }

            fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
                let mut vals = [f32::NAN; $size];
                for i in 0..2 {
                    vals[i] = f32::from_bits(reader.read_bits()?);
                }
                Ok($target::$from(&vals))
            }

            fn bit_length(&self) -> u32 {
                std::mem::size_of::<Self>() as u32 * 8
            }
        }
    };

    (f32, $target:ident, $size:expr, $to:ident, $from:ident, owned) => {
        impl Serde for $target {
            fn ser(&self, writer: &mut dyn crate::BitWrite) {
                for x in self.$to() {
                    writer.write_bits(x.to_bits());
                }
            }

            fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
                let mut vals = [f32::NAN; $size];
                for i in 0..2 {
                    vals[i] = f32::from_bits(reader.read_bits()?);
                }
                Ok($target::$from(vals))
            }

            fn bit_length(&self) -> u32 {
                std::mem::size_of::<Self>() as u32 * 8
            }
        }
    };
}

impl_glam!(f32, Vec2, 2, to_array, from_array, owned);
impl_glam!(f32, Vec3, 3, to_array, from_array, owned);
impl_glam!(f32, Vec3A, 3, to_array, from_array, owned);
impl_glam!(f32, Vec4, 4, to_array, from_array, owned);
impl_glam!(f32, Mat2, 4, to_cols_array, from_cols_array);
impl_glam!(f32, Mat3, 9, to_cols_array, from_cols_array);
impl_glam!(f32, Mat3A, 9, to_cols_array, from_cols_array);
impl_glam!(f32, Mat4, 16, to_cols_array, from_cols_array);
impl_glam!(f32, Quat, 4, to_array, from_array, owned);
impl_glam!(f32, Affine2, 6, to_cols_array, from_cols_array);
impl_glam!(f32, Affine3A, 12, to_cols_array, from_cols_array);

#[test]
fn glam_type_equivalency_test() {
    use crate::{BitWriter, BitReader};
    use glam::EulerRot;
    
    // Checks that T has the same value when it goes through serialisation/deserialisation.
    // Does not test for endian or other platform-related problem. May also suffer from fails due to floating points even if the bits are the same.
    // TODO: This should be somewhere else so it can be used in other parts of the code for testing.
    fn type_equivalency_check<T: Serde + PartialEq + std::fmt::Debug>(test_values: &[T]) {
        for val in test_values {
            let mut writer = BitWriter::new();
            val.ser(&mut writer);
            let bytes = writer.to_bytes();
            let mut reader = BitReader::new(&bytes);
            let output = T::de(&mut reader).expect("Deserialisation failed (impl must be faulty)");
            assert_eq!(*val, output);
        }
    }

    // Basically just runs through all the constants, because I can't think of good testing values.
    // This might need to be extended with more values.
    // type_equivalency_check::<Affine2>(&[Affine2::ZERO, Affine2::IDENTITY]);
    // type_equivalency_check::<Affine3A>(&[Affine3A::ZERO, Affine3A::IDENTITY]);
    // type_equivalency_check::<BVec2>(&[BVec2::TRUE, BVec2::FALSE]);
    // type_equivalency_check::<BVec3>(&[BVec3::TRUE, BVec3::FALSE]);
    // type_equivalency_check::<BVec3A>(&[BVec3A::TRUE, BVec3A::FALSE]);
    // type_equivalency_check::<Vec2>(&[Vec2::ZERO, Vec2::ONE, Vec2::NEG_ONE, Vec2::X, Vec2::Y, Vec2::NEG_X, Vec2::NEG_Y]);
    // type_equivalency_check::<Vec3>(&[Vec3::ZERO, Vec3::ONE, Vec3::NEG_ONE, Vec3::X, Vec3::Y, Vec3::Z, Vec3::NEG_X, Vec3::NEG_Y, Vec3::NEG_Z]);
    // type_equivalency_check::<Vec3A>(&[Vec3A::ZERO, Vec3A::ONE, Vec3A::NEG_ONE, Vec3A::X, Vec3A::Y, Vec3A::Z, Vec3A::NEG_X, Vec3A::NEG_Y, Vec3A::NEG_Z]);
    // type_equivalency_check::<Vec4>(&[Vec4::ZERO, Vec4::ONE, Vec4::NEG_ONE, Vec4::X, Vec4::Y, Vec4::Z, Vec4::W, Vec4::NEG_X, Vec4::NEG_Y, Vec4::NEG_Z, Vec4::NEG_W]);
    // type_equivalency_check::<Quat>(&[Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0)]);
    // type_equivalency_check::<Mat2>(&[Mat2::ZERO, Mat2::IDENTITY]);
    // type_equivalency_check::<Mat3>(&[Mat3::ZERO, Mat3::IDENTITY]);
    // type_equivalency_check::<Mat3A>(&[Mat3A::ZERO, Mat3A::IDENTITY]);
    // type_equivalency_check::<Mat4>(&[Mat4::ZERO, Mat4::IDENTITY]);
}