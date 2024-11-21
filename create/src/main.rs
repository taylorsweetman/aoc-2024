use regex::Regex;
use std::{fs, io::Error, path::Path};

fn get_new_folder_name() -> Result<String, Error> {
    let re = Regex::new(r#"aoc\d\d"#).unwrap();
    let root_dirs = fs::read_dir(".")?;

    let current_cnt = root_dirs
        .map(|d| re.is_match(d.expect("dir error").file_name().to_str().unwrap()))
        .filter(|b| *b)
        .count();

    let new_cnt = current_cnt + 1;
    let suffix = if new_cnt < 10 {
        &(String::from("0") + &new_cnt.to_string())
    } else {
        &new_cnt.to_string()
    };

    Ok(String::from("aoc") + suffix)
}

fn recursive_dir_cp(src: &Path, dest: &Path) -> Result<(), Error> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            recursive_dir_cp(&entry.path(), &dest.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dest.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn update_cargo_files(new_folder_name: &str) -> Result<(), Error> {
    let toml_path = Path::new("./").join(new_folder_name).join("Cargo.toml");
    let lock_path = Path::new("./").join(new_folder_name).join("Cargo.lock");

    let toml_file_contents =
        fs::read_to_string(&toml_path).expect("Something went wrong reading the file");
    let lock_file_contents =
        fs::read_to_string(&lock_path).expect("Something went wrong reading the file");

    let new_toml: String = toml_file_contents.replace(
        r#"name = "template""#,
        format!(r#"name = "{}""#, new_folder_name).as_str(),
    );
    let new_lock: String = lock_file_contents.replace(
        r#"name = "template""#,
        format!(r#"name = "{}""#, new_folder_name).as_str(),
    );

    fs::write(toml_path, new_toml)?;
    fs::write(lock_path, new_lock)
}

fn main() {
    let new_folder_name = get_new_folder_name().unwrap();
    let new_folder_rel_name = "./".to_string() + &new_folder_name;
    println!("new folder name: {}", new_folder_name);

    let src_path = Path::new("./create/template");
    let dest_path = Path::new(&new_folder_rel_name);
    recursive_dir_cp(src_path, dest_path).expect("cp -r failed");
    update_cargo_files(new_folder_name.as_str()).expect("failed to update Cargo files");
}
