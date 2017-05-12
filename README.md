# Rust Wiring

Figuring out ways to test-drive an app. Experimenting with ways to wire collaborators
in an easy-to-maintain way.

----

First attempt was to use a structure of trait references. That got fairly
complicated because of ownership and sized constraints

----

Discovered something: don't need references as config members if I just parameterize the config on its actual types and use type constraints to enfore the implemented traits.  Using move semantics, I can have `with_whatever` methods that replace the current type with the new type for each wiring field.

Unfortunately, I can't use update syntax so as the number of fields grows, the verbosity of the `with_whatever` function implementations also grows. Need to solve that problem next. If I do, then I think this is a pretty good mechanism.

Maybe macros can alleviate the lack of update syntax.
