use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let mut max = i64::MIN;
    let mut maxsum = i64::MIN;
    let mut cursum = 0i64;
    for _ in 0..n {
        let value = token.next::<i64>();
        cursum += value;
        if value > max {
            max = value;
        }
        if cursum < 0 {
            cursum = 0;
        } else if cursum > maxsum {
            maxsum = cursum;
        }
    }
    if maxsum > 0 {
        writeln!(out, "{maxsum}").unwrap();
    } else {
        writeln!(out, "{max}").unwrap();
    }
}

pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}
