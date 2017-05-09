# Rust Wiring

Figuring out ways to test-drive an app.

In Rust, you can't have dangling pointers so if you
have a wiring structure that you want to build up,
you can't just create an empty structure because you
would have dangling component on init.


