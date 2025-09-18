use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Task {
    // Order of fields matters for ordering.
    // We want to order after priority, and then task
    priority: i32, 
    task: i32,
}

impl Task {
    pub fn new(task: i32, priority: i32) -> Self {
        Task { priority, task }
    }
}

struct TaskManager {
    // Retrieve task with highest priority
    priorities: BinaryHeap<Task>,
    // Store current priority, which allows lazy remove of priorities
    tasks: HashMap<i32, (i32, i32)>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        Self {
            priorities: tasks.iter().map(|task| Task::new(task[1], task[2])).collect(),
            tasks: tasks.iter().map(|task| (task[1], (task[0], task[2]))).collect(),
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.priorities.push(Task::new(task_id, priority));
        self.tasks.insert(task_id, (user_id, priority));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let (user, priority) = self.tasks.get_mut(&task_id).unwrap();

        // Update priority for given task
        *priority = new_priority;

        // Add task as new element to the heap without removing the older variant
        self.priorities.push(Task::new(task_id, new_priority));
    }

    fn rmv(&mut self, task_id: i32) {
        // We only remove the task from the map, as we remove from the heap during execution
        self.tasks.remove(&task_id);
    }

    fn exec_top(&mut self) -> i32 {
        // Retrieve the most important task according to the ordering (priority, then id)
        while let Some(task) = self.priorities.pop() {
            // Retrieve the relevant user and priority
            let (user, priority) = match self.tasks.remove(&task.task) {
                Some(task) => task,
                // Task does not exists anymore, so we continue
                None => continue,
            };
            
            // Is the priority still the latest?
            if priority == task.priority {
                return user;
            }

            // We re-insert, as the task is still relevant.
            // The highest entry inside the heap just wasn't the latest
            self.tasks.insert(task.task, (user, priority));
        }

        -1
    }
}
