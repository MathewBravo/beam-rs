// Create a VM emulation
pub struct VM {
    registers: [i32; 32],
}

impl VM {
    // Return a new instance of the VM emulation
    pub fn new() -> VM {
        VM { registers: [0; 32] }
    }
}
