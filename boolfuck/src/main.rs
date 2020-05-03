use itertools::Itertools;

#[derive(Debug)]
struct Bits {
    ptr: usize,
    bytes: Vec<u8>,
}

impl Bits {
    fn new() -> Bits { Bits { ptr: 0, bytes: Vec::new() } }
    fn from(v: Vec<u8>) -> Bits { Bits { ptr: 0, bytes: v } }

    fn get(&self) -> u8 {
        let index = self.ptr / 8;
        let bit: u8 = (self.ptr % 8) as u8;
        let mask = (1 as u8) << bit;
        if index >= self.bytes.len() || self.bytes[index] & mask == 0 { 0 } else { 1 }
    }

    fn set(&mut self, value: u8) {
        let index = self.ptr / 8;
        let bit: u8 = (self.ptr % 8) as u8;

        while index >= self.bytes.len() {
            self.bytes.push(0);
        }

        if value == 0 {
            self.bytes[index] &= (1 << bit) ^ 0xFF
        } else {
            self.bytes[index] |= 1 << bit;
        };
    }

    fn right(&mut self) {
        self.ptr += 1;
    }

    fn left(&mut self) {
        if self.ptr == 0 {
            self.ptr = 7;
            self.bytes.insert(0, 0);
        } else {
            self.ptr -= 1;
        }
    }
}

fn match_offset<'a, T: Iterator<Item=&'a char>>(iter: T, target: char, source: char) -> usize {
    let mut count = 0;
    for (i, c) in iter.enumerate() {
        match *c {
            t if t == target && count == 0 => return i + 1,
            t if t == target => count -= 1,
            s if s == source => count += 1,
            _ => ()
        }
    }
    return 1;
}

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let code = code.chars().collect_vec();
    let mut tape = Bits::new();
    let mut output = Bits::new();
    let mut input = Bits::from(input);
    let mut c: usize = 0;

    while c < code.len() {
        match code[c] {
            '+' => tape.set(if tape.get() == 0 { 1 } else { 0 }),
            ',' => {
                tape.set(input.get());
                input.right();
            },
            ';' => {
                output.set(tape.get());
                output.right();
            },
            '<' => tape.left(),
            '>' => tape.right(),
            _ => ()
        }
        // println!("code: {}, tape: {:?}, output: {:?}", code[c], tape, output);
        match code[c] {
            '[' if tape.get() == 0 =>
                c += match_offset(code[(c + 1)..].iter(), ']', '[') + 1,
            ']' =>
                c -= match_offset(code[..c].iter().rev(), '[', ']'),
            _ => c += 1,
        }
    }
    return output.bytes;
}

#[cfg(test)]
mod tests {
    use crate::boolfuck;
    #[test]
    fn example_test_cases() {
        // Hello World Program taken from the official website
        assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
        // Echo until byte(0) encountered
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
        // Two numbers multiplier
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
    }
}
/*
#[test]
fn example_test_cases() {
    // Hello World Program taken from the official website
    assert_eq!(boolfuck(";;;+;+;;+;+;", Vec::new()), b"H", "Your interpreter did not work with the code example provided on the official website");
    assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
    // Echo until byte(0) encountered
    assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
    // Two numbers multiplier
    assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
}
*/
