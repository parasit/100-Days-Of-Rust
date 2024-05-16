
fn sort_array(a1: &mut [u32], a2: &[u32]) {
    let s2 = a2.len();
    let s1 = a1.len()-s2;
    a1[s1..(s1+s2)].copy_from_slice(a2);
    a1.sort();
}

fn main() {
    let mut a = [1,2,3,4,0,0,0];
    let b = [2,5,6];
    // a[4..8].copy_from_slice(&b);
    println!("a {:?}", a);
    println!("b {:?}", b);

    sort_array(&mut a, &b);
    println!("{:?}", a);
}
