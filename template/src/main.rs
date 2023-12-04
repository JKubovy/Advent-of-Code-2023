fn first_part(input: &str) -> u32 {
    unimplemented!();
}

fn second_part(input: &str) -> u32 {
    unimplemented!();
}

fn main() {
    let input = include_str!("../../inputs/{{project-name}}.input");
    let first_part = first_part(input);
    println!("First part: {}", first_part);
    let second_part = second_part(input);
    println!("Second part: {}", second_part);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let data = include_str!("../../inputs/{{project-name}}.test");
        let result = first_part(data);
        assert_eq!(result, unimplemented!());
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../../inputs/{{project-name}}.input");
        let result = first_part(data);
        assert_eq!(result, unimplemented!());
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../../inputs/{{project-name}}.test");
        let result = second_part(data);
        assert_eq!(result, unimplemented!());
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../../inputs/{{project-name}}.input");
        let result = second_part(data);
        assert_eq!(result, unimplemented!());
    }
}
