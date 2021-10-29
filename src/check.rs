//! Forgetting and pruning snapshots

use std::time::Instant;

use anyhow::{anyhow, Context, Result};
use slog::{error, info};

use crate::restic::Restic;

impl<'a> Restic<'a> {
    /// Checks the repository for errors.
    pub fn check(&self, read_data: bool) -> Result<()> {
        let mut cmd = self.new_command();
        cmd.arg("check");

        if read_data {
            cmd.arg("--read-data");
        }

        info!(self.logger(), "Checking repository"; "read-data" => read_data, "command" => ?cmd);
        let start = Instant::now();
        let status = cmd
            .status()
            .with_context(|| format!("Could not run {:?}", cmd))?;
        let duration = Instant::now() - start;

        if status.success() {
            info!(self.logger(), "Checked repository in {:?}", duration; "command" => ?cmd);
            Ok(())
        } else {
            error!(self.logger(), "Checking repository failed"; "status" => %status, "command" => ?cmd);
            Err(anyhow!("Restic check failed with {}", status))
        }
    }
}
