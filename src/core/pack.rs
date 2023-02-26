pub fn pack_2xu4(a: u8, b: u8) -> u8 {
    ((a & 0x0F) << 4) | (b & 0x0F)
}

pub fn unpack_2xu4(p: u8) -> (u8, u8) {
    ((p >> 4) & 0x0F, p & 0x0F)
}

pub fn pack_4xu4(a: u8, b: u8, c: u8, d: u8) -> u16 {
    (d as u16 & 0x0F) | (c as u16 & 0x0F) << 4 | (b as u16 & 0x0F) << 8 | (a as u16 & 0x0F) << 12
}

pub fn unpack_4xu4(p: u16) -> (u8, u8, u8, u8) {
    (
        ((p >> 12) as u8 & 0x0F),
        (p >> 8) as u8 & 0x0F,
        (p >> 4) as u8 & 0x0F,
        p as u8 & 0x0F,
    )
}

#[cfg(test)]
mod pack_test {
    use super::*;

    #[test]
    fn test_u8_pack() {
        let a: u8 = 15;
        let b: u8 = 15;
        let c: u8 = 15;
        let d: u8 = 15;
        let p2: u8 = pack_2xu4(a, b);
        let p4 = pack_4xu4(a, b, c, d);

        // assert_eq!(31, p2);

        assert_eq!((a, b), unpack_2xu4(p2));
        assert_eq!((a, b, c, d), unpack_4xu4(p4));

        println!("{} {}", p2, p4);
    }
}
