/*
	Copyright 2019 Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.
*/

use log::*;
use std::path::Path;
use constants::*;
use std::process::Command;

pub fn check_files() -> u8 {
	debug!("*** Check files");

	let mut missing_files = 0;
	missing_files += file_missing(ENCLAVE_FILE);
	missing_files += file_missing(RSA_PUB_KEY);
	missing_files += file_missing(ECC_PUB_KEY);

	// remote attestation files
	missing_files += file_missing(RA_SPID);
	missing_files += file_missing(RA_API_KEY);

	missing_files
}

fn file_missing(path: &str) -> u8 {
	if Path::new(path).exists() {
		debug!("File '{}' found", path);
		0
	} else {
		error!("File '{}' not found", path);
		1
	}
}

pub fn get_wasm_hash() -> Vec<String> {
	let sha_cmd = Command::new("sha256sum")
		.arg("./bin/worker_enclave.compact.wasm")
		.output()
		.expect("Failed to get sha256sum of worker_enclave.wasm");

	std::str::from_utf8(&sha_cmd.stdout).unwrap()
		.split("  ")
		.map(|s| s.to_string())
		.collect()
}
