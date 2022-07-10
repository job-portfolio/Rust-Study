
/// /// Generate library docs for the following item.
/// //! Generate library docs for the enclosing item.

fn main() {

    /// Documentation for foo module
    mod foo {
    }

    mod bar {
        //! This is documentation for the `bar` module.
        //!
        //! # Examples
    
        // ...
    }
}
