use std::{env, path::PathBuf, process::ExitCode};

use doctor_core::{
    audit_path, audit_path_with_play, audit_path_with_project, audit_path_with_project_and_play,
    PlayPlatform,
};

fn print_usage() {
    eprintln!(
        "Usage: android-release-doctor [--project <android-module>] [--play] [--play-platform <mobile|wear|automotive|tv|xr>] <release.apk|release.aab>"
    );
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let mut project_path: Option<PathBuf> = None;
    let mut play_requested = false;
    let mut play_platform = PlayPlatform::Mobile;
    let mut play_platform_explicit = false;
    let mut artifact_path: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        if arg == "--project" {
            let Some(path) = args.next() else {
                print_usage();
                return ExitCode::from(2);
            };
            if project_path.replace(PathBuf::from(path)).is_some() {
                print_usage();
                return ExitCode::from(2);
            }
            continue;
        }

        if arg == "--play" {
            if play_requested {
                print_usage();
                return ExitCode::from(2);
            }
            play_requested = true;
            continue;
        }

        if arg == "--play-platform" {
            let Some(platform) = args.next() else {
                print_usage();
                return ExitCode::from(2);
            };
            let Some(parsed) = PlayPlatform::parse(&platform) else {
                eprintln!("Unknown Play platform: {platform}");
                print_usage();
                return ExitCode::from(2);
            };
            play_platform = parsed;
            play_platform_explicit = true;
            continue;
        }

        if arg.starts_with('-') || artifact_path.is_some() {
            print_usage();
            return ExitCode::from(2);
        }

        artifact_path = Some(PathBuf::from(arg));
    }

    let Some(artifact_path) = artifact_path else {
        print_usage();
        return ExitCode::from(2);
    };

    if play_platform_explicit && !play_requested {
        eprintln!("--play-platform requires --play");
        return ExitCode::from(2);
    }

    let result = match (project_path.as_deref(), play_requested) {
        (Some(project_path), true) => {
            audit_path_with_project_and_play(&artifact_path, project_path, play_platform)
        }
        (Some(project_path), false) => audit_path_with_project(&artifact_path, project_path),
        (None, true) => audit_path_with_play(&artifact_path, play_platform),
        (None, false) => audit_path(&artifact_path),
    };

    match result {
        Ok(report) => {
            print!("{}", report.render_text());
            let (_, _, blockers) = report.counts();
            if blockers > 0 {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(error) => {
            eprintln!("Android Release Doctor: {error}");
            ExitCode::from(2)
        }
    }
}
