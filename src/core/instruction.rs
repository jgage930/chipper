pub struct Instruction(u16);

impl Instruction {
    pub fn digits(&self) -> (u16, u16, u16, u16) {
        let digit_1 = (self.0 & 0xF000) >> 12;
        let digit_2 = (self.0 & 0x0F00) >> 8;
        let digit_3 = (self.0 & 0x00F0) >> 4;
        let digit_4 = self.0 & 0x000F;

        (digit_1, digit_2, digit_3, digit_4)
    }

    // The lowest 12 bits.
    pub fn nnn(&self) -> u16 {
        self.0 & 0x0FFF
    }

    // The lowest 4 bits of he instruction.
    pub fn n(&self) -> u16 {
        todo!();
    }

    // The lowest 4 bits of the high byte.
    pub fn x(&self) -> u16 {
        todo!();
    }

    // The upper 4 bits of the low byte.
    pub fn y(&self) -> u16 {
        todo!();
    }

    // The lowest 8 bits.
    pub fn kk(&self) -> u16 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn instruction() -> Instruction {
        Instruction(0xC8E7)
    }

    #[test]
    fn test_digits() {
        let output = instruction().digits();
        let expected = (0x000C, 0x0008, 0x000E, 0x0007);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_nnn() {
        let output = instruction().nnn();
        let expected = 0x08E7;
        assert_eq!(output, expected);
    }
}
