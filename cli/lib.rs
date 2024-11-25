// extract required items from main.rs

use args::Flags;
pub use deno_runtime::UNSTABLE_GRANULAR_FLAGS;
pub use deno_terminal::colors;
use factory::CliFactory;
use util::display;

pub use deno_config;
pub use deno_graph;
pub use deno_runtime;

pub mod args;
pub mod auth_tokens;
pub mod cache;
pub mod cdp;
pub mod emit;
pub mod errors;
pub mod factory;
pub mod file_fetcher;
pub mod graph_container;
pub mod graph_util;
pub mod http_util;
pub mod js;
pub mod jsr;
pub mod lsp;
pub mod module_loader;
pub mod node;
pub mod npm;
pub mod ops;
pub mod resolver;
pub mod shared;
pub mod standalone;
pub mod task_runner;
pub mod tools;
pub mod tsc;
pub mod util;
pub mod version;
pub mod worker;

#[allow(clippy::print_stderr)]
pub(crate) fn unstable_exit_cb(feature: &str, api_name: &str) {
  eprintln!(
    "Unstable API '{api_name}'. The `--unstable-{}` flag must be provided.",
    feature
  );
  std::process::exit(70);
}
