// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

pub trait PluMAPlugin {
    fn input(&mut self, filepath: String);
    fn run(&mut self);
    fn output(&mut self, filepath: String);
}
