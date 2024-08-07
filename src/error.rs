// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;

#[derive(Debug)]
pub enum Error {
    RecoverError,
    SignError,
    KeyPairError,
    SignatureError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::RecoverError => "Recover Error",
            Error::SignError => "Sign Error",
            Error::KeyPairError => "KeyPair Error",
            Error::SignatureError => "Signature Error",
        };
        f.write_fmt(format_args!("Crypto error: {}", message))
    }
}
