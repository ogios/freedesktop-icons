use crate::theme;
use crate::theme::error::ThemeError::ThemeIndexNotFound;
use dirs::home_dir;
use ini::Ini;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use walkdir::WalkDir;
use xdg::BaseDirectories;

pub(crate) static BASE_PATHS: Lazy<Vec<PathBuf>> = Lazy::new(icon_theme_base_paths);

/// Look in $HOME/.icons (for backwards compatibility), in $XDG_DATA_DIRS/icons and in /usr/share/pixmaps (in that order).
/// Paths that are not found are filtered out.
fn icon_theme_base_paths() -> Vec<PathBuf> {
    let home_icon_dir = home_dir().expect("No $HOME directory").join(".icons");
    let mut data_dirs: Vec<_> = BaseDirectories::new()
        .map(|bd| {
            bd.get_data_dirs()
                .into_iter()
                .map(|p| p.join("icons"))
                .collect()
        })
        .unwrap_or_default();
    data_dirs.push(home_icon_dir);
    data_dirs.into_iter().filter(|p| p.exists()).collect()
}

#[derive(Debug)]
pub struct ThemePath(pub PathBuf);

impl ThemePath {
    pub(super) fn index(&self, greedy: bool) -> theme::Result<Ini> {
        let index = self.0.join("index.theme");

        if !index.exists() {
            match greedy {
                true => return Ok(self.generate_index()),
                false => Err(ThemeIndexNotFound(self.0.clone()))?,
            }
        }

        Ok(Ini::load_from_file(index)?)
    }

    fn generate_index(&self) -> Ini {
        let mut index = Ini::new();

        let sizes_dirs: Vec<PathBuf> = WalkDir::new(&self.0)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.into_path())
            .collect();

        let mut app_dirs: Vec<String> = vec![];

        for size in sizes_dirs {
            let mut paths: Vec<String> = WalkDir::new(&size)
                .into_iter()
                .filter_map(|e| e.ok())
                .map(|e| e.into_path())
                .filter(|p| p.is_dir())
                .collect::<Vec<PathBuf>>()
                .iter()
                .filter_map(|p| p.strip_prefix(&self.0).ok())
                .filter_map(|p| p.to_str())
                .map(|p| p.to_owned())
                .collect();

            let size_num: usize = size
                .to_str()
                .unwrap()
                .split('x')
                .next()
                .unwrap_or("128")
                .parse()
                .unwrap_or(128);

            for path in paths.clone() {
                index.set_to(Some(path), "Size".to_owned(), size_num.to_string())
            }

            app_dirs.append(&mut paths);
        }

        index.set_to(
            Some("Icon Theme"),
            "Directories".to_owned(),
            app_dirs.join(",").to_owned(),
        );

        index
    }
}

#[cfg(test)]
mod test {
    use crate::theme::paths::icon_theme_base_paths;
    use crate::theme::{get_all_themes, Theme};
    use anyhow::Result;
    use speculoos::prelude::*;

    #[test]
    fn should_get_all_themes() {
        let themes = get_all_themes(false).unwrap();
        assert_that!(themes.get("hicolor")).is_some();
    }

    #[test]
    fn should_get_theme_paths_ordered() {
        let base_paths = icon_theme_base_paths();
        assert_that!(base_paths).is_not_empty()
    }

    #[test]
    fn should_read_theme_index() -> Result<()> {
        let themes = get_all_themes(false)?;
        let themes: Vec<&Theme> = themes.values().flatten().collect();
        assert_that!(themes).is_not_empty();
        Ok(())
    }
}
