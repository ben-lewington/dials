use dials::spec;

#[test]
fn can_construct_bitflags() {
    spec! {
        struct MyFlags {
            flag_0: bool,
            flag_1: bool,
            flag_2: bool,
            flag_3: bool,
            flag_4: u3,
            flag_5: bool,
        }
    }
    let mut x = MyFlags(0);
    assert_eq!(x.flag_1(), false);
    x.set_flag_4(5);
    assert_eq!(x.flag_4(), 5);
    x.set_flag_4(8);
    assert_eq!(x.flag_4(), 0);
}
