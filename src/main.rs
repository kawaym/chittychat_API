mod utils;
mod tests;

fn main() {
    let env: utils::load_envs::EnvFile = utils::load_envs::load();
}
