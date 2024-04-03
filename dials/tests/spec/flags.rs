use dials::spec;

#[test]
fn can_construct_bitflags_into_u8() {
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

#[test]
fn can_construct_bitflags_with_bigger_container_types() {
    spec! {
        struct MyFlags {
            flag_0: u2,
            flag_1: u2,
            flag_2: u4,
            flag_4: u3,
            flag_5: u5,
        }
    }
    let mut x = MyFlags(0);
    assert_eq!(x.flag_1(), 0);
    x.set_flag_1(3);
    assert_eq!(x.flag_1(), 3);
    x.set_flag_4(8);
    assert_eq!(x.flag_4(), 0);

    spec! {
        struct MyOtherFlags {
            flag_0: u4,
            flag_1: u4,
            flag_2: u8,
            flag_4: u6,
            flag_5: u10,
        }
    }
}
