use crate::bindgen::E;

mod bindgen {
    use rust_struct_bindgen_proc_macro::btf_struct_bindgen_with_elf;
    btf_struct_bindgen_with_elf!("assets/simple_prog.bpf.o");
}

mod util;

#[test]
fn test_deserializing() {
    let bin_data = std::fs::read(util::get_assets_dir().join("dumper_test.bin")).unwrap();
    let st = bindgen::S::from_bytes(&bin_data).unwrap();
    assert_eq!(st.f_str, "A-String");
    for i in 0..2 {
        for j in 0..3 {
            for k in 0..4 {
                assert_eq!(st.f_arr1[i][j][k], ((i << 16) + (j << 8) + k) as i32);
            }
        }
    }
    for i in 0..10 {
        assert_eq!(st.f_str_arr[i], format!("hello {}", i));
    }
    assert_eq!(st.f_ft, 1.23f32);
    assert_eq!(st.f_dbl, 4.56f64);
    assert_eq!(st.f_u8v, 0x12);
    assert_eq!(st.f_u16v, 0x1234);
    assert_eq!(st.f_u32v, 0x12345678);
    assert_eq!(st.f_u64v, 0x123456789abcdef0);
    assert_eq!(st.f_i8v, -0x12);
    assert_eq!(st.f_i16v, -0x1234);
    assert_eq!(st.f_i32v, -0x12345678);
    assert_eq!(st.f_i64v, -0x123456789abcdef0);
    assert!(matches!(st.f_e, E::E_C));
}

#[test]
fn test_serializing() {
    let bin_data = std::fs::read(util::get_assets_dir().join("dumper_test.bin")).unwrap();
    // let st = bindgen::S::from_bytes(&bin_data).unwrap();
    let mut arr1 = [[[0i32; 4]; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            for k in 0..4 {
                arr1[i][j][k] = ((i << 16) + (j << 8) + k) as i32;
            }
        }
    }
    let mut str_arr = vec![];
    for i in 0..10 {
        str_arr.push(format!("hello {}", i));
    }
    let st = bindgen::S {
        f_arr1: arr1,
        f_str: "A-String".into(),
        f_str_arr: str_arr.try_into().unwrap(),
        f_ft: 1.23,
        f_dbl: 4.56,
        f_u8v: 0x12,
        f_i8v: -0x12,
        f_u16v: 0x1234,
        f_i16v: -0x1234,
        f_u32v: 0x12345678,
        f_i32v: -0x12345678,
        f_u64v: 0x123456789abcdef0,
        f_i64v: -0x123456789abcdef0,
        f_e: bindgen::E::E_C,
    };
    let ser_bytes = st.to_bytes().unwrap();
    // Ok to directly compare bytes, since we ensured the unused area of the binary is filled with zero, in both `simple_prog.c` and here
    assert_eq!(ser_bytes, bin_data);
}
