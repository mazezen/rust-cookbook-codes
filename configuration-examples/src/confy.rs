use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Prefs {
    username: String,
    theme: String,
    autosave: bool,
}

impl Default for Prefs {
    fn default() -> Self {
        Prefs {
            username: "anonymous".to_owned(),
            theme: "dark".to_owned(),
            autosave: true,
        }
    }
}

fn main() -> Result<(), confy::ConfyError> {
    let path = confy::get_configuration_file_path("cookbook_prefs", None)?;
    println!("per-user config path: {}", path.display());

    let scratch = std::env::temp_dir().join("cookbook_prefs.toml");
    let mut prefs: Prefs = confy::load_path(&scratch)?;
    println!("loaded {prefs:?}");

    prefs.theme = "light".to_owned();
    confy::store_path(&scratch, &prefs)?;
    println!("saved {prefs:?} to {}", scratch.display());

    Ok(())
}
