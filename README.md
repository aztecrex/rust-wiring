# Rust Wiring

Figuring out ways to test-drive an app.

In Rust, you can't have dangling pointers so if you
have a wiring structure that you want to build up,
you can't just create an empty structure because you
would have dangling component on init.

----

Discovered something: don't need references as config members if I just parameterize the config on its actual types and use type constraints to enfore the implemented traits.  Using move semantics, I can have `with_whatever` methods that replace the current type with the new type for each wiring field.

Unfortunately, I can't use update syntax so as the number of fields grows, the verbosity of the `with_whatever` function implementations also grows. Need to solve that problem next. If I do, then I think this is a pretty good mechanism.


