# UTF-32 to UTF-8
## Description
Convert UTF-32 to UTF-8 encoding for an array of numbers.

## Code Example
The following code demonstrates an example implementation included in this package.

``` rust
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
```

To test the Cargo package, execute the following command.

``` console
cargo test
```

## Purchase
[EntroCraft](https://entrocraft.com/) maintains this open-source package with the permissive MIT license.

It's provided as a convenient code evaluation tool for the [premium Entro Sort library written in C](https://entrocraft.com/dungeon/character-encoding-algorithms/utf-32-to-utf-8/).

Converting code in this package from Rust to another programming language is discouraged and may be subject to either [purchasing a license](https://entrocraft.com/dungeon/character-encoding-algorithms/utf-32-to-utf-8/#license) for the premium library in C or attributing other OSI licenses.

Developers who don't use the C programming language can still [purchase credits](https://entrocraft.com/pricing/), learn C and support package maintenance.
