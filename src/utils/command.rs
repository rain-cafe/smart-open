use std::{env, fs, path::Path};

pub fn get_path() -> Vec<String> {
    let path = env::var("PATH").unwrap_or_default();

    let split = if cfg!(target_family = "windows") { path.split(";") } else { path.split(":") };

    split.map(|x| x.to_string()).collect()
}

pub fn is_program_in_path(program: &str) -> bool {
    for p in get_path() {
        let buf = Path::new(&p).join(program);
        let p_str = buf.to_str().expect("Failed to join");
        if fs::metadata(p_str).is_ok() {
            return true;
        }
    }
    false
}

pub fn is_program_not_in_path(program: &str) -> bool {
    !is_program_in_path(program)
}

#[cfg(test)]
mod tests {
    use crate::utils::command::{get_path, is_program_in_path, is_program_not_in_path};

    #[test]
    fn get_path_should_parse_the_path() {
        assert!(get_path().len() > 0);
    }

    #[test]
    fn is_program_in_path_should_return_true_if_program_exists() {
        assert_eq!(is_program_in_path("cargo"), true);
    }

    #[test]
    fn is_program_in_path_should_return_false_if_program_does_not_exists() {
        assert_eq!(is_program_in_path("bogus-cli"), false);
    }

    #[test]
    fn is_program_not_in_path_should_return_false_if_program_exists() {
        assert_eq!(is_program_not_in_path("cargo"), false);
    }

    #[test]
    fn is_program_not_in_path_should_return_true_if_program_does_not_exists() {
        assert_eq!(is_program_not_in_path("bogus-cli"), true);
    }
}