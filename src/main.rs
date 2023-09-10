use anyhow::Result;
use shallot::*;
use shallot_strings::*;

fn main() -> Result<()> {
    let mut environment: Environment<StringyExpression> = Environment::default();
    shallot::builtins::set_environment(&mut environment);
    shallot_strings::set_environment(&mut environment);
    run_repl::<StringyExpression>(&mut environment)
}
