// needs lib.rs in /src/ to find the crate
extern crate tigershark3;
use tigershark3::frame_num::find_replace_frame_num;

#[test]
fn two_numbers_b() {
    let x = "/my/weird/path/test34.12.1234.bgeo.sc";
    let r = "/my/weird/path/test34.##.####.bgeo.sc";
    assert_eq!(find_replace_frame_num(x), r);
}

#[test]
fn missing_dot() {
    let x = "test.34exr";
    let r = "test.34exr";
    assert_eq!(find_replace_frame_num(x), r);
}

#[test]
fn noslash() {
    let x = "test.34.exr";
    let r = "test.##.exr";
    assert_eq!(find_replace_frame_num(x), r);
}

#[test]
fn test_regular_path() {
    let x = "/bob/joe/test.34.exr";
    let r = "/bob/joe/test.##.exr";
    assert_eq!(find_replace_frame_num(x), r);
}
