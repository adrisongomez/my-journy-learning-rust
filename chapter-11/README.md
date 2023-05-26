# Chapter 11: Testing and Writing Automated Tests

To create a test in rust
```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn should_fail(){
        panic!("Make this test fail")
    }

    #[test]
    fn rectangle_can_hold_method() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };

        assert!(rect1.can_hold(&rect2))
    }
    #[test]
    fn rectangle_cannot_hold() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };

        assert!(!rect2.can_hold(&rect1))
    }
}
```

and run `cargo test`

You can use `#[ignore]` to ignore test.
You can use `cargo test -- --ignore` to run ignore test.
You can use `cargo test <module-name or  test-name>` to filter the test to run

You can use `cargo test --test-threads=1` to run all the tests in a single thread.

You can create a `tests` directory and add a files; every files is a crates so you need to import your library. You don't need to use the `#[cfg(test)]` because cargo knows that you are running integration tests.

If you want to run an specific file inside the tests directory use `cargo test --test <file_name>`
