use std::sync::{Mutex, MutexGuard};

pub trait MutexCreate<T> {
    /// Creates a mutex
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::better_sms::mutex::{MutexWork, MutexCreate};
    ///
    /// let variable = String::from("variable name");
    ///
    /// let mutex = variable.clone().create_mutex();
    /// assert_eq!(*mutex.lock_unw(), variable);
    /// ```
    fn create_mutex(self) -> Mutex<T>;
}

pub trait MutexWork<'a, T> {
    /// Shorthand for .lock().unwarp()
    fn lock_unw(&'a self) -> MutexGuard<'a, T>;
}

pub trait MutexGuardWork<'a, T> {
    /// A feature that allows you to work with MutexGuard immediately
    ///
    /// # Examples
    /// ```
    /// use crate::better_sms::mutex::{MutexWork, MutexCreate, MutexGuardWork};
    ///
    /// let variable = String::from("variable name");
    /// let mutex = variable.create_mutex();
    /// let result_var = String::from("result variable name");
    ///
    /// mutex.lock_unw().use_guard(|guard| {
    ///     *guard = String::from("result variable name");
    /// });
    /// assert_eq!(*mutex.lock_unw(), result_var);
    /// ```
    ///
    /// Or
    ///
    /// ```
    /// use crate::better_sms::mutex::{MutexWork, MutexCreate, MutexGuardWork};
    ///
    /// let variable = String::from("variable name");
    /// let mutex = variable.create_mutex();
    /// let result_var = String::from("result variable name");
    ///
    /// let result = mutex.lock_unw().use_guard(|guard| {
    ///     *guard = String::from("result variable name");
    ///     return guard.to_string();
    /// });
    /// assert_eq!(*mutex.lock_unw(), result_var);
    /// ```
    fn use_guard<O, F: FnOnce(&mut T) -> O>(&mut self, f: F) -> O;
}

impl<T> MutexCreate<T> for T {
    fn create_mutex(self) -> Mutex<T> {
        Mutex::new(self)
    }
}

impl<'a, T> MutexWork<'a, T> for Mutex<T> {
    fn lock_unw(&'a self) -> MutexGuard<'a, T> {
        self.lock().unwrap()
    }
}

impl<'a, T> MutexGuardWork<'a, T> for MutexGuard<'a, T> {
    fn use_guard<O, F: FnOnce(&mut T) -> O>(&mut self, f: F) -> O {
        f(self)
    }
}
