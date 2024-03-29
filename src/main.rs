use crate::cli::{run, Cli};
use std::{env, fs, io, path::PathBuf, process::exit};

mod cli;
mod digest;
mod static_vals;

const fn builtin_dockerfile_content() -> &'static str {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "Dockerfile"))
}

fn get_deb_suite(tmm_arch: &str) -> &str {
    match tmm_arch {
        "armv3" => "lenny",
        "sparc" | "s390" => "wheezy",
        "armv4t" => "stretch",
        "mipsbe" => "buster",
        "mipsle" => "bookworm",
        _ => "sid",
    }
}

fn main() -> io::Result<()> {
    // Depends: docker.io | docker
    //
    env::set_var("DOCKER_BUILDKIT", "1");
    let args = argh::from_env::<Cli>();

    if args.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        exit(0)
    }

    let arch_list = [
        "x64", "rv64gc", "arm64", "armv7a", "armv5te", "mips64le", "ppc64le",
        "s390x", "x86", //
        "alpha", "hppa", "loong64", "m68k", "ppc", "ppc64", "sh4", "sparc64",
        "ia64", "x32", //
        "mipsle", "mipsbe", "armv4t", "sparc", "s390", "armv3",
    ];

    let reg = cli::get_registry(&args);

    if let Some(ref arch) = args.arch {
        if !arch_list.contains(&arch.as_str()) {
            panic!("Only support: {arch_list:?}")
        }
        if args.digest {
            digest::gen_digest(arch);
            exit(0);
        }
        build_docker(&reg, arch, args.push)?;
        exit(0);
    }

    if let Some(manifest) = args.subcmd {
        manifest.create_and_push(reg.get_zsh(), args.push);
    };

    Ok(())
}

fn build_docker(
    reg: &cli::Registry<'_>,
    arch: &String,
    push: bool,
) -> io::Result<()> {
    let b_arg = "--build-arg";

    let deb_arch = archmap::debian_arch::map()
        .get(arch)
        .copied()
        .expect("Failed to get debian arch");

    let platform = archmap::linux_oci_platform::map()
        .get(arch)
        .copied()
        .expect("Invalid OCI platform");

    let tmp_dir = create_docker_file()?;

    let zsh_repo = reg.get_zsh();

    let target_repo = format!("{zsh_repo}:{arch}");
    let args = [
        "build",
        "--progress=plain",
        //
        b_arg,
        &format!("REGION={}", static_vals::os_region()),
        //
        b_arg,
        &format!("DEB_ARCH={deb_arch}",),
        //
        b_arg,
        &format!("DEB_SUITE={}", get_deb_suite(arch)),
        //
        b_arg,
        &format!("ZSH_REPO={zsh_repo}"),
        //
        b_arg,
        &format!("SID_REPO={}", reg.get_sid()),
        //
        "--platform",
        platform,
        //
        "--tag",
        &target_repo,
        //
        &tmp_dir.to_string_lossy(),
    ];

    run(&args);

    if push {
        run(&["push", &target_repo])
    }

    fs::remove_dir_all(&tmp_dir)
}

fn create_docker_file() -> io::Result<PathBuf> {
    let tmp_dir = env::temp_dir()
        .join(env!("CARGO_PKG_NAME"))
        .join(
            static_vals::now_time()
                .to_string()
                .replace([':', ' '], "_"),
        );

    eprintln!("[INFO] creating the tmp dir: {tmp_dir:?}");
    fs::create_dir_all(&tmp_dir)?;

    let dk_name = "Dockerfile";
    let docker_file = tmp_dir.join(dk_name);

    let current_dk = env::current_dir()?.join(dk_name);

    if current_dk.exists() {
        eprintln!("[INFO] using this file: {current_dk:?}");
        fs::copy(current_dk, docker_file)?;
    } else {
        fs::write(docker_file, builtin_dockerfile_content())?;
    }

    Ok(tmp_dir)
}
