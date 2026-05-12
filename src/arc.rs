use std::sync::Arc;

pub trait ArcCreate<T> {
    /// Creates a arc
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::better_sms::{arc::ArcCreate, mutex::{MutexWork, MutexCreate}};
    ///
    /// let variable = String::from("variable name");
    ///
    /// let arc_mutex = variable.clone().create_mutex().create_arc();
    /// assert_eq!(*arc_mutex.lock_unw(), variable);
    /// ```
    fn create_arc(self) -> Arc<T>;
}

impl<T> ArcCreate<T> for T {
    fn create_arc(self) -> Arc<T> {
        Arc::new(self)
    }
}
