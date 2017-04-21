### Rust as a language

The idea of Rust-lang is to provide safety for system programming.(although its also
a general purpose language). To achieve this the following features are introduced.

#### Ownership

Variable bindings in `Rust` has a special feature `ownership`, which prevents 
unauthorized reuse of variables. 

For example:

```
let v = vec![1,2,3];
let v2 = v;
```

In the above case, by binding `v2` with `v`, we have `transfered` the ownership 
of `vec![1,2,3]` to v2. Therefore any subsequent use of `v` would be illegal.

This requires programmer to explicitly decide whether to `borrow`(by using `&`
to reference) a variable or
`consume` one. 

#### Lifetime

The concept of `Lifetime` of a variable is introduced to avoid problems such as
`dangling pointer`. 

Consider the syntax:

```aidl
	fn rule(self, method: Method, uri: &'static str, view: Box<View>) -> RouterBuilder {
		match self {
			RouterBuilder {regexs: mut r, methods: mut m, views: mut v} => {
				r.push(uri);
				m.push(method);
				v.push(view);

				RouterBuilder {
					regexs: r,
					methods: m,
					views: v,
				}
			}
		}
	}
```

Here the `&'static str` specifies that the str should have `'static` lifetime, which
will survive throughout the program.

```aidl
fn skip_prefix<a', 'b>(line: &a' str, prefix: &b' str) -> &a' str {
}
```

Another case of lifetime usage, where a function takes variables with different lifetime. 

