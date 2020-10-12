// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

pub trait PluMAPlugin {
    fn input<'a>(&mut self, filepath: &'a str)-> Result<(), Box<dyn std::error::Error>>;
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn output<'a>(&mut self, filepath: &'a str) -> Result<(), Box<dyn std::error::Error>>;
}
