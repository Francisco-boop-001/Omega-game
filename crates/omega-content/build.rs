use std::fs;
use std::path::{Path, PathBuf};

const MAX_MAP_DIMENSION: usize = 256;
const LEGACY_MAP_VERSION: &str = "v2";

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir not set"));
    let map_dir = root.join("..").join("..").join("tools").join("libsrc");
    println!("cargo:rerun-if-changed={}", map_dir.display());

    let Ok(entries) = fs::read_dir(&map_dir) else {
        println!(
            "cargo:warning=omega-content: skipped content check, map directory missing: {}",
            map_dir.display()
        );
        return;
    };

    let mut map_paths: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("map"))
        .collect();
    map_paths.sort();

    if map_paths.is_empty() {
        panic!("omega-content build check failed: no .map files found in {}", map_dir.display());
    }

    for path in map_paths {
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed reading {}: {err}", path.display()));
        validate_legacy_map_structure(&content, &path);
    }
}

fn validate_legacy_map_structure(raw: &str, path: &Path) {
    let mut lines = raw.lines();
    let Some(version_line) = lines.next() else {
        panic!("{}: empty file", path.display());
    };
    assert!(
        version_line == LEGACY_MAP_VERSION,
        "{}: unsupported version header `{}` (expected `{}`)",
        path.display(),
        version_line,
        LEGACY_MAP_VERSION
    );

    let Some(map_header) = lines.next() else {
        panic!("{}: missing map header", path.display());
    };
    assert!(
        map_header.starts_with("map "),
        "{}: malformed map header `{}`",
        path.display(),
        map_header
    );

    let Some(dim_header) = lines.next() else {
        panic!("{}: missing dimension header", path.display());
    };
    let mut parts = dim_header.split_whitespace();
    let depth: usize = parts
        .next()
        .unwrap_or("")
        .parse()
        .unwrap_or_else(|_| panic!("{}: invalid depth in `{}`", path.display(), dim_header));
    let wh = parts.next().unwrap_or("");
    let (w, h) = wh
        .split_once(',')
        .unwrap_or_else(|| panic!("{}: invalid dimensions in `{}`", path.display(), dim_header));
    let width: usize = w
        .parse()
        .unwrap_or_else(|_| panic!("{}: invalid width in `{}`", path.display(), dim_header));
    let height: usize = h
        .parse()
        .unwrap_or_else(|_| panic!("{}: invalid height in `{}`", path.display(), dim_header));

    assert!(
        (1..=MAX_MAP_DIMENSION).contains(&depth),
        "{}: depth {} out of range 1..={}",
        path.display(),
        depth,
        MAX_MAP_DIMENSION
    );
    assert!(
        (1..=MAX_MAP_DIMENSION).contains(&width),
        "{}: width {} out of range 1..={}",
        path.display(),
        width,
        MAX_MAP_DIMENSION
    );
    assert!(
        (1..=MAX_MAP_DIMENSION).contains(&height),
        "{}: height {} out of range 1..={}",
        path.display(),
        height,
        MAX_MAP_DIMENSION
    );

    for level in 0..depth {
        for row in 0..height {
            let Some(map_row) = lines.next() else {
                panic!(
                    "{}: unexpected EOF while reading level {} row {}",
                    path.display(),
                    level + 1,
                    row + 1
                );
            };
            assert!(
                map_row.chars().count() == width,
                "{}: level {} row {} has width {}, expected {}",
                path.display(),
                level + 1,
                row + 1,
                map_row.chars().count(),
                width
            );
        }

        let Some(separator) = lines.next() else {
            panic!("{}: missing level separator after level {}", path.display(), level + 1);
        };
        assert!(
            separator.starts_with('='),
            "{}: invalid level separator `{}` after level {}",
            path.display(),
            separator,
            level + 1
        );
    }
}
