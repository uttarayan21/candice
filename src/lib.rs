use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
pub trait Candice: AsRef<Path> {
    fn extension(&self) -> Option<&OsStr> {
        self.as_ref().extension()
    }
    fn file_name(&self) -> Option<&OsStr> {
        self.as_ref().file_name()
    }
    fn file_stem(&self) -> Option<&OsStr> {
        self.as_ref().file_stem()
    }
    // fn extension_mut(&mut self) -> Option<&mut OsStr>;
    // fn file_name_mut(&mut self) -> Option<&mut OsStr>;
    // fn file_stem_mut(&mut self) -> Option<&mut OsStr>;
    fn to_path_buf(&self) -> PathBuf {
        self.as_ref().to_path_buf()
    }

    fn with_extension(&self, extension: impl AsRef<OsStr>) -> PathBuf {
        self.as_ref().with_extension(extension)
    }

    fn with_file_name(&self, file_name: impl AsRef<OsStr>) -> PathBuf {
        self.as_ref().with_file_name(file_name)
    }

    fn with_file_stem(&self, file_stem: impl AsRef<OsStr>) -> PathBuf {
        let mut path = self.to_path_buf();
        let ext = self.extension();
        let fs = self.file_stem();
        if let Some(fs) = fs {
            // path.set_file_stem(file_stem);
            let mut fs = fs.to_owned();
            fs.extend(std::iter::once(file_stem.as_ref()));
            path.set_file_name(fs);
        }
        if let Some(ext) = ext {
            path.set_extension(ext);
        }
        path
    }

    fn push_extension(&self, extension: impl AsRef<OsStr>) -> PathBuf {
        self.with_extension_as(|ext| {
            // let mut ext = ext.map(|ext| ext.to_owned()).unwrap_or_default();
            // ext.extend(std::iter::once(extension.as_ref()));
            // ext.into()
            if let Some(ext) = ext {
                let mut ext = ext.to_owned();
                ext.extend([".".as_ref(), extension.as_ref()]);
                ext.into()
            } else {
                extension.as_ref().to_owned()
            }
        })
    }

    // fn push_file_stem(&self, file_stem: impl AsRef<OsStr>) -> PathBuf {
    //     match self.file_stem() {
    //         Some(fs) => {
    //             let mut path = self.to_path_buf();
    //             let mut final_fs = fs.to_owned();
    //             final_fs.extend(std::iter::once(file_stem.as_ref()));
    //             path.set_file_name(final_fs);
    //             path
    //         }
    //         None => {
    //             let mut path = self.to_path_buf();
    //             path.set_file_name(file_stem);
    //             path
    //         }
    //     }
    // }

    fn with_file_name_as(&self, f: impl FnOnce(Option<&OsStr>) -> OsString) -> PathBuf {
        let mut path = self.to_path_buf();
        path.set_file_name(f(self.file_name()));
        path
    }

    fn with_file_name_as_utf8(&self, f: impl FnOnce(Option<&str>) -> String) -> PathBuf {
        self.with_file_name_as(|file_name| {
            f(file_name.and_then(|file_name| file_name.to_str())).into()
        })
    }

    fn with_file_stem_as(&self, f: impl FnOnce(Option<&OsStr>) -> OsString) -> PathBuf {
        // let mut path = self.to_path_buf();
        self.with_file_stem(f(self.file_stem()))
    }

    fn with_file_stem_as_utf8(&self, f: impl FnOnce(Option<&str>) -> String) -> PathBuf {
        self.with_file_stem_as(|file_stem| {
            f(file_stem.and_then(|file_stem| file_stem.to_str())).into()
        })
    }
    fn with_extension_as(&self, f: impl FnOnce(Option<&OsStr>) -> OsString) -> PathBuf {
        self.with_extension(f(self.extension()))
    }
    fn with_extension_as_utf8(&self, f: impl FnOnce(Option<&str>) -> String) -> PathBuf {
        self.with_extension_as(|ext| f(ext.and_then(|ext| ext.to_str())).into())
    }
}

impl<P: AsRef<Path>> Candice for P {
    // fn extension(&self) -> Option<&OsStr> {
    //     self.as_ref().extension()
    // }
    // fn file_name(&self) -> Option<&OsStr> {
    //     self.as_ref().file_name()
    // }
    // fn file_stem(&self) -> Option<&OsStr> {
    //     self.as_ref().file_stem()
    // }
    // fn to_path_buf(&self) -> PathBuf {
    //     self.as_ref().to_path_buf()
    // }
}

#[test]
pub fn push_extension() {
    // Sanity check
    assert_eq!(
        Path::new("foo.html").with_extension("txt.rs"),
        Path::new("foo.txt.rs")
    );
    assert_eq!(
        Path::new("foo.txt").push_extension(".rs"),
        Path::new("foo.txt..rs")
    );
    assert_eq!(
        Path::new("foo.txt").push_extension("rs"),
        Path::new("foo.txt.rs")
    );
    assert_eq!(
        Path::new("foo").push_extension("rs").push_extension(".txt"),
        Path::new("foo.rs..txt")
    )
}
