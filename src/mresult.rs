#![allow(unused)]

enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
       MResult::Ok(value)
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        matches!(*self, MResult::Ok(_))
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        !self.is_ok()  // seems like cheating but it's either a value or an error so why not?
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        match self {
            MResult::Ok(t) => t,
            MResult::Err(e) => panic!("I'm panicking b/c there's an error.")
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        match self {
            MResult::Err(e) => e,
            MResult::Ok(t) => panic!("I'm panicking b/c there is not an error.")
        }
    }
}

// Add unit tests below
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let x: MResult<u8, &str> = MResult::ok(27);
        assert_eq!(x.is_ok(), true);
        assert_eq!(x.is_err(), false);
        assert_eq!(x.unwrap(), 27);
    }

    #[test]
    fn test_err() {
        let y: MResult<u8, &str> = MResult::err("uh oh");
        assert_eq!(y.is_ok(), false);
        assert_eq!(y.is_err(), true);
        assert_eq!(y.unwrap_err(), "uh oh");
    }
}