use std::{
    collections::HashMap,
    io::Read,
    process::{Command, ExitStatus},
};

fn main() -> std::io::Result<()> {
    let compressed_index =
        reqwest::blocking::get("https://origin.warframe.com/PublicExport/index_en.txt.lzma")
            .expect("Public Export index request should succeed")
            .bytes()
            .expect("Public Export index request should have a body")
            .to_vec();

    std::fs::write("/tmp/index_en.txt.lzma", &compressed_index).unwrap();

    assert!(
        Command::new("xz")
            .args(["-df", "/tmp/index_en.txt.lzma"])
            .spawn()
            .expect("xz command should be usable")
            .wait()
            .expect("child should exit")
            .success(),
        "xz command failed to run"
    );

    let index_file =
        std::fs::read_to_string("/tmp/index_en.txt").expect("index.txt should be writable");

    let index = index_file
        .lines()
        .map(|line| {
            let category = line
                .chars()
                .skip("Export".len())
                .take_while(|c| *c != '_')
                .collect::<String>();
            (category, line)
        })
        .collect::<HashMap<_, _>>();

    for (category, line) in index.into_iter() {
        let url = format!("http://content.warframe.com/PublicExport/Manifest/{line}");
        let json = reqwest::blocking::get(url.as_str())
            .expect(format!("Category '{category}' should be accessible at '{url}'").as_str())
            .json();
    }

    Ok(())
}

fn upgrades() {
    Command::new("cat");
}
