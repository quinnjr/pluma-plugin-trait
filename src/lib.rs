// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

pub trait PluMAPlugin {
    fn input<'a>(&mut self, filepath: String);
    fn run(&mut self);
    fn output<'a>(&mut self, filepath: String);
}
