use argh::FromArgs;
use std::{
    borrow::Cow,
    ffi::OsStr,
    process::{self, Command},
};
use tinyvec::TinyVec;

use crate::static_vals::today;

#[derive(FromArgs, Debug)]
/// Build zsh static docker containers
pub(crate) struct Cli {
    /// [ "x64", "rv64gc", "arm64", "armv7a", "armv5te", "mips64le", "ppc64le", "s390x",
    /// "x86", "alpha", "hppa", "loong64", "m68k", "ppc", "ppc64", "sh4", "sparc64",
    /// "ia64", "x32", "mipsle", "mipsbe", "armv4t", "sparc", "s390", "armv3" ]
    #[argh(option)]
    pub(crate) arch: Option<String>,

    #[argh(switch)]
    /// whether to push to the registry
    pub(crate) push: bool,

    #[argh(subcommand)]
    pub(crate) subcmd: Option<Manifest>,

    #[argh(switch)]
    /// display version
    pub(crate) version: bool,

    #[argh(switch)]
    /// github registry: ghcr.io/2moe/zsh-static
    ghcr_reg: bool,

    #[argh(switch)]
    /// tmm registry: reg.tmoe.me:2096/pkgs/zsh-static
    tmm_reg: bool,

    #[argh(option)]
    /// specify the docker registry.
    /// e.g., ghcr.io/[owner]/[repo]
    custom_reg: Option<String>,
}
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "manifest")]
/// create manifests
pub(crate) struct Manifest {
    #[argh(option)]
    /// acceptable arguments are architectures split by comma(`,`) or space(` `)
    /// (e.g., `--archs x64,rv64gc,arm64` or  `--archs "x64 rv64gc arm64"`)
    archs: String,

    #[argh(switch)]
    /// create latest manifest (i.e., zsh:x64, zsh:arm64 -> zsh:latest).
    latest: bool,

    #[argh(switch)]
    /// create latest manifest (i.e., zsh:x64, zsh:rv64gc -> zsh:2023-03-01).
    date: bool,
}

type Archs<'a> = TinyVec<[&'a str; 25]>;
type Tags<'a> = TinyVec<[&'a str; 2]>;

impl Manifest {
    pub(crate) fn parse(&self) -> (Tags, Archs) {
        let archs = &self.archs;
        let v = archs
            .split([',', ' '])
            .filter(|x| !x.is_empty())
            .collect::<Archs>();
        let mut tags = Tags::new();

        if self.latest {
            tags.push("latest");
        }
        if self.date {
            tags.push(today());
        }

        if tags.is_empty() {
            panic!("Requires `--latest` or `--date`");
        }

        (tags, v)
    }

    pub(crate) fn create_and_push(&self, zsh_repo: &str, push: bool) {
        let (tags, archs) = self.parse();
        type Repos = TinyVec<[String; 25]>;
        type DockerArgs<'a> = TinyVec<[&'a str; 29]>;

        let repos = archs
            .iter()
            .map(|arch| format!("{zsh_repo}:{arch}"))
            .collect::<Repos>();

        for tag in tags {
            let repo = format!("{zsh_repo}:{tag}");
            let mut c_args = DockerArgs::new();
            c_args.extend(["manifest", "create", "--amend", &repo]);
            c_args.extend(repos.iter().map(|x| x.as_str()));
            run(&c_args);

            if push {
                run(&["manifest", "push", "--purge", &repo])
            }
        }
    }
}
// ------------------------------
pub(crate) struct Registry<'a> {
    zsh: Cow<'a, str>,
    sid: &'a str,
}

impl<'a> Registry<'a> {
    pub(crate) fn get_zsh(&self) -> &str {
        &self.zsh
    }

    pub(crate) fn get_sid(&self) -> &str {
        self.sid
    }
}

impl<'a> Default for Registry<'a> {
    fn default() -> Self {
        const SID_REPO: &str = "ghcr.io/2cd/debian-sid";
        Self {
            zsh: Cow::from("ghcr.io/2moe/zsh-static"),
            sid: SID_REPO,
        }
    }
}
pub(crate) fn get_registry<'a>(args: &Cli) -> Registry<'a> {
    let ghcr_repo = Default::default();
    if args.ghcr_reg {
        return ghcr_repo;
    }

    if args.tmm_reg {
        return Registry {
            zsh: Cow::from("reg.tmoe.me:2096/pkgs/zsh-static"),
            sid: "reg.tmoe.me:2096/debian/sid",
        };
    }

    if let Some(ref s) = args.custom_reg {
        return Registry {
            zsh: Cow::from(s.to_owned()),
            ..ghcr_repo
        };
    }

    ghcr_repo
}

pub(crate) fn run<S>(args: &[S])
where
    S: AsRef<OsStr>,
{
    eprint!("run: docker\nargs:\n    ");
    for a in args.iter().map(|x| x.as_ref()) {
        eprint!("{a:?} ");
    }
    eprintln!();

    let status = || {
        Command::new("docker")
            .args(args)
            .status()
    };

    let retry = || {
        eprintln!("[WARN] Retrying ...");

        if !status()
            .expect("Failed to run docker")
            .success()
        {
            process::exit(1)
        }
    };

    match status() {
        Ok(s) if !s.success() => retry(),
        Err(e) => {
            eprintln!("[ERROR] {e}");
            retry()
        }
        _ => {}
    }
}
