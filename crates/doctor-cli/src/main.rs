use std::{env, process::ExitCode};

use doctor_core::audit_path;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("Usage: android-release-doctor <release.apk|release.aab>");
        return ExitCode::from(2);
    };

    if args.next().is_some() {
        eprintln!("Usage: android-release-doctor <release.apk|release.aab>");
        return ExitCode::from(2);
    }

    match audit_path(&path) {
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
