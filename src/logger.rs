pub struct Logger<'sink> {
    sinks: Vec<Box<&'sink dyn LogSink>>,
    log_level: Severity,
}

impl<'sink> Logger<'sink> {
    pub fn new() -> Self {
        let mut logger = Logger {
            sinks: vec![],
            log_level: Severity::default(),
        };
        logger.add_sink(&DefaultSink);
        logger
    }

    fn add_sink(&mut self, sink: &'sink impl LogSink) {
        self.sinks.push(Box::new(sink));
    }

    fn set_log_level(&mut self, level: Severity) {
        self.log_level = level;
    }

    pub fn log_info(&self, message: &str) {
        for sink in &self.sinks {
            sink.collect(&("LOG INFO -> ".to_string() + message));
        }
    }

    pub fn log_trace(&mut self, message: &impl Into<String>) {}

    fn log(&self, severity: Severity, message: &str) {
        for x in &self.sinks {
            x.collect(&message);
        }
    }
}

impl<'sink> Default for Logger<'sink> {
    fn default() -> Self {
        let mut logger = Self {
            sinks: vec![],
            log_level: Severity::Info,
        };
        logger.add_sink(&DefaultSink);
        logger
    }
}

trait LogSink {
    fn collect(&self, message: &str);
}

struct LogEntry {
    severity: Severity,
    message: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
enum Severity {
    Debug,
    Trace,
    #[default]
    Info,
    Warning,
    Error,
    Critical,
}

struct DefaultSink;

impl LogSink for DefaultSink {
    fn collect(&self, message: &str) {
        println!("{}", &message);
    }
}
