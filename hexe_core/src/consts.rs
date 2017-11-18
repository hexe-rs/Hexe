#[cfg(target_pointer_width = "64")]
pub const PTR_SIZE: usize = 8;

#[cfg(target_pointer_width = "32")]
pub const PTR_SIZE: usize = 4;

#[cfg(target_pointer_width = "16")] // Just in case
pub const PTR_SIZE: usize = 2;
