# `scope_timer` usage
`scope_timer` crate provides very easy way to create a timer and trace the info.

## Examples
```rs
    fn fib(n: u64) -> u64 {
        if n <= 1 {
            return 1;
        }
        
        fib(n - 1) + fib(n - 2)
    }
    
    fn main() {
        let _handle = TimerScope::new("my_timer", TimeFormat::SecondsF32(3), None, false);
        println!("{}", fib(46));
    }
```
```
Output:
Label: my_timer | Time: 5.973
```

Also, you can do like this:
```rs
    let mut handle = TimerScope::default();
    handle.name = "my_timer";
    handle.log_level = Some(LogLevel::Info);
```

## Log Level
`scope_timer` crate has minimal dependencies. You can use log crate for your projects.

For this, you specify log level and
use log **logging implementation** as example `env_logger` crate:

```rs
    env_logger::init();

    let mut handle = TimerScope::default();
    handle.name = "my_timer";
    handle.log_level = Some(LogLevel::Info);
```

## Known Issues
You can't do more elegant timer creation that would be correct in work.

Like this:
```rs
    let _handle = TimerScope {
        name: "my_timer",
        ..Default::default()
    };
```
But PR are welcome. I will try to fix this issue to make timer omega-easy-to-use.
