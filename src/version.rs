use once_cell::sync::Lazy;
use regex::Regex;

pub static VERSION_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"v?(?<major>\d+)\.(?<minor>\d+)(?:\.(?<patch>\d+))?(?:(?<pre>a|b|c|rc)(?<preid>\d+))?",
    )
    .unwrap()
});

pub fn from_python_version(version: String) -> Option<String> {
    let Some(caps) = VERSION_PATTERN.captures(&version) else {
        return None;
    };

    let mut version = format!(
        "{}.{}.{}",
        &caps["major"],
        &caps["minor"],
        caps.name("patch").map(|c| c.as_str()).unwrap_or("0"),
    );

    if let Some(pre) = caps.name("pre") {
        let preid = format!("-{}.{}", pre.as_str(), &caps["preid"]);
        version.push_str(&preid);
    }

    Some(version)
}
