mod context;
mod switch;
mod task;

use core::cell::RefCell;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};

pub use context::TaskContext;
use lazy_static::lazy_static;

use self::task::TaskControlBlock;

pub struct TaskManager {
    num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

unsafe impl Sync for TaskManager {}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        // TODO
        
    };
}

impl TaskManager {
    fn run_first_task(&self) {}

    fn mark_current_suspended(&self) {}

    fn mark_current_exited(&self) {}

    fn find_next_task(&self) -> Option<usize> {
        Option::Some(1)
    }

    fn run_next_task(&self) {}
}

pub fn run_first_task(){
    TASK_MANAGER.run_first_task();
}