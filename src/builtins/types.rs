use std::sync::Arc;
use lazy_static::lazy_static;
use crate::builtins::pyobjects::PyClass;

// lazy_static! {
//     pub static ref object: Arc<PyClass> = Arc::new(PyClass::new_internal_attrs("object", vec!["__new__"], vec![]));
//     pub static ref rangeType: Arc<PyClass> = Arc::new(PyClass::new_internal_attrs("range", vec!["__new__"], vec![]));
// }