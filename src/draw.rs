use crate::waka_apis::structs::Stats;

pub fn draw(status: Stats, size: i32) -> std::string::String {
    let mut result = String::new();
    let languages = status.data.languages;
    let mut max_len = 0;
    languages.iter().for_each(|i| {
        max_len = if max_len > i.name.len() {
            max_len
        } else {
            i.name.len()
        }
    });
    println!("{}", max_len);
    result+=r#"
#### <a href="https://github.com/someone120/someone120" target="_blank">📊 Weekly development breakdown</a>
```text
    "#;
    for i in languages {
        result += "\n";
        result += &i.name;
        for _ in 0..(max_len - i.name.len()) {
            result += " ";
        }
        result+="|";
        for _ in 0..((i.percent / 100.0) * size as f64) as i32 {
            result += "#";
        }
        for _ in 0..(size - ((i.percent / 100.0) * size as f64) as i32) {
            result += " ";
        }
        result +=&format!("|{} {}%",i.text,i.percent);
    }
    result += "\n```\n";
    result
}
