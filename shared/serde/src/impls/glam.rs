//! Trait implementations for glam types.

use glam::{Affine2, Vec2, Vec3, Mat2, Mat3, Mat3A, Vec3A, Affine3A, BVec2, BVec3, BVec3A};
use crate::{serde::Serde, BitWriter, BitReader};

fn read_f32(reader: &mut crate::BitReader) -> Result<f32, crate::SerdeErr> {
    Ok(f32::from_bits(reader.read_bits()?))
}

impl Serde for Affine2 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.matrix2.ser(writer);
        self.translation.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let matrix2 = Mat2::de(reader)?;
        let translation = Vec2::de(reader)?;
        Ok(Affine2::from_mat2_translation(matrix2, translation))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Affine3A {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.matrix3.ser(writer);
        self.translation.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let matrix3 = Mat3A::de(reader)?;
        let translation = Vec3A::de(reader)?;
        Ok(Affine3A { matrix3, translation })
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for BVec2 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.x.ser(writer);
        self.y.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let x = reader.read_bit()?;
        let y = reader.read_bit()?;
        Ok(BVec2::new(x, y))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for BVec3 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.x.ser(writer);
        self.y.ser(writer);
        self.z.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let x = reader.read_bit()?;
        let y = reader.read_bit()?;
        let z = reader.read_bit()?;
        Ok(BVec3::new(x, y, z))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for BVec3A {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        // Exactly what the into_bool_array function does in glam, except glam has it private for some reason.
        let bitmask = self.bitmask();
        let array = [(bitmask & 1) != 0, (bitmask & 2) != 0, (bitmask & 4) != 0];
        for idx in array {
            writer.write_bit(idx);
        }
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let x = reader.read_bit()?;
        let y = reader.read_bit()?;
        let z = reader.read_bit()?;
        Ok(BVec3A::new(x, y, z))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Vec2 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.x.ser(writer);
        self.y.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let mut vec = Vec2::ZERO;
        vec.x = read_f32(reader)?;
        vec.y = read_f32(reader)?;

        Ok(vec)
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Vec3 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.x.ser(writer);
        self.y.ser(writer);
        self.z.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let mut vec = Vec3::ZERO;
        vec.x = read_f32(reader)?;
        vec.y = read_f32(reader)?;
        vec.z = read_f32(reader)?;

        Ok(vec)
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Vec3A {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        Vec3::from(*self).ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        Ok(Vec3A::from(Vec3::de(reader)?))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Mat2 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.x_axis.ser(writer);
        self.y_axis.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let x_axis = Vec2::de(reader)?;
        let y_axis = Vec2::de(reader)?;
        Ok(Mat2::from_cols(x_axis, y_axis))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Mat3 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.x_axis.ser(writer);
        self.y_axis.ser(writer);
        self.z_axis.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let x_axis = Vec3::de(reader)?;
        let y_axis = Vec3::de(reader)?;
        let z_axis = Vec3::de(reader)?;
        Ok(Mat3::from_cols(x_axis, y_axis, z_axis))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Mat3A {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.x_axis.ser(writer);
        self.y_axis.ser(writer);
        self.z_axis.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let x_axis = Vec3A::de(reader)?;
        let y_axis = Vec3A::de(reader)?;
        let z_axis = Vec3A::de(reader)?;
        Ok(Mat3A::from_cols(x_axis, y_axis, z_axis))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

#[test]
fn glam_type_equivalency_test() {
    // Basically just runs through all the constants, because I can't think of good testing values.
    type_equivalency_check::<Affine2>(&[Affine2::ZERO, Affine2::IDENTITY, Affine2::from_cols_array(&[0.0, 120.0, -15.0, 201423.0, 1019.0, -0.0])]);
    type_equivalency_check::<Affine3A>(&[Affine3A::ZERO, Affine3A::IDENTITY, Affine3A::from_cols_array(&[0.0, 120.0, -15.0, 201423.0, 1019.0, -0.0, 5925.0, 0.000021, 102.0, 20.0, -250.0, -0.000052])]);
    type_equivalency_check::<Vec2>(&[Vec2::ZERO, Vec2::ONE, Vec2::NEG_ONE, Vec2::X, Vec2::Y, Vec2::NEG_X, Vec2::NEG_Y]);
    type_equivalency_check::<Vec3>(&[Vec3::ZERO, Vec3::ONE, Vec3::NEG_ONE, Vec3::X, Vec3::Y, Vec3::Z, Vec3::NEG_X, Vec3::NEG_Y, Vec3::NEG_Z]);
    type_equivalency_check::<Vec3A>(&[Vec3A::ZERO, Vec3A::ONE, Vec3A::NEG_ONE, Vec3A::X, Vec3A::Y, Vec3A::Z, Vec3A::NEG_X, Vec3A::NEG_Y, Vec3A::NEG_Z]);
}

/// Checks that T has the same value when it goes through serialisation/deserialisation.
/// 
/// Does not test for endian or other platform-related problem. May also suffer from fails due to floating points even if the bits are the same.
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