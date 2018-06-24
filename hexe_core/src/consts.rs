use core::mem;

pub const PTR_SIZE: usize = mem::size_of::<usize>();

pub const BOARD_DOTS: [u8; 127] = *b". . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .\n\
                                     . . . . . . . .";
