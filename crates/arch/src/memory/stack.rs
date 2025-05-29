pub mod arguments;
pub mod auxiliary;
pub mod environment;

use human::info;

#[repr(C)]
#[derive(Debug)]
pub struct Stack<'a, 'b> {
    pub pointer: crate::Pointer,
    pub arguments: Option<arguments::Vector<'a>>,
    pub environment: Option<environment::Vector<'b>>,
    pub auxiliary: Option<auxiliary::Vector>,
    pub latter: crate::Pointer,
}

impl<'a, 'b> Stack<'a, 'b> {
    pub fn from_pointer(stack_pointer: crate::Pointer) -> Self {
        let (arguments, pointer) = arguments::Vector::from_pointer(stack_pointer);
        let (environment, pointer) = environment::Vector::from_pointer(pointer);
        // let (auxiliary, pointer) = auxiliary::Vector::from_pointer(pointer);

        let auxiliary = None;

        Self {
            pointer: stack_pointer,
            arguments,
            environment,
            auxiliary,
            latter: pointer,
        }
    }

    pub unsafe fn current() -> Self {
        Self::from_pointer(crate::Pointer::current())
    }

    pub fn print(&self) {
        info!("--- Stack Contents ---\n");
        
        info!("Arguments:\n");
        if let Some(args) = &self.arguments {
            args.print();
        } else {
            arguments::Vector::default().print();
        }
        
        info!("Environment Variables:\n");
        if let Some(env) = &self.environment {
            env.print();
        } else {
            environment::Vector::default().print();
        }
        
        // self.auxiliary
        //     .as_ref()
        //     .unwrap_or_else(|| &auxiliary::Vector::default())
        //     .print();
        
        info!("---------------------\n");
    }
}
