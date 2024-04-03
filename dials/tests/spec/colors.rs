
dials::spec!{
    struct Rbga {
        red: u8,
        blue: u8,
        green: u8,
        alpha: u8,
    }
}

#[test]
fn rgba_word() {
    let x = Rbga(0);

    let x = x.alpha() as u8;
}
