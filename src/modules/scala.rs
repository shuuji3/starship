use crate::configs::scala::ScalaConfig;
use crate::formatter::StringFormatter;

use super::{Context, Module, RootModuleConfig};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("scala");
    let config: ScalaConfig = ScalaConfig::try_load(module.config);

    let is_scala_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_scala_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let scala_version = get_scala_version(context)?;
                    Some(Ok(scala_version))
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `scala`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_scala_version(context: &Context) -> Option<String> {
    let output = context.exec_cmd("scalac", &["-version"])?;
    let scala_version = if output.stdout.is_empty() {
        output.stderr
    } else {
        output.stdout
    };

    parse_scala_version(&scala_version)
}

fn parse_scala_version(scala_version: &str) -> Option<String> {
    let version = scala_version
        // split into ["Scala", "compiler", "version", "2.13.5", "--", ...]
        .split_whitespace()
        // take "2.13.5"
        .nth(3)?;

    Some(format!("v{}", &version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn test_parse_scala_version() {
        let scala_2_13 =
            "Scala compiler version 2.13.5 -- Copyright 2002-2020, LAMP/EPFL and Lightbend, Inc.";
        assert_eq!(parse_scala_version(scala_2_13), Some("v2.13.5".to_string()));
    }

    #[test]
    fn test_parse_dotty_version() {
        let dotty_version = "Scala compiler version 3.0.0-RC1 -- Copyright 2002-2021, LAMP/EPFL";
        assert_eq!(
            parse_scala_version(dotty_version),
            Some("v3.0.0-RC1".to_string())
        );
    }

    #[test]
    fn folder_without_scala_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("scala").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_scala_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Test.scala"))?.sync_all()?;
        let actual = ModuleRenderer::new("scala").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🆂 v2.13.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_scala_file_no_scala_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Test.scala"))?.sync_all()?;
        let actual = ModuleRenderer::new("scala")
            .cmd("scalac -version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🆂 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_sbt_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("build.sbt"))?.sync_all()?;
        let actual = ModuleRenderer::new("scala").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🆂 v2.13.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_scala_env_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".scalaenv"))?.sync_all()?;
        let actual = ModuleRenderer::new("scala").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🆂 v2.13.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_sbt_env_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".sbtenv"))?.sync_all()?;
        let actual = ModuleRenderer::new("scala").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🆂 v2.13.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_metals_dir() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".metals"))?;
        let actual = ModuleRenderer::new("scala").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🆂 v2.13.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
