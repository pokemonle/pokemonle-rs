pub mod prelude;
pub mod request;
pub mod response;

use crate::define_extra_struct;

define_extra_struct!(WithName { name: String });

impl<T> WithName<T> {
    pub fn new(item: T, name: String) -> Self {
        Self { name, item }
    }

    pub fn new_from_tuple(tuple: (T, String)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}
