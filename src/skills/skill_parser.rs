pub fn parse_skill_file(byte_content: &Vec<u8>) -> Vec<Vec<String>> {
    let mut hype_c = 0;
    let mut i = 0;
    let mut k = 0;
    let mut ext_content = vec![];

    let mut kvs = vec![];
    while i < byte_content.len() {
        if hype_c >= 2 {
            ext_content.push(vec![String::from("body_offset"), i.to_string()]);
            break;
        }
        if byte_content[i] == 10 {
            if &byte_content[k..i] == &[45, 45, 45] {
                hype_c += 1;
            }
            if &byte_content[k..i] != &[45, 45, 45] && byte_content.len() > 0 {
                kvs.push(String::from_utf8_lossy(&byte_content[k..i]).to_string());
            }
            if kvs.len() > 0 {
                ext_content.push(kvs.clone());
            }
            kvs.clear();
            k = i + 1;
        } else if byte_content[i] == 58 {
            if byte_content.len() > 0 {
                kvs.push(String::from_utf8_lossy(&byte_content[k..i]).to_string());
            }
            k = i + 1;
        }
        i += 1;
    }
    ext_content
}
