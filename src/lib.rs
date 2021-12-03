#[macro_use]
extern crate log;

pub enum TimeFormat {
	Seconds,
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

pub struct ScopeTimer<'a> {
	pub label: &'a str,
	pub time_format: TimeFormat,
	pub now: std::time::Instant,
	pub log_level: Option<LogLevel>,
	pub debug_only: bool,
}

impl<'a> ScopeTimer<'a> {
	pub fn new(label: &'a str, time_format: TimeFormat, log_level: Option<LogLevel>, debug_only: bool) -> Self {
		Self {
			label,
			time_format,
			now: std::time::Instant::now(),
			log_level,
			debug_only,
		}
	}

	pub fn elapsed_time(&self) {
		match self.debug_only {
			false => self.print_timer_info(),
			true if cfg!(debug_assertions) => self.print_timer_info(),
			_ => (),
		}
	}

	fn print_timer_info(&self) {
		match &self.log_level {
			None => match self.time_format {
				TimeFormat::Seconds => println!("Label: {} | Time: {}secs", self.label, self.now.elapsed().as_secs()),
				TimeFormat::SecondsF32(precision) => println!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f32(), precision = precision),
				TimeFormat::SecondsF64(precision) => println!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f64(), precision = precision),
				TimeFormat::Milliseconds => println!("Label: {} | Time: {}ms", self.label, self.now.elapsed().as_millis()),
				TimeFormat::Microseconds => println!("Label: {} | Time: {}μs", self.label, self.now.elapsed().as_micros()),
				TimeFormat::Nanoseconds => println!("Label: {} | Time: {}ns", self.label, self.now.elapsed().as_nanos()),
			},
			Some(debug_mode) => match debug_mode {
				LogLevel::Error => {
					match self.time_format {
						TimeFormat::Seconds => error!("Label: {} | Time: {}secs", self.label, self.now.elapsed().as_secs()),
						TimeFormat::SecondsF32(precision) => error!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f32(), precision = precision),
						TimeFormat::SecondsF64(precision) => error!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f64(), precision = precision),
						TimeFormat::Milliseconds => error!("Label: {} | Time: {}ms", self.label, self.now.elapsed().as_millis()),
						TimeFormat::Microseconds => error!("Label: {} | Time: {}μs", self.label, self.now.elapsed().as_micros()),
						TimeFormat::Nanoseconds => error!("Label: {} | Time: {}ns", self.label, self.now.elapsed().as_nanos()),
					}
				}
				LogLevel::Warning => {
					match self.time_format {
						TimeFormat::Seconds => warn!("Label: {} | Time: {}secs", self.label, self.now.elapsed().as_secs()),
						TimeFormat::SecondsF32(precision) => warn!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f32(), precision = precision),
						TimeFormat::SecondsF64(precision) => warn!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f64(), precision = precision),
						TimeFormat::Milliseconds => warn!("Label: {} | Time: {}ms", self.label, self.now.elapsed().as_millis()),
						TimeFormat::Microseconds => warn!("Label: {} | Time: {}μs", self.label, self.now.elapsed().as_micros()),
						TimeFormat::Nanoseconds => warn!("Label: {} | Time: {}ns", self.label, self.now.elapsed().as_nanos()),
					}
				}
				LogLevel::Info => {
					match self.time_format {
						TimeFormat::Seconds => info!("Label: {} | Time: {}secs", self.label, self.now.elapsed().as_secs()),
						TimeFormat::SecondsF32(precision) => info!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f32(), precision = precision),
						TimeFormat::SecondsF64(precision) => info!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f64(), precision = precision),
						TimeFormat::Milliseconds => info!("Label: {} | Time: {}ms", self.label, self.now.elapsed().as_millis()),
						TimeFormat::Microseconds => info!("Label: {} | Time: {}μs", self.label, self.now.elapsed().as_micros()),
						TimeFormat::Nanoseconds => info!("Label: {} | Time: {}ns", self.label, self.now.elapsed().as_nanos()),
					}
				}
				LogLevel::Debug => {
					match self.time_format {
						TimeFormat::Seconds => debug!("Label: {} | Time: {}secs", self.label, self.now.elapsed().as_secs()),
						TimeFormat::SecondsF32(precision) => debug!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f32(), precision = precision),
						TimeFormat::SecondsF64(precision) => debug!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f64(), precision = precision),
						TimeFormat::Milliseconds => debug!("Label: {} | Time: {}ms", self.label, self.now.elapsed().as_millis()),
						TimeFormat::Microseconds => debug!("Label: {} | Time: {}μs", self.label, self.now.elapsed().as_micros()),
						TimeFormat::Nanoseconds => debug!("Label: {} | Time: {}ns", self.label, self.now.elapsed().as_nanos()),
					}
				}
				LogLevel::Trace => {
					match self.time_format {
						TimeFormat::Seconds => trace!("Label: {} | Time: {}secs", self.label, self.now.elapsed().as_secs()),
						TimeFormat::SecondsF32(precision) => trace!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f32(), precision = precision),
						TimeFormat::SecondsF64(precision) => trace!("Label: {} | Time: {:.precision$}secs", self.label, self.now.elapsed().as_secs_f64(), precision = precision),
						TimeFormat::Milliseconds => trace!("Label: {} | Time: {}ms", self.label, self.now.elapsed().as_millis()),
						TimeFormat::Microseconds => trace!("Label: {} | Time: {}μs", self.label, self.now.elapsed().as_micros()),
						TimeFormat::Nanoseconds => trace!("Label: {} | Time: {}ns", self.label, self.now.elapsed().as_nanos()),
					}
				}
			}
		}
	}
}

impl<'a> Default for ScopeTimer<'a> {
	fn default() -> Self {
		Self {
			label: "timer",
			time_format: TimeFormat::SecondsF32(3),
			now: std::time::Instant::now(),
			log_level: Default::default(),
			debug_only: Default::default(),
		}
	}
}

impl<'a> Drop for ScopeTimer<'a> {
	fn drop(&mut self) {
		self.elapsed_time();
	}
}
