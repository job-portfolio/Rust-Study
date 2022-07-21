impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

/*
Recall that T is the generic type representing the type of the value in the Some variant of an Option. 
That type T is also the return type of the unwrap_or_else function: code that calls unwrap_or_else on 
an Option<String>, for example, will get a String.

Next, notice that the unwrap_or_else function has an additional generic type parameter, F. The F type
is the type of the parameter named f, which is the closure we provide when calling unwrap_or_else.

The trait bound specified on the generic type F is FnOnce() -> T, which means F must be able to be
called at least once, take no arguments, and return a T. Using FnOnce in the trait bound expresses the
constraint that unwrap_or_else is only going to call f at most one time. In the body of unwrap_or_else,
we can see that if the Option is Some, f won’t be called. If the Option is None, f will be called once.
Because all closures implement FnOnce, unwrap_or_else accepts the most different kinds of closures and 
is as flexible as it can be.
*/