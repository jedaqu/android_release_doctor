use std::{env, path::PathBuf, process::ExitCode};

use doctor_core::{audit_path, audit_path_with_project};

fn print_usage() {
    eprintln!(
        "Usage: android-release-doctor [--project <android-module>] <release.apk|release.aab>"
    );
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let mut project_path: Option<PathBuf> = None;
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

    let result = match project_path.as_deref() {
        Some(project_path) => audit_path_with_project(&artifact_path, project_path),
        None => audit_path(&artifact_path),
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
