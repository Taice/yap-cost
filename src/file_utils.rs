use std::io;

pub fn get_vec(filename: &str) -> Vec<(String, u32, f32)> {
    let mut map: Vec<(String, u32, f32)> = vec![];

    for line in std::fs::read_to_string(filename).unwrap().split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut separator = 0;
        for (i, ch) in line.chars().enumerate() {
            if ch == ';' {
                separator = i;
            }
        }

        map.push((
            line[..separator].to_string(),
            line[(separator + 1)..].trim().parse().unwrap_or(0),
            0.,
        ));
    }

    map
}

pub fn write_vec_to_file(filename: &str, map: &Vec<(String, u32, f32)>) -> io::Result<()> {
    let mut file_contents = String::new();

    for (name, char_count, _) in map {
        file_contents += name;
        file_contents += ";";
        file_contents += &char_count.to_string();
        file_contents += "\n";
    }

    std::fs::write(filename, file_contents)?;
    Ok(())
}

pub fn insert_to_file(k: String, v: u32, filename: &str) -> io::Result<()> {
    let mut map = get_vec(filename);
    map.push((k, v, 0.));

    write_vec_to_file(filename, &map)
}

pub fn remove_from_file(k: &String, filename: &str) -> io::Result<()> {
    let mut map = get_vec(filename);

    let index = map
        .iter()
        .fold(0, |acc, (v, x, _)| if v == k { *x } else { acc });

    map.remove(index as usize);

    write_vec_to_file(filename, &map)
}
