pub fn find_replace_frame_num(filepath: &str) -> Result<String, ()> {
    let mut s = filepath.split("/").collect::<Vec<_>>();
    if s.is_empty() {
        return Err(());
    }
    //
    let filename = s.last().ok_or(());
    let fs = filename?.split(".").collect::<Vec<_>>();

    let mut fs_: Vec<String> = Vec::new();

    for st in fs {
        let mut s: String = String::new();
        let n = st.parse::<u32>();
        if n.is_ok() {
            let hash = st.chars().map(|_| '#').collect::<String>();
            s.push_str(&hash);
        } else {
            s.push_str(st);
        }
        fs_.push(s);
    }
    let x = fs_.join(".");

    if let Some(last) = s.last_mut() {
        *last = &x
    }

    Ok(s.join("/"))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn two_numbers() {
//         let x = "/my/weird/path/test34.12.1234.bgeo.sc";
//         let r = "/my/weird/path/test34.##.####.bgeo.sc";
//         assert_eq!(find_replace_frame_num(x).unwrap_or("".to_string()), r);
//     }
//
//     #[test]
//     fn missing_dot() {
//         let x = "test.34exr";
//         let r = "test.34exr";
//         assert_eq!(find_replace_frame_num(x).unwrap_or("".to_string()), r);
//     }
//
//     #[test]
//     fn noslash() {
//         let x = "test.34.exr";
//         let r = "test.##.exr";
//         assert_eq!(find_replace_frame_num(x).unwrap_or("".to_string()), r);
//     }
//
//     #[test]
//     fn test_regular_path() {
//         let x = "/bob/joe/test.34.exr";
//         let r = "/bob/joe/test.##.exr";
//         assert_eq!(find_replace_frame_num(x).unwrap_or("".to_string()), r);
//     }
// }
