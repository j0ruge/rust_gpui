use rust_gpui::config::resolve_data_dir;
use std::path::{Path, PathBuf};

#[test]
fn absolute_override_wins_and_relative_override_fails() {
    assert_eq!(
        resolve_data_dir(
            Some(Path::new("/tmp/tasks")),
            Some(Path::new("/xdg")),
            Some(Path::new("/home/test"))
        )
        .unwrap(),
        PathBuf::from("/tmp/tasks")
    );
    for path in ["relative", ""] {
        assert!(
            resolve_data_dir(
                Some(Path::new(path)),
                Some(Path::new("/xdg")),
                Some(Path::new("/home/test"))
            )
            .is_err()
        );
    }
}

#[test]
fn absolute_xdg_wins_over_home() {
    assert_eq!(
        resolve_data_dir(None, Some(Path::new("/xdg")), Some(Path::new("/home/test"))).unwrap(),
        PathBuf::from("/xdg/rust-gpui")
    );
    assert_eq!(
        resolve_data_dir(None, Some(Path::new("/xdg")), None).unwrap(),
        PathBuf::from("/xdg/rust-gpui")
    );
}

#[test]
fn invalid_xdg_falls_back_to_absolute_home() {
    for xdg in [None, Some(Path::new("relative")), Some(Path::new(""))] {
        assert_eq!(
            resolve_data_dir(None, xdg, Some(Path::new("/home/test"))).unwrap(),
            PathBuf::from("/home/test/.local/share/rust-gpui")
        );
    }
}

#[test]
fn refuses_missing_or_relative_home_without_other_valid_path() {
    for home in [None, Some(Path::new("relative")), Some(Path::new(""))] {
        assert!(resolve_data_dir(None, None, home).is_err());
    }
}
