use std::process::{Command, Stdio};

pub fn is_program_in_path(program: &str) -> bool {
    return match Command::new(program).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
        Ok(_) => true,
        Err(_) => false, 
    };
}

pub fn is_program_not_in_path(program: &str) -> bool {
    !is_program_in_path(program)
}

#[cfg(test)]
mod tests {
    use crate::utils::command::{is_program_in_path, is_program_not_in_path};

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