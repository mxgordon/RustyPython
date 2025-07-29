use malachite::base::num::conversion::traits::SaturatingFrom;
use malachite::Integer;
use crate::builtins::structure::pyobject::PyObject;

#[derive(Debug)]
pub struct Statics {
    true_: PyObject,
    false_: PyObject,
    
    none_: PyObject,

    ints: [PyObject; 262],
    
    not_implemented: PyObject,
}

impl Statics {
    pub fn new() -> Self {
        Statics {
            true_: PyObject::create_new_bool(true),
            false_: PyObject::create_new_bool(false),
            
            none_: PyObject::create_new_none(),

            ints: PyObject::create_cached_ints(-5, 256).try_into().expect("Int array wasn't sized corrected"),
            
            not_implemented: PyObject::create_new_not_implemented(),
        }
    }
    
    pub fn get_bool(&self, value: bool) -> &PyObject {
        if value {
            &self.true_
        } else {
            &self.false_
        }
    }
    
    pub fn none(&self) -> &PyObject {
        &self.none_
    }
    
    pub fn not_implemented(&self) -> &PyObject {
        &self.not_implemented
    }

    pub fn get_int(&self, value: Integer) -> PyObject {
        if (-5..=256).contains(&value) {
            return self.ints[(i32::saturating_from(&value) + 5) as usize].clone();
        }

        PyObject::new_int_uncached(value)
    }

    pub fn get_int_borrow(&self, value: &Integer) -> PyObject {
        if -5 <= *value && *value <= 256 {
            return self.ints[(i32::saturating_from(value) + 5) as usize].clone();
        }

        PyObject::new_int_uncached(value.clone())
    }
}
