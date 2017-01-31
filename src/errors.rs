// Copyright 2016  Jonas mg
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::error;
use std::error::Error as ErrorT;
use std::fmt;
use std::io;
use std::time;

#[derive(Debug)]
pub enum IdError {
    Io(io::Error),
    Time(time::SystemTimeError),
}

impl From<io::Error> for IdError {
    fn from(err: io::Error) -> IdError {
        IdError::Io(err)
    }
}
impl From<time::SystemTimeError> for IdError {
    fn from(err: time::SystemTimeError) -> IdError {
        IdError::Time(err)
    }
}

impl error::Error for IdError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            IdError::Io(ref err) => Some(err),
            IdError::Time(ref err) => Some(err),
        }
    }
    fn description(&self) -> &str {
        match *self {
            IdError::Io(ref err) => err.description(),
            IdError::Time(ref err) => err.description(),
        }
    }
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IdError::Io(ref err) => err.fmt(f),
            IdError::Time(ref err) => err.fmt(f),
        }
    }
}
