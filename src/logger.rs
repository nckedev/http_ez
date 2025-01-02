#[derive(Clone)]
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

    pub fn set_log_level(&mut self, level: Severity) {
        self.log_level = level;
    }

    pub fn log_debug<'a, T: Into<&'a str>>(&self, message: T) {
        self.log(Severity::Debug, message.into());
    }

    pub fn log_trace<'a, T: Into<&'a str>>(&self, message: T) {
        self.log(Severity::Trace, message.into());
    }

    pub fn log_info(&self, message: &str) {
        self.log(Severity::Info, message);
    }
    pub fn log_warning<'a, T: Into<&'a str>>(&self, message: T) {
        self.log(Severity::Warning, message.into());
    }

    pub fn log_error<'a, T: Into<&'a str>>(&self, message: T) {
        self.log(Severity::Error, message.into());
    }

    pub fn log_critical<'a, T: Into<&'a str>>(&self, message: T) {
        self.log(Severity::Critical, message.into());
    }

    fn log(&self, severity: Severity, message: &str) {
        if severity < self.log_level {
            return;
        }

        let t = chrono::Utc::now().to_string();
        let timestamp = format!("[{t}]");

        let prefix = match severity {
            Severity::Debug => "[DEBUG] ",
            Severity::Trace => "[TRACE] ",
            Severity::Info => "[INFO] ",
            Severity::Warning => "[WARNING] ",
            Severity::Error => "[ERROR] ",
            Severity::Critical => "[CRITICAL] ",
        };

        for x in &self.sinks {
            let s = format!("{timestamp}{prefix} -> {message}");
            x.collect(&s);
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Severity {
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
