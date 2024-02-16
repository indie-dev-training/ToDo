use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}
};
use serde::{Deserialize, Serialize};

// for create todo
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTodo {
    text: String
}

// created todo
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    id: u32,
    text: String,
    status: String
}

impl Todo {
    fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            status: String::from("not Ready")
        }
    }
}

pub trait TodoRepository:  Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: NewTodo) -> Todo;
    fn find(&self, id: u32) -> Option<Box<Todo>>;
    fn find_all(&self) -> Vec<Todo>;
}

// todo collection
type Todos = HashMap<u32, Todo>;

// repositories
#[derive(Debug, Clone)]
pub struct InMemoryTodoRepository {
    pub store: Arc<RwLock<Todos>>
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::default()
        }
    }

    // write store(thread safe)
    fn write_store(&self) -> RwLockWriteGuard<Todos> {
        self.store.write().unwrap()
    }

    fn read_store(&self) -> RwLockReadGuard<Todos> {
        self.store.read().unwrap()
    }
}

impl TodoRepository for InMemoryTodoRepository {
    fn create(&self, payload: NewTodo) -> Todo {
        let mut store = self.write_store();
        let id = (store.len() + 1) as u32;
        let todo = Todo::new(id, payload.text.clone());
        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: u32) -> Option<Box<Todo>> {
        let store = self.read_store();
        let todo = store.get(&id)?;
        let todo = Box::new(todo.clone());
        Some(todo)
    }

    fn find_all(&self) -> Vec<Todo> {
        let store = self.read_store();
        Vec::from_iter(store.values().map(|todo| todo.clone()))
    }
}
