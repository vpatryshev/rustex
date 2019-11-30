use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
  let v = vec![4, 8, 19, 27, 34, 10];
  let r = &v;
  let aside = v;  // move vector to aside
  r[0];

  let args: Vec<String> = env::args().collect();

  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1)
  });

  if let Err(e) = run(config) {
    eprintln!("Application error: {}", e);

    process::exit(1);
  }
}
