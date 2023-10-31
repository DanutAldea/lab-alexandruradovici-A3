//! Make me compile only by reordering the lines in `main()`, but without
//! adding, changing or removing any of them.

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    // x can be borrowed mutably only after its previous borrow
    // expires, which menas y is not used anymore
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
