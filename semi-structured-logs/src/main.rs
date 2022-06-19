mod lib;
use lib::structured_logs::{info, warn, log, error, LogLevel};

fn main(){

    emits_error();
    emits_info();
    emits_warning();
    log_emits_info();
    log_emits_warning();
}


fn emits_info() {
    assert_eq!(info("Timezone changed"), "[INFO]: Timezone changed");
}


fn emits_warning() {
    assert_eq!(warn("Timezone not set"), "[WARNING]: Timezone not set");
}


fn emits_error() {
    assert_eq!(error("Disk full"), "[ERROR]: Disk full");
}


fn log_emits_info() {
    assert_eq!(
        log(LogLevel::Info, "Timezone changed"),
        "[INFO]: Timezone changed"
    );
}

fn log_emits_warning() {
    assert_eq!(
        log(LogLevel::Warning, "Timezone not set"),
        "[WARNING]: Timezone not set"
    );
}

fn log_emits_error() {
    assert_eq!(log(LogLevel::Error, "Disk full"), "[ERROR]: Disk full");
}


