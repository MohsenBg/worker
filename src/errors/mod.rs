pub enum ExitCode {
    Success = 0,
    EnvFailed = 1,
    ReadShhPasswordFailed = 2,
    GitNotFound = 3,
    GitAddFailed = 4,
}
