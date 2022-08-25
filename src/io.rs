use std::{fs::File, io::Read, path::Path};

pub fn read_source_file(file_name: &str) -> String {
    let path_str = format!("sources/{}", file_name);
    let path = Path::new(&path_str);

    let mut file =
        File::open(path).expect(format!("File {} doesnt exist", path.to_str().unwrap()).as_str());

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    buf
}
