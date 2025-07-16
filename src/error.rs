#[non_exhaustive]
#[cfg(feature = "network")]
#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("Failed to create reqwest client: '{source}'.")]
    ClientBuilderError { source: reqwest::Error },

    #[error("Host name: '{input}' does not match the expected host.\n{message}")]
    HostNotMatch { input: String, message: String },

    #[error("Failed to parse URL: '{url}' because: '{source}'.")]
    ParseURLError {
        url: String,
        source: url::ParseError,
    },

    #[error(
        "Error while making request to: '{url}' using method: '{method}' with error: '{source}'"
    )]
    RequestFailed {
        url: String,
        method: String,
        source: reqwest::Error,
    },
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum IOError {
    #[error("WriteIOError: '{message}'.")]
    WriteError {
        message: String,
        source: std::io::Error,
    },

    #[error("ReadIOError: '{message}'.")]
    ReadError {
        message: String,
        source: std::io::Error,
    },

    #[error("FlushIOError: Failed to flush.")]
    FlushError { source: std::io::Error },

    #[error("OtherIOError: '{message}'.")]
    OtherError {
        message: String,
        source: std::io::Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum IOErrorFile {}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ShawtyError {
    #[error("This is an example error!")]
    ExampleError(),

    #[error("WeirdError: '{message}'.\n")]
    WeirdError { message: String },

    #[cfg(feature = "network")]
    #[error("Failed to send a request.")]
    Request(#[from] RequestError),

    #[error("Operation failed, IO error occurred.")]
    IO(#[from] IOError),

    #[error("Error while executing command: '{command}':\nArgs '{command_args:?}'.")]
    ExecuteCommandError {
        command: String,
        command_args: Vec<String>,
        spawning_or_executing: String,
        error: std::io::Error,
    },
}
