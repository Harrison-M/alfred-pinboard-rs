use super::*;
use std::io::Write;
use chrono::prelude::*;

pub fn run(mut config: Config, mut pinboard: Pinboard) {
    info!("Starting in run");
    match pinboard.is_cache_outdated(config.update_time) {
        Err(err) => {
            io::stdout()
                .write(format!("Error: {}", err).as_ref())
                .expect("Couldn't write to stdout");
            process::exit(1);
        }
        Ok(needs_update) => {
            if needs_update {
                pinboard.update_cache().unwrap_or_else(|err| {
                    io::stdout()
                        .write(format!("Error: {}", err).as_ref())
                        .expect("Couldn't write to stdout");
                    process::exit(1);
                });
                config.update_time = Utc::now();
                if let Err(_) = config.save() {
                    io::stdout()
                        .write(
                            format!("Error: Couldn't save update time to workflow's config file!")
                                .as_ref(),
                        )
                        .expect("Couldn't write to stdout");
                }
                io::stdout()
                    .write(format!("Updated cache files!").as_ref())
                    .expect("Couldn't write to stdout");
            } else {
                io::stdout()
                    .write(format!("Cache is already up-to-date!").as_ref())
                    .expect("Couldn't write to stdout");
            }
        }
    }
}