use std::{env, process::Command, str};

fn main() {
    println!("cargo:rustc-check-cfg=cfg(doc_cfg)");
    println!("cargo:rustc-check-cfg=cfg(error_in_core)");

    let compiler = match rustc_version() {
        Some(compiler) => compiler,
        None => return,
    };

    // error_in_core was added in 1.81.
    if compiler.is_at_least(81) {
        println!("cargo:rustc-cfg=error_in_core");
    }
}

struct Compiler {
    minor: u32,
    channel: ReleaseChannel,
}

impl Compiler {
    fn is_at_least(&self, minor: u32) -> bool {
        // Earlier nightlies within a version might not have a feature that was
        // added later in the cycle, so we guard by minor + 1.
        self.minor >= minor
            && (self.channel == ReleaseChannel::Stable || self.channel == ReleaseChannel::Beta)
            || self.minor >= minor + 1
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReleaseChannel {
    Stable,
    Beta,
    Nightly,
}

fn rustc_version() -> Option<Compiler> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    let minor = pieces.next()?.parse().ok()?;
    let channel = if version.contains("nightly") {
        ReleaseChannel::Nightly
    } else if version.contains("beta") {
        ReleaseChannel::Beta
    } else {
        ReleaseChannel::Stable
    };
    Some(Compiler { minor, channel })
}
