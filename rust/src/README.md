## parameters.rs

```sh
echo -en "#[rustfmt::skip]\npub mod parameters {\n    #[allow(dead_code)]\n    pub const PATTERN_INSTANCES: [f32; 167_265] = [" > parameters.rs
cat ../../python/data/parameters/0925.txt >> parameters.rs
echo -e "];\n}" >> parameters.rs
```

## Debugging
```rs
extern crate console_error_panic_hook;
use std::panic;

panic::set_hook(Box::new(console_error_panic_hook::hook));
```
