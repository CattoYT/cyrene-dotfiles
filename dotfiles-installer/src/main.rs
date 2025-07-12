use std::fs::exists;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::path::PathBuf;
// if i include all of my dotfile directories here then i just install each as required

#[cfg(target_os = "macos")]
fn install() {
    let dotfile_path: PathBuf = std::env::current_dir().unwrap();
    let config_path: PathBuf = std::env::home_dir().unwrap();
    create_linked_dotfile(&dotfile_path, &config_path, "yabai".to_string());
}

fn create_linked_dotfile(dotfile_path: &Path, config_path: &Path, foldername: String) {
    if (!exists(dotfile_path.join(&foldername)).unwrap()) {
        todo!("manage non existant configs cuz its probably being cooked");
    }
    println!("{}", &dotfile_path.display());
    match symlink(
        dotfile_path.join(&foldername),
        config_path.join(".config").join(&foldername),
    ) {
        Ok(_) => {
            println!("{} symlink created", &foldername);
        }
        Err(_) => {
            println!("symlink failed, continue with caution");
        }
    }
}

fn install_common() {
    let dotfile_path: PathBuf = std::env::current_dir().unwrap();
    let config_path: PathBuf = std::env::home_dir().unwrap();

    create_linked_dotfile(&dotfile_path, &config_path, "nvim".to_string());
    create_linked_dotfile(&dotfile_path, &config_path, "kitty".to_string());
    create_linked_dotfile(&dotfile_path, &config_path, "cava".to_string());

    create_linked_dotfile(&dotfile_path, &config_path, "karabiner".to_string());

    //zshrc
    match symlink(&dotfile_path.join(".zshrc"), &config_path.join(".zshrc")) {
        Ok(_) => {
            println!("symlink created");
        }
        Err(_) => {
            println!("symlink failed, continue with caution");
        }
    }
}

fn main() {
    println!("Welcome to the installer");
    install_common();

    //install();
}
