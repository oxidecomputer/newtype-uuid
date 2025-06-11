// Copyright 2025 Oxide Computer Company

//! Handle lists of errors that occur while generating the proc macro.
//!
//! See the documentation of [`ErrorStore`] for more information.

use std::cell::RefCell;

/// Top-level struct that holds all errors encountered during the invocation of
/// a proc macro.
///
/// This allows for collecting errors from multiple sources, and for tracking
/// errors in a hierarchical fashion.
#[derive(Debug)]
pub(crate) struct ErrorStore<T> {
    data: RefCell<ErrorStoreData<T>>,
}

impl<T> ErrorStore<T> {
    /// Create a new `ErrorStore`.
    pub(crate) fn new() -> Self {
        Self {
            data: RefCell::new(ErrorStoreData::default()),
        }
    }

    /// Obtain the list of errors collected by this store.
    ///
    /// This consumes the store, and implies that there are no [`ErrorSink`]
    /// instances that are still alive.
    pub(crate) fn into_inner(self) -> Vec<T> {
        std::mem::take(&mut self.data.borrow_mut().errors)
    }

    /// Create a new sink for collecting errors.
    ///
    /// This is a top-level sink, i.e. it has no parent.
    pub(crate) fn sink(&mut self) -> ErrorSink<'_, T> {
        let new_id = self.data.borrow_mut().register_sink(None);
        ErrorSink {
            data: &self.data,
            id: new_id,
        }
    }
}

/// A collector for errors.
///
/// An `ErrorSink` is a context into which errors can be pushed. It can have
/// child `ErrorSink` instances, and the [`ErrorStore`] from which it is
/// ultimately derived tracks whether any errors were pushed to a given
/// `ErrorSink` or its descendants.
///
/// The lifetime parameter `'a` is the lifetime of the `ErrorStore` that the
/// `ErrorSink` is ultimately derived from. The parameter ensures that
/// `ErrorSink` instances don't outlive the [`ErrorStore`] -- this means that at
/// the time an [`ErrorStore`] is consumed, there aren't any outstanding
/// `ErrorSink` instances.
#[derive(Debug)]
pub(crate) struct ErrorSink<'a, T> {
    // It's a bit weird to use both a lifetime parameter and a RefCell, but it
    // makes sense here. With `Rc<RefCell<T>>`, there's no way to statically
    // guarantee that the error collection process is done. The lifetime
    // parameter statically guarantees that.
    //
    // Do we need interior mutability? Because of our nested structure, the only
    // other alternatives are some kind of `&mut &mut &mut ... T`, or dynamic
    // dispatch. Both seem worse than just doing this.
    data: &'a RefCell<ErrorStoreData<T>>,
    id: usize,
}

impl<'a, T> ErrorSink<'a, T> {
    pub(crate) fn push_critical(&self, error: T) {
        // This is always okay because we only briefly borrow the RefCell at any
        // time.
        self.data.borrow_mut().push_critical(self.id, error);
    }

    #[allow(unused)]
    pub(crate) fn push_warning(&self, error: T) {
        // This is always okay because we only briefly borrow the RefCell at any
        // time.
        self.data.borrow_mut().push_warning(error);
    }

    pub(crate) fn has_critical_errors(&self) -> bool {
        // ErrorStore::push_critical_error propagates `has_critical_errors` up the tree while
        // writing errors, so we can just check the current ID while reading
        // this information.
        self.data.borrow().sinks[self.id].has_critical_errors
    }

    pub(crate) fn new_child(&self) -> ErrorSink<'a, T> {
        let mut errors = self.data.borrow_mut();
        let new_id = errors.register_sink(Some(self.id));
        Self {
            data: self.data,
            id: new_id,
        }
    }
}

#[derive(Debug)]
struct ErrorStoreData<T> {
    errors: Vec<T>,
    sinks: Vec<ErrorSinkData>,
}

impl<T> Default for ErrorStoreData<T> {
    fn default() -> Self {
        Self {
            errors: Vec::new(),
            sinks: Vec::new(),
        }
    }
}

impl<T> ErrorStoreData<T> {
    /// Critical errors block progress
    fn push_critical(&mut self, id: usize, error: T) {
        self.errors.push(error);
        self.sinks[id].has_critical_errors = true;

        // Propagate the fact that errors were encountered up the tree.
        let mut curr = id;
        while let Some(parent) = self.sinks[curr].parent {
            self.sinks[parent].has_critical_errors = true;
            curr = parent;
        }
    }

    /// Warning errors do not block progress
    fn push_warning(&mut self, error: T) {
        self.errors.push(error);
    }

    fn register_sink(&mut self, parent: Option<usize>) -> usize {
        // len is the next ID
        let id = self.sinks.len();
        self.sinks.push(ErrorSinkData::new(parent));
        id
    }
}

#[derive(Debug)]
struct ErrorSinkData {
    // The parent ID in the map.
    parent: Option<usize>,
    // Whether an error was pushed via this specific context or a descendant.
    has_critical_errors: bool,
}

impl ErrorSinkData {
    fn new(parent: Option<usize>) -> Self {
        Self {
            parent,
            has_critical_errors: false,
        }
    }
}
