
const STEP: usize = 20;
const LOWER:i32 = 0;
const UPPER:i32 = 300;

fn main() {
    // Pretty awful to make it here.
    // I cannot run a range using decimal values in rust
    // And the for loop is not so simple
    for fahr in (LOWER..UPPER).step_by(STEP) {
        let f_fahr = fahr as f32;
        println!("{:3.0} {:6.1}\n", f_fahr, (5.0/9.0)*(f_fahr - 32.0))
    }
}

// Another option is to implement another iterator to do it, more complex and I don't understand
// yet all concepts, I will keep it here as reference to implement one later

/* struct SimpleStepRange(isize, isize, isize);  // start, end, and step

impl Iterator for SimpleStepRange {
    type Item = isize;

    #[inline]
    fn next(&mut self) -> Option<isize> {
        if self.0 < self.1 {
            let v = self.0;
            self.0 = v + self.2;
            Some(v)
        } else {
            None
        }
    }
}

fn main() {
    for i in SimpleStepRange(0, 10, 2) {
        println!("{}", i);
    }
} */