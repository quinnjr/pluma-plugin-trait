// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

pub trait PluMAPlugin<'a> {
    fn input(&mut self, filepath: &'a str)-> Result<(), Box<dyn std::error::Error>>;
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn output(&mut self, filepath: &'a str) -> Result<(), Box<dyn std::error::Error>>;
}
