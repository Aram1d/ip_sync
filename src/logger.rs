use env_logger::{Builder, Env, Target};

pub fn init_logger() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .target(Target::Stdout)
        .init();
}
