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
