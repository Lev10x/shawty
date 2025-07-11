#[derive(Debug, thiserror::Error)]
pub enum ShawtyError {
    #[error("This is an example error!")]
    ExampleError(),

    #[error("Failed to write: '{data}':\n\tCause by: '{source}'\n")]
    WriteIOError {
        data: String,
        source: std::io::Error,
    },

    #[error("Failed to read: '{data}':\n\tCause by: '{source}'\n")]
    ReadIOError {
        data: String,
        source: std::io::Error,
    },

    #[error("Failed to fluse:\n\tCause by: '{source}'\n")]
    FlushIOError { source: std::io::Error },

    #[error("IO Error: '{message}':\n\tCause by: '{source}'\n")]
    OtherIOError {
        message: String,
        source: std::io::Error,
    },

    #[cfg(feature = "network")]
    #[error(
        "Error while making request to: '{url}' with method: '{method}':\n\tCause by: '{source}'\n"
    )]
    RequestError {
        url: String,
        method: String,
        source: reqwest::Error,
    },

    #[cfg(feature = "network")]
    #[error("Client Builder error: '{source}'.")]
    ClientBuilderError { source: reqwest::Error },

    #[error("Error while asking for input:\n\tCause by: '{0}'\n.")]
    GetInputError(std::io::Error),

    #[error("Error while getting cwd:\n\tCause by: '{0}'\n.")]
    GetCWDError(std::io::Error),

    #[error(
        "Error while printting debug message (Failed to write to stdout):\n\tCause by: '{0}'\n."
    )]
    DebugPrintIOError(std::io::Error),

    #[cfg(feature = "network")]
    #[error("Failed to parse URL: '{url}'\n\tCause by: '{source}'\n.")]
    ParseURLError {
        url: String,
        source: url::ParseError,
    },

    #[cfg(feature = "network")]
    #[error("Host name: '{input}' does not match the expected host.\n{message}")]
    HostNotMatch { input: String, message: String },

    #[error(
        "Error while executing command: '{command}' with args '{command_args:?}':\n\tCause by: '{error}'\n."
    )]
    ExecuteCommandError {
        command: String,
        command_args: Vec<String>,
        error: std::io::Error,
    },
}
