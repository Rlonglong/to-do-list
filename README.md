# To Do List
## demo
<img src="https://github.com/user-attachments/assets/4132f4db-e11b-4ee9-957b-8c7c9a4af5aa" width="70%">


## usage
```
Available commands:
    Note: Please input date with the form "YYYY/MM/DD".

    - add [TASK] [DEADLINE]
        adds a new task
        Example: todo add "buy carrots" 2024/09/24
    - edit [OLD_TASK] [NEW_TASK] [NEW_DEADLINE]
        edits an existing task
        Example: todo edit "buy banana" "buy apple" 2005/01/30
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
```
