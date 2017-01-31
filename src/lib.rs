// Copyright 2016  Jonas mg
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Generates numeric identifiers.

use std::io;
use std::time;

extern crate byteorder;
extern crate rand;

use byteorder::{ReadBytesExt, WriteBytesExt};

mod errors;
pub use errors::*;

/// Generates a Combid (Combined Identifier), a combination of a timestamp and
/// some random bits. The timestamp ensures they are ordered chronologically,
/// and the random bits ensure that each ID is unique, even if thousands of
/// people are creating IDs at the same time.
///
/// timestamp     - 5 bytes (40 bits) - 4 bytes from seconds and the other ones from nanoseconds  
/// random number - 3 bytes (24 bits) - gives us up to 16_777_216 possible values
///
/// # Examples
///
/// ```
/// extern crate combid;
/// extern crate rand;
///
/// let mut rng = rand::thread_rng();
///
/// let id = match combid::gen(&mut rng) {
///     Ok(v) => v,
///     Err(e) => panic!(e), // handle error
/// };
/// println!("combid: {}", id);
/// ```
///
pub fn gen<R: rand::Rng>(rng: &mut R) -> Result<i64, IdError> {
    let now = match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
        Ok(v) => v,
        Err(e) => return Err(IdError::Time(e)),
    };

    let mut v_time_sec: Vec<u8> = Vec::with_capacity(8);
    match v_time_sec.write_u64::<byteorder::BigEndian>(now.as_secs()) {
        Ok(_) => (),
        Err(e) => return Err(IdError::Io(e)),
    }

    let mut v_time_nsec: Vec<u8> = Vec::with_capacity(4);
    match v_time_nsec.write_u32::<byteorder::BigEndian>(now.subsec_nanos()) {
        Ok(_) => (),
        Err(e) => return Err(IdError::Io(e)),
    }

    let mut v_rand: Vec<u8> = Vec::with_capacity(4);
    match v_rand.write_u32::<byteorder::BigEndian>(rng.gen()) {
        Ok(_) => (),
        Err(e) => return Err(IdError::Io(e)),
    }

    let array: [u8; 8] = [v_time_sec[4],
                          v_time_sec[5],
                          v_time_sec[6],
                          v_time_sec[7],
                          v_time_nsec[0],
                          v_rand[0],
                          v_rand[1],
                          v_rand[2]];

    let mut rd = io::Cursor::new(array);
    match rd.read_i64::<byteorder::BigEndian>() {
        Ok(v) => Ok(v),
        Err(e) => return Err(IdError::Io(e)),
    }
}

/// Generates an identifier based in the current time.
pub fn gen_timeid() -> Result<i64, IdError> {
    let now = match time::UNIX_EPOCH.elapsed() {
        Ok(v) => v,
        Err(e) => return Err(IdError::Time(e)),
    };

    let mut v_time_sec: Vec<u8> = Vec::with_capacity(8);
    match v_time_sec.write_u64::<byteorder::BigEndian>(now.as_secs()) {
        Ok(_) => (),
        Err(e) => return Err(IdError::Io(e)),
    }

    let mut v_time_nsec: Vec<u8> = Vec::with_capacity(4);
    match v_time_nsec.write_u32::<byteorder::BigEndian>(now.subsec_nanos()) {
        Ok(_) => (),
        Err(e) => return Err(IdError::Io(e)),
    }

    let array: [u8; 8] = [v_time_sec[4],
                          v_time_sec[5],
                          v_time_sec[6],
                          v_time_sec[7],
                          v_time_nsec[0],
                          v_time_nsec[1],
                          v_time_nsec[2],
                          v_time_nsec[3]];

    let mut rdr = io::Cursor::new(array);
    match rdr.read_i64::<byteorder::BigEndian>() {
        Ok(v) => Ok(v),
        Err(e) => return Err(IdError::Io(e)),
    }
}

// == Iterators
//

/// Iterator which will generate combids using a thread-local RNG.
pub struct Generator<T> {
    rng: T,
}

impl<T: rand::Rng> Generator<T> {
    pub fn new(rng: T) -> Generator<T> {
        Generator { rng: rng }
    }
}

impl<T: rand::Rng> Iterator for Generator<T> {
    type Item = Result<i64, IdError>;

    fn next(&mut self) -> Option<Result<i64, IdError>> {
        match gen(&mut self.rng) {
            Ok(v) => Some(Ok(v)),
            Err(e) => Some(Err(e)),
        }
    }
}

/// Iterator which will generate identifiers based in the current time.
///
/// # Examples
///
/// ```
/// let mut gen_time = combid::TimeGenerator {};
///
/// let timeid = match gen_time.next() {
///     Some(v) => {
///         match v {
///             Ok(v) => v,
///             Err(e) => panic!(e), // handle error
///         }
///     },
///     None => unreachable!(),
/// };
/// println!("timeid: {}", timeid);
/// ```
///
pub struct TimeGenerator {}

impl Iterator for TimeGenerator {
    type Item = Result<i64, IdError>;

    fn next(&mut self) -> Option<Result<i64, IdError>> {
        match gen_timeid() {
            Ok(v) => Some(Ok(v)),
            Err(e) => Some(Err(e)),
        }
    }
}

// == Tests
//

#[cfg(test)]
mod tests {
    use std::time;

    use byteorder::{BigEndian, WriteBytesExt};
    use rand;

    use super::*;

    #[test]
    // Check the bytes filled at marshaling a time.
    fn check_bytes_time() {
        let now = time::UNIX_EPOCH.elapsed().unwrap();
        let mut v_time_sec: Vec<u8> = Vec::with_capacity(8);
        v_time_sec.write_u64::<BigEndian>(now.as_secs()).unwrap();

        // Should be like [0, 0, 0, 0, 87, 89, 71, 199]
        assert_eq!(v_time_sec[3], 0);
    }

    #[test]
    fn generators() {
        let mut rng = rand::thread_rng();

        let id = gen(&mut rng).unwrap();
        assert!(id > 0);

        let id = gen_timeid().unwrap();
        assert!(id > 0);
    }

    #[test]
    fn iterators() {
        let mut rng = rand::thread_rng();
        let mut gen = Generator::new(&mut rng);
        let mut result: Result<i64, IdError>;

        print!("\n== Generating Combids\n");
        for _ in 0..5 {
            result = gen.next().unwrap();
            println!("{:?}", result);
        }

        print!("\n== Generating time ids\n");
        let mut gen_time = TimeGenerator {};
        for _ in 0..5 {
            result = gen_time.next().unwrap();
            println!("{:?}", result);
        }
        println!("");
    }
}
