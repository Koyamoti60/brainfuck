use std::fmt::Error;

pub struct Interpreter {
    input_p: usize,
    input: Vec<char>,
    heap_p: usize,
    heap: [u8; 4096]
}

impl Interpreter {
    pub fn new(input: String) -> Self {
        Interpreter {
            input_p: 0,
            input: input.chars().collect(),
            heap_p: 0,
            heap: [0; 4096]
        }
    }

    pub fn run(&mut self, source: String) -> Result<String, Error> {
        let mut out = String::new();
        let mut p = 0;
        let source = source.chars().collect::<Vec<char>>();
        let mut t = 0;

        while p < source.len() {
            match source[p] {
                '>' => {
                    if self.heap_p >= self.heap.len() {
                        panic!();
                    }
                    self.heap_p += 1;
                },
                '<' => {
                    self.heap_p -= 1;
                },
                '+' => {
                    self.heap[self.heap_p] += 1;
                },
                '-' => {
                    self.heap[self.heap_p] -= 1;
                },
                '.' => {
                    //print!("{}", self.heap[self.heap_p] as char);
                    out.push(self.heap[self.heap_p] as char);
                },
                ',' => {
                    if self.input_p >= self.input.len() {
                        self.heap[self.heap_p] = 0;
                    } else {
                        self.heap[self.heap_p] = self.input[self.input_p] as u8;
                        self.input_p += 1;
                    }
                },
                '[' => {
                    if self.heap[self.heap_p] == 0 {
                        t += 1;
                        while t != 0 {
                            p += 1;
                            if p >= source.len() {
                                panic!();
                            }
                            if source[p] == '[' {
                                t += 1;
                            } else if source[p] == ']' {
                                t -= 1;
                            }
                        }
                    }
                },
                ']' => {
                    if self.heap[self.heap_p] != 0 {
                        t += 1;
                        while t != 0 {
                            p -= 1;
                            if source[p] == '[' {
                                t -= 1;
                            } else if source[p] == ']' {
                                t += 1;
                            }
                        }
                    }
                },
                _ => {}
            }
            p += 1;
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::Interpreter;

    #[test]
    fn helloworld() {
        let mut i = Interpreter::new("".to_string());
        let r = i.run("++++++++++++++++++++++++++++++++.".to_string()).unwrap();
        assert_eq!(r, " ")
    }

    #[test]
    fn with_input() {
        let mut i = Interpreter::new("1abc".to_string());
        let r = i.run(",.,.,.>++++++++++.".to_string()).unwrap();
        assert_eq!(r, "1ab\n")
    }
}