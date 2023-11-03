use crate::version::from_python_version;
use extism_pdk::*;
use proto_pdk::*;
use regex::Regex;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
    fn host_log(input: Json<HostLogInput>);
}

static NAME: &str = "Python";

#[derive(Deserialize)]
struct PythonManifest {
    python_exe: String,
    python_major_minor_version: String,
}

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec![".python-version".into()],
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/python/cpython")?;
    let regex = Regex::new(
        r"v?(?<major>[0-9]+)\.(?<minor>[0-9]+)(?:\.(?<patch>[0-9]+))?(?:(?<pre>a|b|c|rc)(?<preid>[0-9]+))?",
    )
    .unwrap();

    let tags = tags
        .into_iter()
        .filter(|t| t != "legacy-trunk")
        .filter_map(|t| from_python_version(t, &regex))
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

// #[plugin_fn]
// pub fn native_install(
//     Json(input): Json<NativeInstallInput>,
// ) -> FnResult<Json<NativeInstallOutput>> {
//     let mut output = NativeInstallOutput::default();
//     let env = get_proto_environment()?;

//     // https://github.com/pyenv/pyenv/tree/master/plugins/python-build
//     if command_exists(&env, "python-build") {
//         host_log!("Building with `python-build` instead of downloading a pre-built");

//         let result = exec_command!(
//             inherit,
//             "python-build",
//             [
//                 input.context.version.as_str(),
//                 input.install_dir.real_path().to_str().unwrap(),
//             ]
//         );

//         output.installed = result.exit_code == 0;
//     } else {
//         output.skip_install = true;
//     }

//     Ok(Json(output))
// }

#[derive(Deserialize)]
struct ReleaseEntry {
    download: String,
    checksum: Option<String>,
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_proto_environment()?;
    let version = &input.context.version;

    if version.is_canary() {
        return err!(PluginError::UnsupportedCanary { tool: NAME.into() }.into());
    }

    let releases: BTreeMap<Version, BTreeMap<String, ReleaseEntry>> =
        fetch_url("https://raw.githubusercontent.com/moonrepo/python-plugin/master/releases.json")?;

    let release_triples = match version {
        VersionSpec::Version(v) => releases.get(v),
        _ => None,
    };

    let Some(release_triples) = release_triples else {
        return err!("No pre-built available for version {}!", version);
    };

    let triple = get_target_triple(&env, NAME)?;

    let Some(release) = release_triples.get(&triple) else {
        return err!("No pre-built available for architecture {}!", triple);
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some("python".into()),
        checksum_url: release.checksum.clone(),
        download_url: release.download.clone(),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(input): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_proto_environment()?;
    let mut exe_path = format_bin_name("install/bin/python3", env.os);
    let mut globals_lookup_dirs = vec!["$HOME/.local/bin".to_owned()];

    // Manifest is only available for pre-builts
    let manifest_path = input.context.tool_dir.join("PYTHON.json");

    if manifest_path.exists() {
        let manifest: PythonManifest = json::from_slice(&fs::read(manifest_path)?)?;
        exe_path = manifest.python_exe;

        if env.os == HostOS::Windows {
            let formatted_version = manifest.python_major_minor_version.replace('.', "");

            globals_lookup_dirs.push(format!(
                "$APPDATA/Roaming/Python{}/Scripts",
                formatted_version
            ));
            globals_lookup_dirs.push(format!("$APPDATA/Python{}/Scripts", formatted_version));
        }
    }

    Ok(Json(LocateExecutablesOutput {
        globals_lookup_dirs,
        primary: Some(ExecutableConfig::new(exe_path)),
        secondary: HashMap::from_iter([
            // pip
            (
                "pip".into(),
                ExecutableConfig {
                    no_bin: true,
                    shim_before_args: Some("-m pip".into()),
                    ..ExecutableConfig::default()
                },
            ),
        ]),
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn install_global(
    Json(input): Json<InstallGlobalInput>,
) -> FnResult<Json<InstallGlobalOutput>> {
    let result = exec_command!(inherit, "pip", ["install", "--user", &input.dependency]);

    Ok(Json(InstallGlobalOutput::from_exec_command(result)))
}

#[plugin_fn]
pub fn uninstall_global(
    Json(input): Json<UninstallGlobalInput>,
) -> FnResult<Json<UninstallGlobalOutput>> {
    let result = exec_command!(inherit, "pip", ["uninstall", "--yes", &input.dependency]);

    Ok(Json(UninstallGlobalOutput::from_exec_command(result)))
}

// DEPRECATED
// Removed in v0.23!

#[plugin_fn]
pub fn locate_bins(Json(input): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    let env = get_proto_environment()?;
    let mut bin_path = format_bin_name("install/bin/python3", env.os);
    let mut globals_lookup_dirs = vec!["$HOME/.local/bin".to_owned()];

    // Manifest is only available for pre-builts
    let manifest_path = input.context.tool_dir.join("PYTHON.json");

    if manifest_path.exists() {
        let manifest: PythonManifest = json::from_slice(&fs::read(manifest_path)?)?;

        bin_path = manifest.python_exe;

        if env.os == HostOS::Windows {
            let formatted_version = manifest.python_major_minor_version.replace('.', "");

            globals_lookup_dirs.push(format!(
                "$APPDATA/Roaming/Python{}/Scripts",
                formatted_version
            ));

            globals_lookup_dirs.push(format!("$APPDATA/Python{}/Scripts", formatted_version));
        }
    }

    Ok(Json(LocateBinsOutput {
        bin_path: Some(bin_path.into()),
        fallback_last_globals_dir: true,
        globals_lookup_dirs,
        ..LocateBinsOutput::default()
    }))
}

#[plugin_fn]
pub fn create_shims(Json(_): Json<CreateShimsInput>) -> FnResult<Json<CreateShimsOutput>> {
    let mut global_shims = HashMap::new();

    global_shims.insert("pip".into(), ShimConfig::global_with_sub_command("-m pip"));

    Ok(Json(CreateShimsOutput {
        global_shims,
        ..CreateShimsOutput::default()
    }))
}
