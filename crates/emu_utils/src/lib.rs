



/// Trait to get the number of elements in an enum
pub trait EnumCountT {
    /// Count or number of element in an enum
    const COUNT : usize;
}

/// Trait to get an enum from a given index
pub trait EnumFromIndexT : Sized {
    /// Try to convert an index to an enum
    fn from_idx(idx: usize) -> Option<Self>;
    /// Convert an index to an enum, without checking bounds
    /// 
    /// # SAFETY
    /// 
    /// The user is required to make sure that the index is an index of a valid enum variant
    unsafe fn from_idx_unchecked(idx: usize) -> Self;
}



pub fn sign_extend_64(val: u64, sign_bit_idx: u8) -> u64 {
    let bit = (val >> sign_bit_idx as u64) & 0x1;

    let mask = u64::MAX << sign_bit_idx;
    if bit == 1 {
        val | mask
    } else {
        val & !mask
    }
}