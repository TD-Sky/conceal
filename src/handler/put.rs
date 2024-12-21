use std::path::Path;

use crate::error::Result;

pub fn put(items: &[impl AsRef<Path>]) -> Result<()> {
    // Function `delete_all` wouldn't fail even if no file is specified.
    // But this doesn't meet our expectation.
    if items.is_empty() {
        return Err("Please specify the files to trash".into());
    }

    #[cfg(target_os = "macos")]
    {
        use trash::{
            macos::{DeleteMethod, TrashContextExtMacos},
            TrashContext,
        };
        let mut ctx = TrashContext::default();
        ctx.set_delete_method(DeleteMethod::NsFileManager);
        ctx.delete_all(items)?;
    }
    #[cfg(all(not(target_os = "macos"), not(target_os = "android")))]
    {
        trash::delete_all(items)?;
    }

    Ok(())
}
