#[cfg(target_pointer_width = "64")]
pub const PTR_SIZE: usize = 8;

#[cfg(target_pointer_width = "32")]
pub const PTR_SIZE: usize = 4;

pub const BOARD_DOTS: &[u8; 127] = b". . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .";
