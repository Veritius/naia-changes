use bevy_transform::prelude::Transform;
use glam::{Vec3, Quat};
use crate::serde::Serde;

impl Serde for Transform {
    fn ser(&self, writer: &mut dyn crate::BitWrite) {
        self.translation.ser(writer);
        self.rotation.ser(writer);
        self.scale.ser(writer);
    }

    fn de(reader: &mut crate::BitReader) -> Result<Self, crate::SerdeErr> {
        let mut transform = Transform::default();
        transform.translation = Vec3::de(reader)?;
        transform.rotation = Quat::de(reader)?;
        transform.scale = Vec3::de(reader)?;
        Ok(transform)
    }

    fn bit_length(&self) -> u32 {
        std::mem::size_of::<Self>() as u32 * 8
    }
}