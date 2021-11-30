#[macro_use]
extern crate log;

pub enum TimeFormat {
	SecondsF32(usize),
	SecondsF64(usize),
	Milliseconds,
	Microseconds,
	Nanoseconds,
}

pub enum LogLevel {
	Error,
	Warning,
	Info,
	Trace,
	Debug,
}

pub struct TimerScope<'a> {
	pub name: &'a str,
	pub time_format: TimeFormat,
	pub now: std::time::Instant,
	pub log_level: Option<LogLevel>,
	pub debug_only: bool,
}

impl<'a> TimerScope<'a> {
	pub fn new(name: &'a str, time_format: TimeFormat, log_level: Option<LogLevel>, debug_only: bool) -> Self {
		Self {
			name,
			time_format,
			now: std::time::Instant::now(),
			log_level,
			debug_only: if debug_only {
				!cfg!(debug_assertions)
			} else {
				false
			},
		}
	}

	pub fn elapsed_time(&self) {
		match self.debug_only {
			false => match &self.log_level {
				None => match self.time_format {
					TimeFormat::SecondsF32(precision) => println!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f32(), precision = precision),
					TimeFormat::SecondsF64(precision) => println!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f64(), precision = precision),
					TimeFormat::Milliseconds => println!("Label: {} | Time: {}", self.name, self.now.elapsed().as_millis()),
					TimeFormat::Microseconds => println!("Label: {} | Time: {}", self.name, self.now.elapsed().as_micros()),
					TimeFormat::Nanoseconds => println!("Label: {} | Time: {}", self.name, self.now.elapsed().as_nanos()),
				},
				Some(debug_mode) => match debug_mode {
					LogLevel::Error => {
						match self.time_format {
							TimeFormat::SecondsF32(precision) => error!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f32(), precision = precision),
							TimeFormat::SecondsF64(precision) => error!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f64(), precision = precision),
							TimeFormat::Milliseconds => error!("Label: {} | Time: {}", self.name, self.now.elapsed().as_millis()),
							TimeFormat::Microseconds => error!("Label: {} | Time: {}", self.name, self.now.elapsed().as_micros()),
							TimeFormat::Nanoseconds => error!("Label: {} | Time: {}", self.name, self.now.elapsed().as_nanos()),
						}
					}
					LogLevel::Warning => {
						match self.time_format {
							TimeFormat::SecondsF32(precision) => warn!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f32(), precision = precision),
							TimeFormat::SecondsF64(precision) => warn!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f64(), precision = precision),
							TimeFormat::Milliseconds => warn!("Label: {} | Time: {}", self.name, self.now.elapsed().as_millis()),
							TimeFormat::Microseconds => warn!("Label: {} | Time: {}", self.name, self.now.elapsed().as_micros()),
							TimeFormat::Nanoseconds => warn!("Label: {} | Time: {}", self.name, self.now.elapsed().as_nanos()),
						}
					}
					LogLevel::Info => {
						match self.time_format {
							TimeFormat::SecondsF32(precision) => info!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f32(), precision = precision),
							TimeFormat::SecondsF64(precision) => info!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f64(), precision = precision),
							TimeFormat::Milliseconds => info!("Label: {} | Time: {}", self.name, self.now.elapsed().as_millis()),
							TimeFormat::Microseconds => info!("Label: {} | Time: {}", self.name, self.now.elapsed().as_micros()),
							TimeFormat::Nanoseconds => info!("Label: {} | Time: {}", self.name, self.now.elapsed().as_nanos()),
						}
					}
					LogLevel::Debug => {
						match self.time_format {
							TimeFormat::SecondsF32(precision) => debug!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f32(), precision = precision),
							TimeFormat::SecondsF64(precision) => debug!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f64(), precision = precision),
							TimeFormat::Milliseconds => debug!("Label: {} | Time: {}", self.name, self.now.elapsed().as_millis()),
							TimeFormat::Microseconds => debug!("Label: {} | Time: {}", self.name, self.now.elapsed().as_micros()),
							TimeFormat::Nanoseconds => debug!("Label: {} | Time: {}", self.name, self.now.elapsed().as_nanos()),
						}
					}
					LogLevel::Trace => {
						match self.time_format {
							TimeFormat::SecondsF32(precision) => trace!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f32(), precision = precision),
							TimeFormat::SecondsF64(precision) => trace!("Label: {} | Time: {:.precision$}", self.name, self.now.elapsed().as_secs_f64(), precision = precision),
							TimeFormat::Milliseconds => trace!("Label: {} | Time: {}", self.name, self.now.elapsed().as_millis()),
							TimeFormat::Microseconds => trace!("Label: {} | Time: {}", self.name, self.now.elapsed().as_micros()),
							TimeFormat::Nanoseconds => trace!("Label: {} | Time: {}", self.name, self.now.elapsed().as_nanos()),
						}
					}
				}
			},
			true => return,
		}
	}
}

impl<'a> Default for TimerScope<'a> {
	fn default() -> Self {
		Self {
			name: "timer",
			time_format: TimeFormat::SecondsF32(3),
			now: std::time::Instant::now(),
			log_level: Default::default(),
			debug_only: Default::default(),
		}
	}
}

impl<'a> Drop for TimerScope<'a> {
	fn drop(&mut self) {
		self.elapsed_time();
	}
}
