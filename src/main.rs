use uuid::{uuid, Uuid};

//Creating struct named Task with Clone and Debug traits
#[derive( Clone,Debug)]
pub struct  Task{
    id:Uuid,
    description:String,
    completed:bool,
}

//A struct to store Task instances which is a kind of manuel static database
pub struct TodoList{
    data: Vec<Task>,
}

//applying functions for TodoList struct
impl TodoList{
    //instead of creating a new instance manuelly, for the better approach, just call that method to initalize a new instance.
    fn initialize() -> TodoList{
        TodoList { data: Vec::new() }
    }

    //a method to add a new Task into TodoList vector.
    //takes description as a parameter and returns the added Task instance.
    //Uuid is used to create the unique Id to make sure each item has a unique Id.
    fn add_task(&mut self,description:&str) -> Task{
        let data = Task { id: Uuid::new_v4(),description: description.to_string(), completed:false };
        self.data.push(data.clone());
        return data;
    }

    //THIS IS THE ALTERNATIVE FUNCTION OF add_task
    //for the best practice, wrapping up the response in case of failure.
    //because in real cases, data might not be added into database(in this case database is TodoList struct)
    //to make sure if the data is added to database, we call the item from the database.
    //if data is added, returns data; otherwise returns None.
    fn add_task_best_practice(&mut self,description:&str) -> Option<Task>{
        let id: Uuid = Uuid::new_v4();
        self.data.push(Task { id: id,description: description.to_string(), completed:false });

        let added_item = self.data.iter().find(|x| x.id==id);
        match added_item {
            Some(data) => Some(data.clone()),
            None => None
        }
    }

    //a method that completes the Task
    //takes Id as Uuid and if task exists, its completed field is set to true; otherwise returns None.
    fn complete_task(&mut self,_id:Uuid) -> Option<&Task>{
         if let Some(item) = self.data.iter_mut().find(|x| x.id==_id){
            item.completed=true;
            Some(item)
         }
         else{
            print!("Not found{}",_id);
            None
         }
    }

    //a method that print the details of all tasks in the ToDo list with their details
    fn list_tasks(&self) {
        for item in &self.data{
            println!("Id: {} || Description: {} || Completed: {}",item.id,item.description,item.completed);
        }
    }

    //THIS IS THE ALTERNATIVE FUNCTION OF list_tasks
    //in real cases, getting data from database is an action of getting list of items in json format
    //to simulate real usage scenarios, this method returns list of Tasks
    fn list_tasks_best_practice(&self) -> Vec<&Task>{
        return self.data.iter().collect::<Vec<_>>();
    }
}

fn main() {
    //calling to initializer instead of creating an instance manually
    let mut todo_list = TodoList::initialize();

    //adding four different tasks
    let task_one = todo_list.add_task("breakfast");
    let task_two = todo_list.add_task("walk");
    let task_three = todo_list.add_task("coffe");
    let task_four = todo_list.add_task("lunch");

    //alternative method!
    //adding another four different tasks using alternative method
    let task_five = todo_list.add_task_best_practice("work").unwrap();
    let task_six = todo_list.add_task_best_practice("tea").unwrap();
    let task_seven = todo_list.add_task_best_practice("dinner").unwrap();
    let task_eight = todo_list.add_task_best_practice("sleep").unwrap();

    //printing all tasks before calling complete_task function
    println!("TASKS. THAT ARE ADDED");
    todo_list.list_tasks();

    //just because each item has a unique Id, item with that Id does not exist and 
    //so,the method will not change completed status
    todo_list.complete_task(uuid!("9ea5232f-1e52-4540-a795-cdd8c2e8d4f3"));

    //calling complete_task method to complete task_one and task_two
    todo_list.complete_task(task_one.id);
    todo_list.complete_task(task_two.id);
    todo_list.complete_task(task_six.id);
    todo_list.complete_task(task_eight.id);

    println!("\n\n\n****************");
    println!("TASKS. AFTER COMPLETING FOUR TASKS");

    //listing tasks as printing the data in TodoList struct
    todo_list.list_tasks();

    //alternative method!
    //getting vector of tasks and printing as a json
    let data = todo_list.list_tasks_best_practice();
    // println!("{:#?}",data);
}