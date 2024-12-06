// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let len1 = s1.trim().chars().count();
    let len2 = s2.trim().chars().count();
    if len1 == len2 {
        return None;
    }

    if len1 > len2 {
        return Some(s1);
    }
    return Some(s2);
}

fn main() {
    longer_wish("sdads", "asdasd");
}
