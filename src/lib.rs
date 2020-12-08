mod params;

pub use params::Params;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const LOGO: &str = r"
 _      _
| |    | |
| |_   | |   __
| | |  | |  |  |
| _ |  |_|  |  |
Black Water
Asynchronous Port Scanner written in rust

";