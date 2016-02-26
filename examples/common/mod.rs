macro_rules! defer {
    ($code:stmt) => {
        let _cleanup = {
            struct Cleanup<F: FnMut()>(F);
            impl<F: FnMut()> Drop for Cleanup<F> {
                fn drop(&mut self) {
                    (self.0)();
                }
            }
            Cleanup(|| { $code; })
        };
    }
}