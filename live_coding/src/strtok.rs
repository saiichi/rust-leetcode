pub fn strtok<'a>(s: &'a mut &str, pat: char) -> &'a str {
    match s.find(pat) {
        Some(i) => {
            let ret = &s[..i];
            *s = &s[i + pat.len_utf8()..];
            ret
        },
        None => {
            let ret = *s;
            *s = "";
            ret
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s: &str = "Hello, world!";
        assert_eq!(s.find(' '), Some(6));

        let s1 = &mut s;
        let t = strtok(s1, ' ');
        assert_eq!(t, "Hello,");
    }
}