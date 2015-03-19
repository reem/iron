use prelude::*;

/// `Handler`s are responsible for handling requests by creating `Response`s from `Request`s.
pub trait Handler: Send + Sync + 'static {
    /// Produce a `Response` from a Request, with the possibility of error.
    fn handle(&self, &mut Request) -> IronResult<Response>;
}

/// A trait used in generics for converting to `Handler`s.
///
/// This is useful because it allows APIs which take `Handler`s to accept more
/// types without adding specific implementations of `Handler` to standard types.
///
/// In particular, types which are not `Send`, `Sync`, or `'static` cannot
/// implement `Handler`, but can implement `IntoHandler`.
///
/// See `Iron::new` for an idiomatic use of this trait to build a flexible API.
pub trait IntoHandler {
    type Output: Handler;

    /// Convert `Self` into a `Handler`.
    fn into_handler(self) -> Self::Output;
}

impl<H: Handler> IntoHandler for H {
    type Output = H;

    fn into_handler(self) -> H { self }
}

impl<I: IntoHandler> IntoHandler for (Vec<Box<BeforeMiddleware>>, I, Vec<Box<AfterMiddleware>>) {
    type Output = Chain;

    fn into_handler(self) -> Chain {

    }
}

impl<I: IntoHandler> IntoHandler for (Vec<Box<BeforeMiddleware>>, I) {
    type Output = Chain;

    fn into_handler(self) -> Chain {
        (self.0, self.1, vec![])
    }
}

impl<I: IntoHandler> IntoHandler for (I, Vec<Box<AfterMiddleware>>) {
    type Output = Chain;

    fn into_handler(self) -> Chain {
        (vec![], self.0, self.1)
    }
}

impl<F> Handler for F
where F: Send + Sync + 'static + Fn(&mut Request) -> IronResult<Response> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        (*self)(req)
    }
}

impl Handler for Box<Handler> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        (**self).handle(req)
    }
}

