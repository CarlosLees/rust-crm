use regex::Regex;

fn extract_links(text: &str) -> Vec<&str> {
    // 定义 GitHub 和 YouTube 的正则表达式
    let re = Regex::new(r"https://(github\.com/[^\s]+|www\.youtube\.com/watch\?v=[^\s]+)").unwrap();

    // 查找所有匹配
    re.captures_iter(text)
        .filter_map(|cap| cap.get(0))
        .map(|m| m.as_str())
        .collect::<Vec<&str>>()
}

fn main() {
    let text = "转载自 https://www.youtube.com/watch?v=dPO4v5q9ULU
    Bun (1.1.29) vs Node.js (22.9) performance comparison.";
    let links = extract_links(text);

    println!("提取到的链接：{:?}", links);
}
