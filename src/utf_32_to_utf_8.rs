pub fn utf_32_to_utf_8(input: &[u32], output_count: &mut usize, output: &mut [u8]) -> bool {
    let mut code_point: u32;
    let mut i: usize;
    let mut j: usize = 0;
    let mut is_reverse_byte_order: bool;
    let mut is_valid = true;

    if input.len() != 0 {
        i = 0;
        is_reverse_byte_order = false;

        if
            input[0] == 0xFFFE0000 ||
            input[0] == 0x0000FEFF
        {
            i = 1;

            if input[0] == 0xFFFE0000 {
                is_reverse_byte_order = true;
            }
        }

        while
            is_valid == true &&
            i != input.len()
        {
            if is_reverse_byte_order == true {
                code_point = (input[i] << 24) + (((input[i] >> 8) << 24) >> 8) + (((input[i] << 8) >> 24) << 8) + (input[i] >> 24);
            } else {
                code_point = input[i];
            }

            if code_point < 1114112 {
                if
                    code_point < 55296 ||
                    code_point > 57343
                {
                    if code_point < 128 {
                        output[j] = code_point as u8;
                    } else {
                        if code_point < 2048 {
                            output[j] = ((code_point >> 6) + 192) as u8;
                            j += 1;
                            output[j] = ((code_point & 63) + 128) as u8;
                        } else {
                            if code_point < 65536 {
                                output[j] = ((code_point >> 12) + 224) as u8;
                                j += 1;
                                output[j] = (((code_point >> 6) & 63) + 128) as u8;
                                j += 1;
                                output[j] = ((code_point & 63) + 128) as u8;
                            } else {
                                output[j] = ((code_point >> 18) + 240) as u8;
                                j += 1;
                                output[j] = (((code_point >> 12) & 63) + 128) as u8;
                                j += 1;
                                output[j] = (((code_point >> 6) & 63) + 128) as u8;
                                j += 1;
                                output[j] = ((code_point & 63) + 128) as u8;
                            }
                        }
                    }

                    i += 1;
                    j += 1;
                } else {
                    is_valid = false;
                }
            } else {
                is_valid = false;
            }
        }
    }

    *output_count = j;
    return is_valid;
}

pub fn utf_32_be_to_utf_8(input: &[u32], output_count: &mut usize, output: &mut [u8]) -> bool {
    let mut i: usize;
    let mut j: usize = 0;
    let mut is_valid = true;

    if input.len() != 0 {
        i = 0;

        while
            is_valid == true &&
            i != input.len()
        {
            if input[i] < 1114112 {
                if
                    input[i] < 55296 ||
                    input[i] > 57343
                {
                    if input[i] < 128 {
                        output[j] = input[i] as u8;
                    } else {
                        if input[i] < 2048 {
                            output[j] = ((input[i] >> 6) + 192) as u8;
                            j += 1;
                            output[j] = ((input[i] & 63) + 128) as u8;
                        } else {
                            if input[i] < 65536 {
                                output[j] = ((input[i] >> 12) + 224) as u8;
                                j += 1;
                                output[j] = (((input[i] >> 6) & 63) + 128) as u8;
                                j += 1;
                                output[j] = ((input[i] & 63) + 128) as u8;
                            } else {
                                output[j] = ((input[i] >> 18) + 240) as u8;
                                j += 1;
                                output[j] = (((input[i] >> 12) & 63) + 128) as u8;
                                j += 1;
                                output[j] = (((input[i] >> 6) & 63) + 128) as u8;
                                j += 1;
                                output[j] = ((input[i] & 63) + 128) as u8;
                            }
                        }
                    }

                    i += 1;
                    j += 1;
                } else {
                    is_valid = false;
                }
            } else {
                is_valid = false;
            }
        }
    }

    *output_count = j;
    return is_valid;
}

pub fn utf_32_le_to_utf_8(input: &[u32], output_count: &mut usize, output: &mut [u8]) -> bool {
    let mut code_point: u32;
    let mut i: usize;
    let mut j: usize = 0;
    let mut is_valid = true;

    if input.len() != 0 {
        i = 0;

        while
            is_valid == true &&
            i != input.len()
        {
            code_point = (input[i] << 24) + (((input[i] >> 8) << 24) >> 8) + (((input[i] << 8) >> 24) << 8) + (input[i] >> 24);

            if code_point < 1114112 {
                if
                    code_point < 55296 ||
                    code_point > 57343
                {
                    if code_point < 128 {
                        output[j] = code_point as u8;
                    } else {
                        if code_point < 2048 {
                            output[j] = ((code_point >> 6) + 192) as u8;
                            j += 1;
                            output[j] = ((code_point & 63) + 128) as u8;
                        } else {
                            if code_point < 65536 {
                                output[j] = ((code_point >> 12) + 224) as u8;
                                j += 1;
                                output[j] = (((code_point >> 6) & 63) + 128) as u8;
                                j += 1;
                                output[j] = ((code_point & 63) + 128) as u8;
                            } else {
                                output[j] = ((code_point >> 18) + 240) as u8;
                                j += 1;
                                output[j] = (((code_point >> 12) & 63) + 128) as u8;
                                j += 1;
                                output[j] = (((code_point >> 6) & 63) + 128) as u8;
                                j += 1;
                                output[j] = ((code_point & 63) + 128) as u8;
                            }
                        }
                    }

                    i += 1;
                    j += 1;
                } else {
                    is_valid = false;
                }
            } else {
                is_valid = false;
            }
        }
    }

    *output_count = j;
    return is_valid;
}
