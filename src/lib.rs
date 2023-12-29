pub mod utf_32_to_utf_8;

#[cfg(test)]
mod test {
    use utf_32_to_utf_8;

    #[test]
    fn test_utf_32_to_utf_8() {
        let input: [u32; 5] = [0x0000FEFF, 0x00000001, 0x00000002, 0x00000003, 0x000000A9];
        let mut output_count: usize = 0;
        let mut output: [u8; 20] = [0; 20];

        if utf_32_to_utf_8::utf_32_to_utf_8(&input, &mut output_count, &mut output) {
            assert_eq!(5, output_count);
            assert_eq!(0x01, output[0]);
            assert_eq!(0x02, output[1]);
            assert_eq!(0x03, output[2]);
            assert_eq!(0xC2, output[3]);
            assert_eq!(0xA9, output[4]);
        }
    }

    #[test]
    fn test_utf_32_be_to_utf_8() {
        let input: [u32; 5] = [0x0000FEFF, 0x00000001, 0x00000002, 0x00000003, 0x000000A9];
        let mut output_count: usize = 0;
        let mut output: [u8; 20] = [0; 20];

        if utf_32_to_utf_8::utf_32_be_to_utf_8(&input, &mut output_count, &mut output) {
            assert_eq!(8, output_count);
            assert_eq!(0xEF, output[0]);
            assert_eq!(0xBB, output[1]);
            assert_eq!(0xBF, output[2]);
            assert_eq!(0x01, output[3]);
            assert_eq!(0x02, output[4]);
            assert_eq!(0x03, output[5]);
            assert_eq!(0xC2, output[6]);
            assert_eq!(0xA9, output[7]);
        }
    }

    #[test]
    fn test_utf_32_le_to_utf_8() {
        let input: [u32; 5] = [0xFFFE0000, 0x01000000, 0x02000000, 0x03000000, 0xA9000000];
        let mut output_count: usize = 0;
        let mut output: [u8; 20] = [0; 20];

        if utf_32_to_utf_8::utf_32_le_to_utf_8(&input, &mut output_count, &mut output) {
            assert_eq!(8, output_count);
            assert_eq!(0xEF, output[0]);
            assert_eq!(0xBB, output[1]);
            assert_eq!(0xBF, output[2]);
            assert_eq!(0x01, output[3]);
            assert_eq!(0x02, output[4]);
            assert_eq!(0x03, output[5]);
            assert_eq!(0xC2, output[6]);
            assert_eq!(0xA9, output[7]);
        }
    }
}
