// replace frame number with hash (#) chars
// "path/file.1234.ext" -> "path/file.####.ext"

fn find_replace_frame_num(filepath: &str) -> String {
    let f: Vec<_> = filepath
        .split(".")
        .map(|c| {
            if c.parse::<u32>().is_ok() {
                c.chars().map(|_| '#').collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect::<_>();

    f.join(".").to_string()
}
