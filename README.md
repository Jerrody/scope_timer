# `scope_timer` usage
`scope_timer` crate provides very easy way to create a timer and trace the info.

## Examples
```rust
    fn fib(n: u64) -> u64 {
        if n <= 1 {
            return 1;
        }
        
        fib(n - 1) + fib(n - 2)
    }
    
    fn main() {
        let _handle = ScopeTimer::new("my_timer", TimeFormat::SecondsF32(3), None, false);
        println!("{}", fib(46));
    }
```
```
Output:
Label: my_timer | Time: 5.973secs
```

Also, you can do like this:
```rust
    let mut handle = ScopeTimer::default();
    handle.name = "my_timer";
    handle.log_level = Some(LogLevel::Info);
```
or like this:
```rust
    let mut handle = scope_timer::ScopeTimer {
        label: "timer",
        time_format: TimeFormat::SecondsF32(3),
        now: std::time::Instant::now(),
        log_level: None,
        debug_only: true,
    };

    handle.label = "what"
```

## Log Level
`scope_timer` crate has minimal dependencies. You can use log crate for your projects.

For this, you specify log level and
use log **logging implementation** as example `env_logger` crate:

```rust
    env_logger::init();

    let mut handle = ScopeTimer::default();
    handle.name = "my_timer";
    handle.log_level = Some(LogLevel::Info);
```

## Known Issues
You can't do more elegant timer creation that would be correct in work because occurs move which automatically
occurs drop and you get double print of your timer (in time move and  after move).

Like this:
```rust
    let _handle = ScopeTimer {
        name: "my_timer",
        ..Default::default()
    };
```
But PR are welcome. I will try to fix this issue to make timer omega-easy-to-use.
