use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "pkgx",
            "install",
            &format!("opentofu.org@{}", version),
        ])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn init(args: String) -> FnResult<String> {
    let version = dag().get_env("OPENTOFU_VERSION")?;
    match version.is_empty() {
        true => dag().set_envs(vec![("OPENTOFU_VERSION".into(), "latest".into())])?,
        false => dag().set_envs(vec![("OPENTOFU_VERSION".into(), version)])?,
    }

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "tofu@$OPENTOFU_VERSION", "init", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn validate(args: String) -> FnResult<String> {
    let version = dag().get_env("OPENTOFU_VERSION")?;
    match version.is_empty() {
        true => dag().set_envs(vec![("OPENTOFU_VERSION".into(), "latest".into())])?,
        false => dag().set_envs(vec![("OPENTOFU_VERSION".into(), version)])?,
    }

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "tofu@$OPENTOFU_VERSION", "validate", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn plan(args: String) -> FnResult<String> {
    let version = dag().get_env("OPENTOFU_VERSION")?;
    match version.is_empty() {
        true => dag().set_envs(vec![("OPENTOFU_VERSION".into(), "latest".into())])?,
        false => dag().set_envs(vec![("OPENTOFU_VERSION".into(), version)])?,
    }

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "tofu@$OPENTOFU_VERSION", "plan", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn apply(args: String) -> FnResult<String> {
    let version = dag().get_env("OPENTOFU_VERSION")?;
    match version.is_empty() {
        true => dag().set_envs(vec![("OPENTOFU_VERSION".into(), "latest".into())])?,
        false => dag().set_envs(vec![("OPENTOFU_VERSION".into(), version)])?,
    }

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "tofu@$OPENTOFU_VERSION", "apply", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn destroy(args: String) -> FnResult<String> {
    let version = dag().get_env("OPENTOFU_VERSION")?;
    match version.is_empty() {
        true => dag().set_envs(vec![("OPENTOFU_VERSION".into(), "latest".into())])?,
        false => dag().set_envs(vec![("OPENTOFU_VERSION".into(), version)])?,
    }

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "tofu@$OPENTOFU_VERSION", "destroy", &args])?
        .stdout()?;

    Ok(stdout)
}
