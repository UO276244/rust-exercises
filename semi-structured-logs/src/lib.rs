pub mod structured_logs {
    /// various log levels
    #[derive(Clone, PartialEq, Debug)]
    pub enum LogLevel {
        Info,
        Warning,
        Error,
    }
    /// primary function for emitting logs
    pub fn log(level: LogLevel, message: &str) -> String {

        match level {
            LogLevel::Error => error(message),
            LogLevel::Warning => warn(message),
            LogLevel::Info => info(message)
        }

    }
    pub fn info(message: &str) -> String {
        let mut s = String::from("[INFO]: ");
        s.push_str(message);
        s
    }
    pub fn warn(message: &str) -> String {
        let mut s = String::from("[WARNING]: ");
        s.push_str(message);
        s
    }
    pub fn error(message: &str) -> String {
        let mut s = String::from("[ERROR]: ");
        s.push_str(message);
        s
    }

}