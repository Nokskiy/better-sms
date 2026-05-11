pub mod mutex;

#[cfg(test)]
mod tests {
    use crate::mutex::{MutexCreate, MutexGuardWork, MutexWork};

    #[test]
    fn test_create_mutex() {
        let variable = String::from("variable name");

        let mutex = variable.clone().create_mutex();
        assert_eq!(*mutex.lock_unw(), variable);
    }

    #[test]
    fn test_use_guard() {
        let variable = String::from("variable name");

        let mutex = variable.create_mutex();

        let result_var = String::from("result variable name");
        mutex.lock_unw().use_guard(|guard| {
            *guard = String::from("result variable name");
        });

        assert_eq!(*mutex.lock_unw(), result_var);
    }
}
