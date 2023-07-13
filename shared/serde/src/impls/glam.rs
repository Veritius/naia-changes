//! Trait implementations for glam types.

use glam::{Affine2, Affine3A, BVec2, BVec3, BVec3A, Vec2, Vec3, Vec3A, Vec4, Mat2, Mat3, Mat3A, Mat4, Quat};
use crate::serde::Serde;

// TODO: The length of this file can be reduced significantly with macros.

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
        self.to_array().ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let array = <[f32;3] as Serde>::de(reader)?;
        Ok(Vec3::from_array(array))
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

impl Serde for Vec4 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.to_array().ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let array = <[f32;4] as Serde>::de(reader)?;
        Ok(Vec4::from_array(array))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Quat {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.to_array().ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let array = <[f32;4] as Serde>::de(reader)?;
        Ok(Quat::from_array(array))
    }

    fn bit_length(&self) -> u32 {
        todo!()
    }
}

impl Serde for Mat2 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.to_cols_array().ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let array = <[f32;4] as Serde>::de(reader)?;
        Ok(Mat2::from_cols_array(&array))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Mat3 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.to_cols_array().ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let array = <[f32;9] as Serde>::de(reader)?;
        Ok(Mat3::from_cols_array(&array))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Mat3A {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.to_cols_array().ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let array = <[f32;9] as Serde>::de(reader)?;
        Ok(Mat3A::from_cols_array(&array))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

impl Serde for Mat4 {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.to_cols_array().ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let array = <[f32;16] as Serde>::de(reader)?;
        Ok(Mat4::from_cols_array(&array))
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}

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
    type_equivalency_check::<Affine2>(&[Affine2::ZERO, Affine2::IDENTITY]);
    type_equivalency_check::<Affine3A>(&[Affine3A::ZERO, Affine3A::IDENTITY]);
    type_equivalency_check::<BVec2>(&[BVec2::TRUE, BVec2::FALSE]);
    type_equivalency_check::<BVec3>(&[BVec3::TRUE, BVec3::FALSE]);
    type_equivalency_check::<BVec3A>(&[BVec3A::TRUE, BVec3A::FALSE]);
    type_equivalency_check::<Vec2>(&[Vec2::ZERO, Vec2::ONE, Vec2::NEG_ONE, Vec2::X, Vec2::Y, Vec2::NEG_X, Vec2::NEG_Y]);
    type_equivalency_check::<Vec3>(&[Vec3::ZERO, Vec3::ONE, Vec3::NEG_ONE, Vec3::X, Vec3::Y, Vec3::Z, Vec3::NEG_X, Vec3::NEG_Y, Vec3::NEG_Z]);
    type_equivalency_check::<Vec3A>(&[Vec3A::ZERO, Vec3A::ONE, Vec3A::NEG_ONE, Vec3A::X, Vec3A::Y, Vec3A::Z, Vec3A::NEG_X, Vec3A::NEG_Y, Vec3A::NEG_Z]);
    type_equivalency_check::<Vec4>(&[Vec4::ZERO, Vec4::ONE, Vec4::NEG_ONE, Vec4::X, Vec4::Y, Vec4::Z, Vec4::W, Vec4::NEG_X, Vec4::NEG_Y, Vec4::NEG_Z, Vec4::NEG_W]);
    type_equivalency_check::<Quat>(&[Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0)]);
    type_equivalency_check::<Mat2>(&[Mat2::ZERO, Mat2::IDENTITY]);
    type_equivalency_check::<Mat3>(&[Mat3::ZERO, Mat3::IDENTITY]);
    type_equivalency_check::<Mat3A>(&[Mat3A::ZERO, Mat3A::IDENTITY]);
    type_equivalency_check::<Mat4>(&[Mat4::ZERO, Mat4::IDENTITY]);
}