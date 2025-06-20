pub fn build<T: Default>() -> T {
    T::default()
}

pub fn build_with<T, F>(initializer: F) -> T
where
    T: Default,
    F: std::ops::FnOnce(&mut T),
{
    let mut item = build::<T>();
    initializer(&mut item);
    item
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Person {
        name: String,
        age: i32,
    }

    #[test]
    fn build_test_it_builds_a_new_instance_with_default_values() {
        let item = build::<Person>();

        assert_eq!(item.name, "");
        assert_eq!(item.age, 0);
    }

    #[test]
    fn build_with_test_initializes_the_new_instance() {
        let item = build_with(|item: &mut Person| {
            item.name = String::from("pidge");
            item.age = 42;
        });

        assert_eq!(item.name, "pidge");
        assert_eq!(item.age, 42);
    }
}
