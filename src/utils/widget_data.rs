//! A safe helper for storing/retreiving data.
//!
//! A thing worth noting is that keys cannot be unregistered
//! if once registered. If this functionality is needed, the
//! `track_widget_cleanup` function can be used.

use gtk4::Widget;
use gtk4::prelude::*;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static WIDGET_DATA: RefCell<HashMap<usize, HashMap<String, Box<dyn Any>>>> = RefCell::new(HashMap::new());
}

pub trait WidgetDataExt {
    fn set_typed_data<T: 'static>(&self, key: &str, value: T);
    fn get_typed_data<T: Clone + 'static>(&self, key: &str) -> Option<T>;

    fn set_flag(&self, key: &str, value: bool) {
        self.set_typed_data(key, value);
    }

    fn get_flag(&self, key: &str) -> Option<bool> {
        self.get_typed_data::<bool>(key)
    }

    fn track_widget_cleanup(&self);
}

impl<O: IsA<Widget>> WidgetDataExt for O {
    fn set_typed_data<T: 'static>(&self, key: &str, value: T) {
        let id = self.as_ptr() as usize;
        WIDGET_DATA.with(|data| {
            let mut data = data.borrow_mut();
            let entry = data.entry(id).or_default();
            entry.insert(key.to_string(), Box::new(value));
        });
    }

    fn get_typed_data<T: Clone + 'static>(&self, key: &str) -> Option<T> {
        let id = self.as_ptr() as usize;
        WIDGET_DATA.with(|data| {
            data.borrow()
                .get(&id)
                .and_then(|map| map.get(key))
                .and_then(|boxed| boxed.downcast_ref::<T>().cloned())
        })
    }

    fn track_widget_cleanup(&self) {
        let id = self.as_ptr() as usize;

        self.connect_destroy(move |_| {
            WIDGET_DATA.with(|data| {
                data.borrow_mut().remove(&id);
            });
        });
    }
}
