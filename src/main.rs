use tokio;
use crossbeam;
use uuid;

const TASK_QUEUE_CAPACITY: u32 = 10000;

const TASK_QPS_TOTAL: u32 = 132;

const SCHEDULER_COUNT: u32 = 20;

const MACHINE_CPU_COUNT: u32 = 1000;

const MACHINE_GPU_COUNT: u32 = 1000;

const HEARTBEAT_INTERVAL_S: f32 = 1.1;

const RETRIEVE_HEARTBEAT_S: f32 = 1.2;

struct Task {
    id: String,
    cpu_cost: f32,
    gpu_cost: f32
}

struct Machine {
    id: String,
    cpu_count: f32,
    gpu_count: f32
}

impl Machine {
    fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            cpu_count: 0.0,
            gpu_count: 0.0
        }
    }
    
    fn report(&self) {
        
    }
    
    fn run(&mut self, task: &Task) {
        self.cpu_count -= task.cpu_cost;
        self.gpu_count -= task.gpu_cost;
    }
}

struct TaskQueue {
    capacity: u32,
    sender: crossbeam::channel::Sender<Task>,
    receiver: crossbeam::channel::Receiver<Task>
}

impl TaskQueue {
    fn new(capacity: u32) -> Self {
        let (s, r) = crossbeam::channel::bounded(capacity as usize);
        Self {
            capacity,
            sender: s,
            receiver: r
        }
    }

    fn send(&self, task: Task) {
        self.sender.send(task);
    }

    fn receive(&self) -> Task {
        self.receiver.recv().unwrap()
    }
}

struct TaskProducer<'queue> {
    qps: u32,
    task_queue: &'queue TaskQueue
}

impl<'queue> TaskProducer<'queue> {
    fn new(qps: u32, task_queue: &'queue TaskQueue) -> Self {
        Self {
            qps,
            task_queue
        }
    }

    fn start(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100));
        }
    }
}

struct TaskConsumer<'queue> {
    task_queue: &'queue TaskQueue
}

impl<'queue> TaskConsumer<'queue> {
    fn new(task_queue: &'queue TaskQueue) -> Self {
        Self {
            task_queue
        }
    }
}

fn main() {
    let task_queue = TaskQueue::new(TASK_QUEUE_CAPACITY);
    let task_producer = TaskProducer::new(TASK_QPS_TOTAL, &task_queue);
    let task_consumer_list: [TaskConsumer; SCHEDULER_COUNT as usize] = {
        let mut tmp_list: [std::mem::MaybeUninit<TaskConsumer>; SCHEDULER_COUNT as usize] = unsafe {
            std::mem::MaybeUninit::uninit().assume_init()
        };
        for tmp in &mut tmp_list {
            tmp.write(TaskConsumer::new(&task_queue));
        }
        unsafe {
            std::mem::transmute(tmp_list)
        }
    };
}
