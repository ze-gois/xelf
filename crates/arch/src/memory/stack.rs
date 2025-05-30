pub mod arguments;
pub mod auxiliary;
pub mod environment;

use human::info;

#[repr(C)]
#[derive(Debug)]
pub struct Stack<'a, 'b> {
    pub pointer: crate::Pointer,
    pub arguments: arguments::Vector<'a>,
    pub environment: environment::Vector<'b>,
    pub auxiliary: auxiliary::Vector,
    pub latter: crate::Pointer,
}

impl<'a, 'b> Stack<'a, 'b> {
    pub fn from_pointer(stack_pointer: crate::Pointer) -> Self {
        let arguments = arguments::Vector::from_pointer(stack_pointer);

        let environment_pointer = unsafe { stack_pointer.0.add(2 + arguments.counter as usize) };
        let environment = environment::Vector::from_pointer(crate::Pointer(environment_pointer));

        let auxiliary_pointer =
            unsafe { environment_pointer.add(2 + environment.counter as usize) };
        let auxiliary = auxiliary::Vector::from_pointer(crate::Pointer(auxiliary_pointer));

        Self {
            pointer: stack_pointer,
            arguments,
            environment,
            auxiliary,
            latter: stack_pointer,
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
