use std::io;

fn main() -> io::Result<()> {
    // Only compile resources on Windows
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();

        // Set application icon
        res.set_icon("assets/icon.ico");

        // Set application metadata
        res.set("ProductName", "MechAura");
        res.set("FileDescription", "MechAura - Interactive Sound Simulator");
        res.set("CompanyName", "Mukul9897");
        res.set("LegalCopyright", "Copyright (C) 2026 Mukul Sharma");

        // Compile the resource file
        res.compile()?;
    }

    Ok(())
}
