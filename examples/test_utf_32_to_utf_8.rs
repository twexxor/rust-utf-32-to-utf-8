extern crate utf_32_to_utf_8;

use utf_32_to_utf_8::utf_32_to_utf_8::*;

fn main() {
    let input_1: [u32; 5] = [0x0000FEFF, 0x00000001, 0x00000002, 0x00000003, 0x000000A9];
    let input_2: [u32; 5] = [0xFFFE0000, 0x01000000, 0x02000000, 0x03000000, 0xA9000000];
    let mut output_count: usize = 0;
    let mut output: [u8; 20] = [0; 20];
    let mut i: usize = 0;

    if utf_32_to_utf_8(&input_1, &mut output_count, &mut output) {
        while i != output_count {
            println!("0x{:02X}", output[i]);
            i += 1;
        }
    }

    println!("");

    if utf_32_be_to_utf_8(&input_1, &mut output_count, &mut output) {
        i = 0;

        while i != output_count {
            println!("0x{:02X}", output[i]);
            i += 1;
        }
    }

    println!("");

    if utf_32_le_to_utf_8(&input_2, &mut output_count, &mut output) {
        i = 0;

        while i != output_count {
            println!("0x{:02X}", output[i]);
            i += 1;
        }
    }
}
