use std::{env, fs, path::PathBuf, process::ExitCode};

use doctor_core::{
    audit_path, audit_path_with_play, audit_path_with_project, audit_path_with_project_and_play,
    PlayPlatform, ReportV1, ReportV1Context, ENGINE_VERSION,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug)]
struct CliOptions {
    project_path: Option<PathBuf>,
    play_requested: bool,
    play_platform: PlayPlatform,
    format: OutputFormat,
    output_path: Option<PathBuf>,
    artifact_path: PathBuf,
}

fn usage() -> &'static str {
    "Usage: android-release-doctor [--project <android-module>] [--play] [--play-platform <mobile|wear|automotive|tv|xr>] [--format <text|json>] [--output <path>] <release.apk|release.aab>\n\nOptions:\n  --project <android-module>         Audit the Android application Gradle configuration.\n  --play                             Apply the Google Play readiness profile.\n  --play-platform <platform>         Select mobile, wear, automotive, tv, or xr.\n  --format <text|json>               Select human-readable text or Report v1 JSON.\n  --output <path>                    Write the report to a file instead of stdout.\n  --help                             Show this help and exit.\n  --version                          Show the engine version and exit.\n"
}

fn main() -> ExitCode {
    match parse_args(env::args().skip(1)) {
        Ok(Command::Help) => {
            print!("{}", usage());
            ExitCode::SUCCESS
        }
        Ok(Command::Version) => {
            println!("android-release-doctor {ENGINE_VERSION}");
            ExitCode::SUCCESS
        }
        Ok(Command::Run(options)) => run_audit(options),
        Err(error) => {
            eprintln!("Android Release Doctor: {error}");
            eprint!("{}", usage());
            ExitCode::from(2)
        }
    }
}

enum Command {
    Help,
    Version,
    Run(CliOptions),
}

fn parse_args<I>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let mut project_path: Option<PathBuf> = None;
    let mut play_requested = false;
    let mut play_platform = PlayPlatform::Mobile;
    let mut play_platform_explicit = false;
    let mut format = OutputFormat::Text;
    let mut format_explicit = false;
    let mut output_path: Option<PathBuf> = None;
    let mut help_requested = false;
    let mut version_requested = false;
    let mut artifact_path: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        if arg == "--help" {
            if help_requested {
                return Err("duplicate --help".to_string());
            }
            help_requested = true;
            continue;
        }

        if arg == "--version" {
            if version_requested {
                return Err("duplicate --version".to_string());
            }
            version_requested = true;
            continue;
        }

        if arg == "--project" {
            let Some(path) = args.next() else {
                return Err("--project requires a value".to_string());
            };
            if project_path.replace(PathBuf::from(path)).is_some() {
                return Err("duplicate --project".to_string());
            }
            continue;
        }

        if arg == "--play" {
            if play_requested {
                return Err("duplicate --play".to_string());
            }
            play_requested = true;
            continue;
        }

        if arg == "--play-platform" {
            let Some(platform) = args.next() else {
                return Err("--play-platform requires a value".to_string());
            };
            let Some(parsed) = PlayPlatform::parse(&platform) else {
                return Err(format!("unknown Play platform: {platform}"));
            };
            if play_platform_explicit {
                return Err("duplicate --play-platform".to_string());
            }
            play_platform = parsed;
            play_platform_explicit = true;
            continue;
        }

        if arg == "--format" {
            let Some(value) = args.next() else {
                return Err("--format requires a value".to_string());
            };
            if format_explicit {
                return Err("duplicate --format".to_string());
            }
            format = match value.as_str() {
                "text" => OutputFormat::Text,
                "json" => OutputFormat::Json,
                _ => return Err(format!("unknown output format: {value}")),
            };
            format_explicit = true;
            continue;
        }

        if arg == "--output" {
            let Some(path) = args.next() else {
                return Err("--output requires a value".to_string());
            };
            if output_path.replace(PathBuf::from(path)).is_some() {
                return Err("duplicate --output".to_string());
            }
            continue;
        }

        if arg.starts_with('-') {
            return Err(format!("unknown option: {arg}"));
        }

        if artifact_path.replace(PathBuf::from(arg)).is_some() {
            return Err("only one artifact path may be supplied".to_string());
        }
    }

    if help_requested || version_requested {
        if help_requested && version_requested {
            return Err("--help and --version cannot be combined".to_string());
        }
        if project_path.is_some()
            || play_requested
            || play_platform_explicit
            || format_explicit
            || output_path.is_some()
            || artifact_path.is_some()
        {
            return Err("--help/--version are standalone commands".to_string());
        }
        return Ok(if help_requested {
            Command::Help
        } else {
            Command::Version
        });
    }

    let Some(artifact_path) = artifact_path else {
        return Err("an APK or AAB artifact path is required".to_string());
    };

    if play_platform_explicit && !play_requested {
        return Err("--play-platform requires --play".to_string());
    }

    Ok(Command::Run(CliOptions {
        project_path,
        play_requested,
        play_platform,
        format,
        output_path,
        artifact_path,
    }))
}

fn run_audit(options: CliOptions) -> ExitCode {
    let result = match (options.project_path.as_deref(), options.play_requested) {
        (Some(project_path), true) => audit_path_with_project_and_play(
            &options.artifact_path,
            project_path,
            options.play_platform,
        ),
        (Some(project_path), false) => {
            audit_path_with_project(&options.artifact_path, project_path)
        }
        (None, true) => audit_path_with_play(&options.artifact_path, options.play_platform),
        (None, false) => audit_path(&options.artifact_path),
    };

    match result {
        Ok(report) => {
            let output = match options.format {
                OutputFormat::Text => Ok(report.render_text()),
                OutputFormat::Json => {
                    let context = if options.play_requested {
                        ReportV1Context::with_play(options.play_platform)
                    } else {
                        ReportV1Context::without_play()
                    };
                    ReportV1::from_audit_report(&report, context)
                        .to_json()
                        .map(|json| format!("{json}\n"))
                }
            };

            let output = match output {
                Ok(output) => output,
                Err(error) => {
                    eprintln!("Android Release Doctor: failed to serialize report: {error}");
                    return ExitCode::from(2);
                }
            };

            if let Some(path) = options.output_path {
                if let Err(error) = fs::write(&path, output.as_bytes()) {
                    eprintln!(
                        "Android Release Doctor: failed to write report to {}: {error}",
                        path.display()
                    );
                    return ExitCode::from(2);
                }
            } else {
                print!("{output}");
            }

            if report.counts().2 > 0 {
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
