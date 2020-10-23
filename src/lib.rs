// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

pub trait PluMAPlugin {
    fn input(&mut self, filepath: String) -> Result<(), Box<dyn std::error::Error>>;
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn output(&mut self, filepath: String) -> Result<(), Box<dyn std::error::Error>>;
}
