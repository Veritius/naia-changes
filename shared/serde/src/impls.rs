mod array;
mod boxed;
mod hash;
mod option;
mod scalars;
mod string;
mod tuple;
mod vector;

#[cfg(feature="glam_support")]
mod glam;
#[cfg(feature="transform_support")]
mod bevy_transform;