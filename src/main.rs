fn main() {
    println!("{}", println(String::from("Hello, world!")));
}

fn println(s: String) -> String {
    s
}

#[cfg(test)]
mod test {
    use crate::println;

    #[test]
    fn working() {
        assert_eq!("Hello, world!", println(String::from("Hello, world!")));
    }
}
