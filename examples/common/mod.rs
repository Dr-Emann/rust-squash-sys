macro_rules! defer {
    ($code:expr) => {
        let _cleanup = {
            struct Cleanup<F: FnMut()>(F);
            impl<F: FnMut()> Drop for Cleanup<F> {
                fn drop(&mut self) {
                    (self.0)();
                }
            }
            Cleanup(move || {
                $code;
            })
        };
    };
}
