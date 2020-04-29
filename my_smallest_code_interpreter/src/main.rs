use std::collections::HashMap;

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    let mut c_ptr: usize = 0;
    let mut i_ptr: usize = 0;
    let mut d_ptr: usize = 0;
    let mut data: [u8; 30000] = [0; 30000];
    let code = Vec::from(code);
    let brackets = matching_brackets(&code);

    while c_ptr < code.len() {
        match char::from(code[c_ptr]) {
            '>' => d_ptr += 1,
            '<' => d_ptr -= 1,
            '+' if data[d_ptr] == 255 => data[d_ptr] = 0,
            '+' => data[d_ptr] += 1,
            '-' if data[d_ptr] == 0 => data[d_ptr] = 255,
            '-' => data[d_ptr] -= 1,
            '.' => output.push(data[d_ptr]),
            ',' => {
                data[d_ptr] = input[i_ptr];
                i_ptr += 1
            }
            _ => ()
        }

        match char::from(code[c_ptr]) {
            '[' if data[d_ptr] == 0 => c_ptr = brackets.get(&c_ptr).unwrap() + 1,
            ']' if data[d_ptr] != 0 => c_ptr = brackets.get(&c_ptr).unwrap() + 1,
            _ => c_ptr += 1,
        }
    }

    output
}

fn matching_brackets(s: &Vec<u8>) -> HashMap<usize, usize> {
    let mut res: HashMap<usize, usize> = HashMap::new();
    let mut stack: Vec<usize> = Vec::new();
    for (j, c) in s.iter().enumerate() {
        if char::from(*c) == '[' {
            stack.push(j);
        } else if char::from(*c) == ']' {
            let i = stack.pop().unwrap();
            res.insert(i, j);
            res.insert(j, i);
        }
    }
    return res;
}

// the function ez_vec takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
// Without it, character-based tests are a pain

fn ez_vec(s: &str, byte: u8) -> Vec<u8> {
    let mut res = Vec::from(s.as_bytes());
    res.push(byte);
    res
}

#[test]
fn example_tests() {
    // Echo until byte 255 encountered
    assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(), "Codewars");
    // Echo until byte 0 encountered
    assert_eq!(String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(), "Codewars");
    // Multiply two numbers
    assert_eq!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]), vec![72]);
}