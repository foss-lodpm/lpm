use crate::{extraction::ExtractionTasks, validation::ValidationTasks};
use common::pkg::LodPkg;
use db::{pkg::LodPkgCoreDbOps, transaction_op, Transaction, DB_PATH};
use ehandle::RuntimeError;
use min_sqlite3_sys::prelude::*;
use std::{
    fs::{self, create_dir_all},
    io,
    path::Path,
};

pub trait InstallationTasks {
    fn copy_programs(&self) -> Result<(), io::Error>;
    fn start_installation(&mut self) -> Result<(), RuntimeError>;
    fn install_program(&self) -> Result<(), io::Error>;
}

impl<'a> InstallationTasks for LodPkg<'a> {
    fn start_installation(&mut self) -> Result<(), RuntimeError> {
        self.start_extraction()?;
        self.start_validations()?;

        let db = Database::open(Path::new(DB_PATH))?;
        self.insert_to_db(&db)?;

        match self.install_program() {
            Ok(_) => {}
            Err(err) => {
                transaction_op(&db, Transaction::Rollback)?;
                return Err(err.into());
            }
        };

        match self.cleanup() {
            Ok(_) => {}
            Err(err) => {
                transaction_op(&db, Transaction::Rollback)?;
                return Err(err.into());
            }
        };

        match transaction_op(&db, Transaction::Commit) {
            Ok(_) => {}
            Err(err) => {
                transaction_op(&db, Transaction::Rollback)?;
                return Err(err.into());
            }
        };

        db.close();

        Ok(())
    }

    fn install_program(&self) -> Result<(), io::Error> {
        self.copy_programs()
    }

    #[inline(always)]
    fn copy_programs(&self) -> Result<(), io::Error> {
        let source_path = super::EXTRACTION_OUTPUT_PATH.to_string()
            + "/"
            + self.path.unwrap().file_stem().unwrap().to_str().unwrap()
            + "/program/";

        for file in &self.meta_dir.as_ref().unwrap().files.0 {
            let destination_path = Path::new("/").join(&file.path);
            create_dir_all(destination_path.parent().unwrap()).unwrap();

            fs::copy(source_path.clone() + &file.path, destination_path)?;
        }

        Ok(())
    }
}
