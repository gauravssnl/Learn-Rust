trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

trait AsMut<T: ?Sized> {
    fn as_mut(&self) -> &mut T;
}

use std::fs::File;
use std::path::Path;

// AsRef is typically used to make functions more flexible in the argument types they accept.
// open accepts anything it can borrow a &Path from.
// Such types include String and str, the operating system interface string types OsString and OsStr,
// and of course PathBuf and Path;
fn open<P: std::convert::AsRef<Path>>(path: P) -> std::io::Result<File> {
    File::open(path)
}

fn main() {
    let f = open("/home").unwrap();
    println!("f: {:?}", f);
}

// for any types T and U, if T: AsRef<U>, then &T: AsRef<U> as well
// since str: AsRef<Path>, then &str: AsRef<Path> as well. In a sense,
// this is a way to get a limited form of deref coercion
// in checking AsRef bounds on type variables.
impl<'a, X, Y> AsRef<Y> for &'a X
where
    X: AsRef<Y>,
    X: ?Sized,
    Y: ?Sized,
{
    fn as_ref(&self) -> &Y {
        (*self).as_ref()
    }
}
