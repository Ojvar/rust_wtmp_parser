use std::fs::File;
use std::io::{self, BufReader, Read};
use std::mem::size_of;

#[repr(C)]
#[derive(Debug)]
struct Utmpx {
    ut_type: i16,         // Type of login
    ut_pid: i32,          // PID of login process
    ut_line: [u8; 32],    // Device name of tty - "/dev/"
    ut_id: [u8; 4],       // Terminal name suffix, or inittab(5) ID
    ut_user: [u8; 32],    // Username
    ut_host: [u8; 256],   // Hostname for remote login
    ut_exit: ExitStatus,  // Exit status of a process marked as DEAD_PROCESS
    ut_session: i32,      // Session ID
    ut_tv: Timeval,       // Time entry was made
    ut_addr_v6: [i32; 4], // Internet address of remote host
    __unused: [u8; 20],   // Reserved for future use
}

#[repr(C)]
#[derive(Debug)]
struct ExitStatus {
    e_termination: i16, // Process termination status
    e_exit: i16,        // Process exit status
}

#[repr(C)]
#[derive(Debug)]
struct Timeval {
    tv_sec: i32,  // Seconds
    tv_usec: i32, // Microseconds
}

impl Utmpx {
    fn get_ut_user(&self) -> String {
        // Find the position of the first null byte (if any)
        let end = self
            .ut_user
            .iter()
            .position(|&x| x == 0)
            .unwrap_or(self.ut_user.len());
        // Take the valid part of the array
        let slice = &self.ut_user[..end];
        // Convert the slice to a string
        String::from_utf8_lossy(slice).into_owned()
    }
}

fn main() -> io::Result<()> {
    let file = File::open("/var/log/wtmp")?;
    let mut reader = BufReader::new(file);

    // Size of Utmpx struct
    let record_size = size_of::<Utmpx>();
    let mut buffer = vec![0; record_size];

    loop {
        match reader.read_exact(&mut buffer) {
            Ok(_) => {
                let utmpx: Utmpx = unsafe {
                    // Create a reference to the buffer as a Utmpx struct
                    let ptr = buffer.as_ptr() as *const Utmpx;
                    ptr.read()
                };
                // Print the entire Utmpx struct
                println!("{:?}", utmpx);
                // Print the ut_user field as a string
                println!("User: {}", utmpx.get_ut_user());
            }
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                // End of file reached
                break;
            }
            Err(e) => {
                // Handle other errors
                return Err(e);
            }
        }
    }

    Ok(())
}
