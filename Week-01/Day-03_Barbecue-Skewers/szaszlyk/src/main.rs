fn main() {
    let _szaszlyk = [
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ];
    let szaszlyk2 = [
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----",
    ];
    let mut vege = 0;
    let mut miesne = 0;
    let sz_iter = szaszlyk2.iter();
    for x in sz_iter {
        if x.contains('x') {
            miesne += 1;
        } else {
            vege += 1;
        }
        println!("{}", x);
    }
    println!("[{}, {}]", vege, miesne);
}
