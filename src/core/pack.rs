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

pub fn pack_8xu4(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u32 {
    (h as u32 & 0x0F)
        | (g as u32 & 0x0F) << 4
        | (f as u32 & 0x0F) << 8
        | (e as u32 & 0x0F) << 12
        | (d as u32 & 0x0F) << 16
        | (c as u32 & 0x0F) << 20
        | (b as u32 & 0x0F) << 24
        | (a as u32 & 0x0F) << 28
}

pub fn unpack_8xu4(p: u32) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    (
        (p >> 28) as u8 & 0x0F,
        (p >> 24) as u8 & 0x0F,
        (p >> 20) as u8 & 0x0F,
        (p >> 16) as u8 & 0x0F,
        (p >> 12) as u8 & 0x0F,
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
        let e: u8 = 15;
        let f: u8 = 15;
        let g: u8 = 15;
        let h: u8 = 15;
        
        let p2 = pack_2xu4(a, b);
        let p4 = pack_4xu4(a, b, c, d);
        let p8 = pack_8xu4(a, b, c, d, e, f, g, h);

        assert_eq!((a, b), unpack_2xu4(p2));
        assert_eq!((a, b, c, d), unpack_4xu4(p4));
        assert_eq!((a, b, c, d, e, f, g, h), unpack_8xu4(p8));
    }
}
