use cocoa::appkit::NSRunningApplication;
use cocoa::base::{id, nil};
use cocoa::foundation::NSAutoreleasePool;
use objc::{class, msg_send, sel, sel_impl};
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage: program <pid>".to_string());
    }

    let pid: i32 = args[1].parse().map_err(|e| format!("Invalid PID: {}", e))?;

    unsafe {
        // Create an autorelease pool
        let _pool = NSAutoreleasePool::new(nil);

        // Get the running application using class method
        let app: id = msg_send![class!(NSRunningApplication), 
            runningApplicationWithProcessIdentifier: pid];
        
        if app == nil {
            return Err(format!("No application found with PID: {}", pid));
        }

        // Activate the application
        let success: bool = msg_send![app, activateWithOptions: 1 << 0];
        
        if !success {
            return Err("Failed to activate application".to_string());
        }
    }

    Ok(())
}
