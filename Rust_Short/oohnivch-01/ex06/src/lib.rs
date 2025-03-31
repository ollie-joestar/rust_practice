#[allow(dead_code)]
fn  invalid(list: &[u8]) -> bool {
    let set = "0123456789";
    for i in 0..list.len() {
        if !set.contains(list[i]) {
            return false;
        }
    }
    true
}
#[allow(dead_code)]
fn big_add(a: &[u8],b: &[u8]) -> Vec<u8> {
    if a.is_empty() || b.is_empty() || invalid(a) || invalid(b) {
        panic!("Bruh")
    }
    let len;
    if a.len() > b.len() {
        len = a.len();
    } else {
        len = b.len();
    }
    let mut carry = 0;
    for i in len..0 {
        //if &a[i] + &b[i]
    }
    let mut v = Vec::new();
    v
}
