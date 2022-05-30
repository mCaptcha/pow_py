/*
 * mCaptcha is a PoW based DoS protection software.
 * This is the frontend web component of the mCaptcha system
 * Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
 *
 * Use of this source code is governed by Apache 2.0 or MIT license.
 * You shoud have received a copy of MIT and Apache 2.0 along with
 * this program. If not, see <https://spdx.org/licenses/MIT.html> for
 * MIT or <http://www.apache.org/licenses/LICENSE-2.0> for Apache.
 */
use pyo3::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use pow_sha256::{Config, ConfigBuilder, PoW};

#[pyclass(name = "Work", module = "mcaptcha_pow_py")]
#[derive(Debug, PartialEq, Clone)]
pub struct Work {
    #[pyo3(get)]
    pub result: String,
    #[pyo3(get)]
    pub nonce: u64,
}

impl From<PoW<String>> for Work {
    fn from(p: PoW<String>) -> Self {
        Work {
            result: p.result,
            nonce: p.nonce,
        }
    }
}

#[pyclass(name = "PoWConfig", module = "mcaptcha_pow_py")]
#[derive(Debug, PartialEq, Clone)]
pub struct PoWConfig {
    config: Config,
}

#[pymethods]
impl PoWConfig {
    #[new]
    pub fn new(salt: String) -> Self {
        let config = ConfigBuilder::default().salt(salt).build().unwrap();
        Self { config }
    }
    pub fn work(&self, phrase: String, difficulty_factor: u32) -> Work {
        let work = self.config.prove_work(&phrase, difficulty_factor).unwrap();
        work.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pow_sha256::PoWBuilder;

    const SALT: &str = "yrandomsaltisnotlongenoug";
    const PHRASE: &str = "ironmansucks";
    const DIFFICULTY: u32 = 1000;
    #[test]
    fn it_works() {
        let config = PoWConfig::new(SALT.into());
        let work_generated = config.work(PHRASE.into(), DIFFICULTY);

        let work = PoWBuilder::default()
            .result(work_generated.result)
            .nonce(work_generated.nonce)
            .build()
            .unwrap();

        let config = ConfigBuilder::default().salt(SALT.into()).build().unwrap();
        assert!(config.is_valid_proof(&work, &PHRASE.to_string()));
        assert!(config.is_sufficient_difficulty(&work, DIFFICULTY));
    }
}

#[cfg(not(tarpaulin_include))]
#[pymodule]
#[pyo3(name = "mcaptcha_pow_py")]
fn my_extension(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PoWConfig>()?;
    m.add_class::<Work>()?;
    Ok(())
}
